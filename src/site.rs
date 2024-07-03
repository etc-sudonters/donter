use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    fmt::Display,
    mem,
};

use crate::{content, files, jinja, render::render_page, Result};

pub struct Builder(Vec<Box<dyn Processor>>);

impl Builder {
    pub fn new() -> Builder {
        Self(vec![])
    }

    pub fn with_when<F, P>(&mut self, cond: bool, factory: F) -> &mut Self
    where
        P: 'static + Processor,
        F: FnOnce() -> P,
    {
        if cond {
            self.with(factory());
        }

        self
    }

    pub fn with<P: 'static + Processor>(&mut self, processor: P) -> &mut Self {
        self.0.push(Box::new(processor));
        self
    }

    pub fn create<'a>(&mut self) -> crate::Result<Donter<'a>> {
        let mut processors = vec![];
        mem::swap(&mut self.0, &mut processors);
        Donter::create(processors)
    }
}

pub struct Donter<'a> {
    renderer: minijinja::Environment<'a>,
    processors: Vec<Box<dyn Processor>>,
    loaders: Vec<Box<dyn Loader>>,
}

pub struct Initializer<'builder, 'env> {
    renderer: jinja::Builder<'builder, 'env>,
    loaders: &'builder mut Vec<Box<dyn Loader>>,
}

impl<'builder, 'env> Initializer<'builder, 'env> {
    pub fn add_loader(&mut self, loader: Box<dyn Loader>) {
        self.loaders.push(loader)
    }

    pub fn configure_renderer<'a, F>(&'a mut self, configure: F) -> crate::Result<()>
    where
        'builder: 'a,
        F: FnOnce(&'a mut jinja::Builder<'builder, 'env>) -> crate::Result<()>,
    {
        configure(&mut self.renderer)
    }
}

impl<'env> Donter<'env> {
    pub fn create(mut processors: Vec<Box<dyn Processor>>) -> crate::Result<Donter<'env>> {
        let mut renderer = minijinja::Environment::new();
        let mut loaders = Default::default();

        {
            let mut builder = Initializer {
                renderer: jinja::Builder::new(&mut renderer),
                loaders: &mut loaders,
            };

            for processor in processors.iter_mut() {
                processor.initialize(&mut builder)?;
            }
        }

        Ok(Donter {
            processors,
            loaders,
            renderer,
        })
    }

    pub fn load(
        &mut self,
        path: &files::DirPath,
        corpus: &mut content::Corpus,
    ) -> crate::Result<()> {
        for path in files::Walker::walk(path, files::RecursionBehavior::Dont) {
            for loader in self.loaders.iter_mut() {
                if loader.accept(&path)? {
                    let mut builder = content::PageBuilder::new(path.clone());

                    loader.load(Box::new(std::fs::File::open(&path)?), &mut builder)?;

                    for processor in self.processors.iter_mut() {
                        processor.page_load(&mut builder)?;
                    }
                    corpus.add_page(builder.build()?);
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn render_page(&mut self, page: content::Page) -> crate::Result<RenderedPage> {
        let tpl = self
            .renderer
            .get_template(&page.meta.tpl_name)
            .map_err(|e| Box::new(e))?;

        let mut ctx = jinja::RenderContext::new(minijinja::Value::default());
        for processor in self.processors.iter_mut() {
            processor.page_render(&page, &mut ctx)?;
        }

        ctx.merge(
            minijinja::context! { page => minijinja::Value::from_safe_string(render_page(&page.content))},
        );

        Ok(tpl
            .render(ctx)
            .map(|rendered| RenderedPage(rendered.into_bytes().into()))?)
    }

    pub fn process(&mut self, corpus: &mut content::Corpus) -> crate::Result<()> {
        for processor in self.processors.iter_mut() {
            processor.process(corpus)?;
        }

        Ok(())
    }

    pub fn render_corpus(&mut self, corpus: content::Corpus) -> crate::Result<RenderedSite> {
        let mut site = RenderedSite::new();

        for entry in corpus.entries() {
            match entry {
                content::CorpusEntry::Page(mut p) => {
                    let dest = p.meta.url.clone();
                    let page = self.render_page(p)?;
                    site.add_page(dest, page)?;
                }
                content::CorpusEntry::StaticAsset(asset) => {
                    site.add_static_asset(asset.clone().into(), asset)?;
                }
            }
        }

        Ok(site)
    }

    pub fn finalize(&mut self) -> crate::Result<()> {
        for processor in self.processors.iter_mut() {
            processor.finalize()?;
        }
        Ok(())
    }
}

pub struct RenderedPage(VecDeque<u8>);
impl RenderedPage {
    pub fn read(self) -> impl std::io::Read {
        self.0
    }

    pub fn size(&self) -> u64 {
        self.0.len() as u64
    }
}

pub struct IncludedAsset(files::Path);

impl IncludedAsset {
    pub fn read(self) -> crate::Result<impl std::io::Read> {
        Ok(std::fs::File::open(self.0)?)
    }

    pub fn path(self) -> files::Path {
        self.0
    }
}

pub enum Writable {
    Page(RenderedPage),
    Asset(IncludedAsset),
}

pub struct RenderedSite {
    writables: HashMap<files::Path, Writable>,
}

impl RenderedSite {
    pub fn new() -> RenderedSite {
        Self {
            writables: Default::default(),
        }
    }

    pub fn entries(self) -> impl std::iter::Iterator<Item = (files::Path, Writable)> {
        self.writables.into_iter()
    }

    pub fn add_page(&mut self, path: files::FilePath, content: RenderedPage) -> crate::Result<()> {
        match self.writables.entry(path.into()) {
            Entry::Vacant(v) => {
                v.insert(Writable::Page(content));
                Ok(())
            }
            Entry::Occupied(o) => Err(Box::new(SiteError::AlreadyOccupied(o.key().clone()))),
        }
    }

    pub fn add_static_asset(
        &mut self,
        path: files::Path,
        content: content::IncludedPath,
    ) -> crate::Result<()> {
        match self.writables.entry(path) {
            Entry::Vacant(v) => {
                v.insert(Writable::Asset(IncludedAsset(content.into())));
                Ok(())
            }
            Entry::Occupied(o) => Err(Box::new(SiteError::AlreadyOccupied(o.key().clone()))),
        }
    }
}

#[derive(Debug)]
pub enum SiteError {
    AlreadyOccupied(files::Path),
}

impl Display for SiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SiteError::*;
        write!(f, "SiteError::")?;
        match self {
            AlreadyOccupied(dst) => write!(f, "AlreadyOccupied({})", dst),
        }
    }
}

impl std::error::Error for SiteError {}

pub trait Loader {
    fn accept(&mut self, path: &files::FilePath) -> Result<bool>;
    fn load(
        &mut self,
        content: Box<dyn std::io::Read>,
        builder: &mut content::PageBuilder,
    ) -> crate::Result<()>;
}

pub trait Processor {
    fn initialize<'call, 'init>(
        &'call mut self,
        site: &'call mut Initializer<'init, '_>,
    ) -> Result<()>
    where
        'init: 'call,
    {
        Ok(())
    }

    fn page_load(&mut self, page: &mut content::PageBuilder) -> Result<()> {
        Ok(())
    }

    fn process(&mut self, corpus: &mut content::Corpus) -> Result<()> {
        Ok(())
    }

    fn page_render(&mut self, page: &content::Page, tx: &mut jinja::RenderContext) -> Result<()> {
        Ok(())
    }

    fn site_render(&mut self, site: &mut RenderedSite) -> Result<()> {
        Ok(())
    }

    fn finalize(&mut self) -> Result<()> {
        Ok(())
    }
}

pub trait Writer {
    fn write(&mut self, site: RenderedSite) -> crate::Result<()> {
        use Writable::*;

        for (dest, writable) in site.entries() {
            match writable {
                Page(page) => self.write_rendered_page(dest.as_file().unwrap(), page)?,
                Asset(asset) => self.write_static_asset(dest, asset)?,
            }
        }

        Ok(())
    }

    fn flush(self: Box<Self>) -> crate::Result<()> {
        Ok(())
    }

    fn write_rendered_page(
        &mut self,
        path: files::FilePath,
        page: RenderedPage,
    ) -> crate::Result<()>;

    fn write_static_asset(&mut self, path: files::Path, asset: IncludedAsset) -> crate::Result<()>;
}
