use super::IncludedAsset;
use super::RenderedSite;
use std::collections::VecDeque;

use super::RenderedPageMetadata;

use super::RenderedPage;

use super::initializer::Initializer;

use super::Processor;

use super::Loader;

use super::initializer;

use crate::content;
use crate::files;
use crate::jinja;
use crate::render::render_page;
use crate::render::render_summary;

pub struct AppConfig {
    asset_base: files::DirPath,
}

pub struct App<'a> {
    loaders: Vec<Box<dyn Loader + 'a>>,
    processors: Vec<Box<dyn Processor + 'a>>,
    renderer: minijinja::Environment<'a>,
    config: AppConfig,
}

impl<'env> App<'env> {
    pub fn create(mut processors: Vec<Box<dyn Processor + 'env>>) -> crate::Result<App<'env>> {
        let mut renderer = minijinja::Environment::new();
        let mut loaders = Default::default();

        {
            let mut builder = initializer::Initializer {
                renderer: jinja::Builder::new(&mut renderer),
                loaders: &mut loaders,
            };

            for processor in processors.iter_mut() {
                processor.initialize(&mut builder)?;
            }
        }

        Ok(App {
            processors,
            loaders,
            renderer,
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
                        processor.page_load(&mut builder)?;
                    }
                    corpus.add_page(builder)?;
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn render_page<'a, 'b>(
        &mut self,
        page: &'a content::Page,
    ) -> crate::Result<RenderedPage<'b>>
    where
        'a: 'b,
    {
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
                    origin: Some(page.id.clone()),
                    title: &page.meta.title,
                    url: &page.meta.url,
                    when: page.meta.when.as_deref(),
                    summary: page.meta.summary.as_ref().map(|summ| {
                        render_summary(
                            summ.children(),
                            &page.content.footnotes,
                            &page.content.hrefs,
                        )
                    }),
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

    pub fn render<'a>(&'a mut self, corpus: &'a content::Corpus) -> crate::Result<RenderedSite<'a>>
    where
        'env: 'a,
    {
        let mut site = RenderedSite::new();

        for entry in corpus.entries() {
            match entry {
                content::CorpusEntry::Page(p) => {
                    let dest = p.meta.url.clone();
                    let page = self.render_page(&p)?;
                    //page.metadata().origin = Some(p.id);
                    site.add_page(page);
                }
                content::CorpusEntry::StaticAsset(asset) => {
                    site.add_asset(IncludedAsset::create(
                        asset,
                        self.config.asset_base.join(asset),
                    ));
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
