use crate::files;

use super::{doctree, Date, Definitions, Origin};

#[derive(Debug)]
pub struct Page {
    pub(crate) meta: PageMetadata,
    pub(crate) content: PageContents,
}

#[derive(Debug)]
pub struct PageMetadata {
    pub(crate) origin: Origin,
    pub(crate) url: files::FilePath,
    pub(crate) when: Option<Date>,
    pub(crate) status: PageStatus,
    pub(crate) tpl_name: String,
}

#[derive(Debug)]
pub enum PageStatus {
    Published,
    Draft,
}

#[derive(Debug)]
pub struct PageContents {
    pub(crate) content: Vec<doctree::Element>,
    pub(crate) footnotes: Definitions<doctree::FootnoteDefinition>,
    pub(crate) hrefs: Definitions<doctree::HrefDefinition>,
}
