use super::asset::IncludedAsset;
use super::asset::Writable;
use super::initializer;
use super::rendered::RenderedPage;
use super::rendered::RenderedSite;
use super::rendered::RenderingPage;
use super::rendered::RenderingSite;
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
    // called as part of initializing the application
    fn initialize<'call, 'init>(
        &'call mut self,
        site: &'call mut initializer::Initializer<'init, '_>,
    ) -> Result<()>
    where
        'init: 'call,
    {
        Ok(())
    }

    // called after a page has been proccessed from source but before it is placed into the corpus
    fn page_loading(&mut self, page: &mut content::PageBuilder) -> Result<()> {
        Ok(())
    }

    // all pages have been loaded into the corpus
    fn site_loaded(&mut self, corpus: &mut content::Corpus) -> Result<()> {
        Ok(())
    }

    // called once before any rendering happens, the context is placed under "globals" and
    // available to all page renderings
    fn global_render_context<'site>(
        &'site self,
        ctx: &'site mut jinja::RenderContext,
    ) -> Result<()> {
        Ok(())
    }

    // called before the specific page is rendered
    fn page_rendering<'render, 'site>(
        &self,
        page: &'site content::Page,
        rendering: &mut RenderingPage<'render, 'site>,
    ) -> Result<()>
    where
        'site: 'render,
    {
        Ok(())
    }

    // called after all pages have been rendered
    fn site_rendering<'site>(
        &self,
        corpus: &'site content::Corpus,
        site: &mut RenderingSite<'_, 'site, '_>,
    ) -> Result<()> {
        Ok(())
    }

    // called to drop/reset any state accumulated
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
