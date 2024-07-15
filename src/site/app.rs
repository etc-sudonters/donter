use super::rendered::RenderingSite;
use super::IncludedAsset;
use super::RenderedSite;
use super::RenderingPage;
use std::borrow::BorrowMut;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use super::RenderedPageMetadata;

use super::RenderedPage;

use super::initializer::Initializer;

use super::Processor;

use super::Loader;

use super::initializer;

use crate::content;
use crate::files;
use crate::files::Path;
use crate::jinja;
use crate::render::render_page;
use crate::render::render_summary;

pub struct AppConfig {
    asset_base: files::DirPath,
}

pub struct App<'a> {
    loaders: Vec<Box<dyn Loader + 'a>>,
    processors: Vec<Box<dyn Processor + 'a>>,
    renderer: Rc<RefCell<minijinja::Environment<'a>>>,
    config: AppConfig,
    linker: super::Linker<'a>,
}

impl<'env> App<'env> {
    pub fn create(mut site: super::Builder<'env>) -> crate::Result<App<'env>> {
        let mut renderer = minijinja::Environment::new();
        let mut loaders = Default::default();

        {
            let mut builder = initializer::Initializer {
                renderer: jinja::Builder::new(&mut renderer),
                loaders: &mut loaders,
            };

            for processor in site.processors.iter_mut() {
                processor.initialize(&mut builder)?;
            }
        }

        Ok(App {
            linker: super::Linker::new(site.linker_opts),
            processors: site.processors,
            loaders,
            renderer: Rc::new(RefCell::new(renderer)),
            config: AppConfig {
                asset_base: unsafe { files::DirPath::new("assets") },
            },
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
                    let mut builder = corpus.make_page(path.clone());
                    loader.load(Box::new(std::fs::File::open(&path)?), &mut builder)?;
                    for processor in self.processors.iter_mut() {
                        processor.page_loading(&mut builder)?;
                    }
                    corpus.add_page(builder)?;
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn render_page<'rendering, 'site>(
        &'rendering self,
        page: &'site content::Page,
        site: &mut RenderingSite<'rendering, 'site, 'env>,
    ) -> crate::Result<()>
    where
        'env: 'site,
        'site: 'rendering,
    {
        let mut rendering = site.page(&page.meta.tpl_name);

        rendering.values().merge(minijinja::context! {
          page => minijinja::context!{
            content => minijinja::Value::from_safe_string(render_page(&page.content)),
            title => page.meta.title,
            date => page.meta.when
          }
        });

        for processor in self.processors.iter() {
            processor.page_rendering(page, &mut rendering)?;
        }

        let meta = RenderedPageMetadata {
            origin: Some(page.id.clone()),
            title: Cow::Borrowed(&page.meta.title),
            url: Cow::Owned(self.linker.slug(&page.meta.origin)),
            when: page.meta.when.as_deref().map(Cow::Borrowed),
            summary: page.meta.summary.as_ref().map(|summ| {
                render_summary(
                    summ.children(),
                    &page.content.footnotes,
                    &page.content.hrefs,
                )
            }),
        };

        site.render_page(meta, rendering)?;

        Ok(())
    }

    pub fn process(&mut self, corpus: &mut content::Corpus) -> crate::Result<()> {
        for processor in self.processors.iter_mut() {
            processor.site_loaded(corpus)?;
        }

        Ok(())
    }

    pub fn render<'render, 'site>(
        &'render self,
        corpus: &'site content::Corpus,
    ) -> crate::Result<RenderedSite<'site>>
    where
        'env: 'site,
        'site: 'render,
    {
        let mut globals = jinja::RenderContext::empty();
        for processor in self.processors.iter() {
            processor.global_render_context(&mut globals)?;
        }
        let mut site = RenderingSite::new(jinja::Renderer::new(&self.renderer, globals));

        for entry in corpus.entries() {
            match entry {
                content::CorpusEntry::Page(p) => {
                    self.render_page(&p, &mut site)?;
                }
                content::CorpusEntry::StaticAsset(asset) => {
                    site.add_asset(IncludedAsset::create(
                        asset,
                        self.config.asset_base.join(asset),
                    ));
                }
            }
        }

        for processor in self.processors.iter() {
            processor.site_rendering(&corpus, &mut site)?;
        }

        Ok(site.render())
    }

    pub fn finalize(&mut self) -> crate::Result<()> {
        for processor in self.processors.iter_mut() {
            processor.finalize()?;
        }
        Ok(())
    }
}
