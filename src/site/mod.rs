use std::fmt::Display;

use crate::files;

pub mod app;
pub mod asset;
pub mod builder;
pub mod exts;
pub mod initializer;
pub mod linker;
pub mod rendered;
pub use app::App;
pub use asset::IncludedAsset;
pub use asset::Writable;
pub use builder::Builder;
pub use exts::Loader;
pub use exts::Processor;
pub use exts::Writer;
pub use initializer::Initializer;
pub use linker::ArticleSlugSource;
pub use linker::ArticleSlugStyle;
pub use linker::Linker;
pub use linker::Options as LinkerOptions;
pub use rendered::PageTemplate;
pub use rendered::RenderedPage;
pub use rendered::RenderedPageMetadata;
pub use rendered::RenderedSite;
pub use rendered::RenderingPage;
pub use rendered::RenderingSite;

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
