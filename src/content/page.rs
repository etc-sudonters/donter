use std::collections::HashMap;

use crate::files;

use super::{doctree, Definitions, Metadata, Origin};

#[derive(Debug)]
pub struct Page {
    pub(crate) meta: PageMetadata,
    pub(crate) content: PageContents,
}

#[derive(Debug)]
pub struct PageMetadata {
    pub(crate) title: String,
    pub(crate) origin: Origin,
    pub(crate) url: files::FilePath,
    pub(crate) when: Option<String>,
    pub(crate) status: PageStatus,
    pub(crate) tpl_name: String,
    pub(crate) meta: HashMap<String, Metadata>,
    pub(crate) summary: Option<doctree::Group>,
}

#[derive(Debug, Copy, Clone)]
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
