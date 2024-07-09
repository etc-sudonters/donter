use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    fmt::Display,
    marker::PhantomData,
    mem,
};

use crate::{
    content::{self, PageMetadata},
    files, jinja,
    render::{render_page, render_summary},
    Result,
};

pub struct Builder<'a>(Vec<Box<dyn Processor + 'a>>);

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn with_when<F, P>(mut self, cond: bool, factory: F) -> Self
    where
        P: Processor + 'a,
        F: FnOnce() -> P,
    {
        if cond {
            self.with(factory())
        } else {
            self
        }
    }

    pub fn with<P: Processor + 'a>(mut self, processor: P) -> Self {
        self.0.push(Box::new(processor));
        self
    }

    pub fn create(mut self) -> crate::Result<Donter<'a>> {
        Donter::create(self.0)
    }
}

pub struct Donter<'a> {
    renderer: minijinja::Environment<'a>,
    processors: Vec<Box<dyn Processor + 'a>>,
    loaders: Vec<Box<dyn Loader + 'a>>,
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
    pub fn create(mut processors: Vec<Box<dyn Processor + 'env>>) -> crate::Result<Donter<'env>> {
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

    pub fn render_page(&mut self, page: &content::Page) -> crate::Result<RenderedPage> {
        let tpl = self
            .renderer
            .get_template(&page.meta.tpl_name)
            .map_err(|e| Box::new(e))?;

        let mut ctx = jinja::RenderContext::new(minijinja::Value::default());
        for processor in self.processors.iter_mut() {
            processor.page_render(&page, &mut ctx)?;
        }

        ctx.merge(minijinja::context! {
          page => minijinja::context!{
            content => minijinja::Value::from_safe_string(render_page(&page.content)),
            title => page.meta.title,
            date => page.meta.when
          }
        });

        Ok(tpl.render(ctx).map(|rendered| {
            RenderedPage::new(
                VecDeque::from(rendered.into_bytes()),
                RenderedPageMetadata {
                    title: page.meta.title.clone(),
                    origin: page.meta.origin.clone(),
                    url: page.meta.url.clone(),
                    when: page.meta.when.clone(),
                    status: page.meta.status,
                    tpl_name: page.meta.tpl_name.clone(),
                    summary: page
                        .meta
                        .summary
                        .as_ref()
                        .map(|summ| {
                            render_summary(
                                summ.children(),
                                &page.content.footnotes,
                                &page.content.hrefs,
                            )
                        })
                        .unwrap_or(Default::default()),
                },
            )
        })?)
    }

    pub fn process(&mut self, corpus: &mut content::Corpus) -> crate::Result<()> {
        for processor in self.processors.iter_mut() {
            processor.process(corpus)?;
        }

        Ok(())
    }

    pub fn render(&mut self, corpus: &content::Corpus) -> crate::Result<RenderedSite> {
        let mut site = RenderedSite::new();

        for entry in corpus.entries() {
            match entry {
                content::CorpusEntry::Page(p) => {
                    let dest = p.meta.url.clone();
                    let page = self.render_page(&p)?;
                    site.add_page(dest, page)?;
                }
                content::CorpusEntry::StaticAsset(asset) => {
                    site.add_static_asset(asset.clone().into(), asset)?;
                }
            }
        }

        for processor in self.processors.iter_mut() {
            processor.site_render(&mut self.renderer, &corpus, &mut site)?;
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

pub struct RenderedPage {
    content: VecDeque<u8>,
    meta: RenderedPageMetadata,
}

#[derive(Debug, Clone)]
pub struct RenderedPageMetadata {
    pub(crate) title: String,
    pub(crate) origin: content::Origin,
    pub(crate) url: files::FilePath,
    pub(crate) when: Option<String>,
    pub(crate) status: content::PageStatus,
    pub(crate) tpl_name: String,
    pub(crate) summary: String,
}

impl RenderedPage {
    pub fn new<I: Into<VecDeque<u8>>>(content: I, meta: RenderedPageMetadata) -> Self {
        RenderedPage {
            content: content.into(),
            meta,
        }
    }

    pub fn read(self) -> impl std::io::Read {
        self.content
    }

    pub fn size(&self) -> u64 {
        self.content.len() as u64
    }

    pub fn metadata(&self) -> &RenderedPageMetadata {
        &self.meta
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
    origins: HashMap<content::Origin, files::Path>,
}

impl RenderedSite {
    pub fn new() -> RenderedSite {
        Self {
            writables: Default::default(),
            origins: Default::default(),
        }
    }

    pub fn entries(self) -> impl std::iter::Iterator<Item = (files::Path, Writable)> {
        self.writables.into_iter()
    }

    pub fn get_page_by_origin(&self, origin: &content::Origin) -> Option<&RenderedPage> {
        match self.origins.get(origin) {
            None => None,
            Some(dest) => match self.writables.get(dest).unwrap() {
                Writable::Page(p) => Some(p),
                _ => None,
            },
        }
    }

    pub fn add_page(&mut self, path: files::FilePath, content: RenderedPage) -> crate::Result<()> {
        match self.writables.entry(path.clone().into()) {
            Entry::Vacant(v) => {
                let origin = &content.meta.origin;
                self.origins.insert(origin.clone(), v.key().clone());

                v.insert(Writable::Page(content));
                Ok(())
            }
            Entry::Occupied(o) => Err(Box::new(SiteError::AlreadyOccupied(o.key().clone()))),
        }
    }

    pub fn add_static_asset(
        &mut self,
        path: files::Path,
        content: &content::IncludedPath,
    ) -> crate::Result<()> {
        match self.writables.entry(path) {
            Entry::Vacant(v) => {
                v.insert(Writable::Asset(IncludedAsset(content.clone().into())));
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

    fn page_render(&mut self, page: &content::Page, ctx: &mut jinja::RenderContext) -> Result<()> {
        Ok(())
    }

    fn site_render<'a>(
        &mut self,
        renderer: &mut minijinja::Environment<'_>,
        corpus: &content::Corpus,
        site: &mut RenderedSite,
    ) -> Result<()> {
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
            println!("writing {}", dest);
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
