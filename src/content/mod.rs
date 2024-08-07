pub mod corpus;
pub mod definitions;
pub mod doctree;
mod meta;
pub mod origin;
pub mod page;
mod pagebuilder;

pub use corpus::{Corpus, CorpusEntry, IncludedPath};
pub use definitions::Definitions;
pub use meta::Metadata;
pub use origin::Origin;
pub use page::{Page, PageContents};
pub use pagebuilder::PageBuilder;

#[derive(Debug)]
pub enum Error {}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
