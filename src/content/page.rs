use super::{doctree, Date, Definitions, Origin};
use url::Url;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Page {
    pub(crate) meta: PageMetadata,
    pub(crate) content: PageContents,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PageMetadata {
    pub(crate) origin: Origin,
    pub(crate) url: Url,
    pub(crate) when: Option<Date>,
    pub(crate) status: PageStatus,
    pub(crate) tpl_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum PageStatus {
    Published,
    Draft,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PageContents {
    pub(crate) content: Vec<doctree::Element>,
    pub(crate) footnotes: Definitions<doctree::FootnoteDefinition>,
    pub(crate) hrefs: Definitions<doctree::HrefDefinition>,
}
