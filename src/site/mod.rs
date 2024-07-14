use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    fmt::Display,
    marker::PhantomData,
    mem,
};

use crate::{
    content::{self, PageMetadata},
    files,
    render::render_page,
};

pub mod app;
pub mod asset;
pub mod builder;
pub mod exts;
pub mod initializer;
pub mod rendered;
pub use app::App;
pub use asset::IncludedAsset;
pub use asset::Writable;
pub use builder::Builder;
pub use exts::Loader;
pub use exts::Processor;
pub use exts::Writer;
pub use initializer::Initializer;
pub use rendered::PageTemplate;
pub use rendered::RenderedPage;
pub use rendered::RenderedPageMetadata;
pub use rendered::RenderedSite;

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
