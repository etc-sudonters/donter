use super::asset::IncludedAsset;
use super::asset::Writable;
use super::initializer;
use super::rendered::RenderedPage;
use super::rendered::RenderedSite;
use crate::content;
use crate::files;
use crate::jinja;

use initializer::Initializer;

use crate::Result;

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
        site: &'call mut initializer::Initializer<'init, '_>,
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

    fn site_render<'site, 'env>(
        &'site mut self,
        renderer: &mut minijinja::Environment<'env>,
        corpus: &content::Corpus,
        site: &mut RenderedSite<'site>,
    ) -> Result<()> {
        Ok(())
    }

    fn finalize(&mut self) -> Result<()> {
        Ok(())
    }
}

pub trait Writer {
    fn write(&mut self, site: RenderedSite<'_>) -> crate::Result<()> {
        use Writable::*;

        for (_, writable) in site.entries() {
            match writable {
                Page(page) => self.write_rendered_page(page)?,
                Asset(asset) => self.write_static_asset(asset)?,
            }
        }

        Ok(())
    }

    fn flush(self: Box<Self>) -> crate::Result<()> {
        Ok(())
    }

    fn write_rendered_page(&mut self, page: RenderedPage) -> crate::Result<()>;

    fn write_static_asset(&mut self, asset: IncludedAsset) -> crate::Result<()>;
}
