use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    fmt::Display,
    marker::PhantomData,
    mem,
};

use crate::{
    content::{self, PageMetadata},
    files, jinja,
    render::render_page,
    Result,
};

pub mod app;
pub mod builder;
pub mod initializer;
pub mod rendered;
pub use app::App;
pub use builder::Builder;
pub use initializer::Initializer;
pub use rendered::IncludedAsset;
pub use rendered::RenderedPage;
pub use rendered::RenderedPageMetadata;
pub use rendered::RenderedSite;
pub use rendered::Writable;

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
