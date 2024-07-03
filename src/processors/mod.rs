#![allow(unused_imports)]

mod linker;
mod staticfiles;
mod tag;

pub use linker::{ArticleSlugStyle, Linker, Options as LinkerOptions};
pub use staticfiles::StaticFiles;
pub use tag::Tags;
