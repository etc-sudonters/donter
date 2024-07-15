use std::collections::HashMap;

use super::definitions::Definitions;
use super::doctree;
use super::page::{Page, PageContents, PageMetadata};
use super::{CorpusEntry, Metadata};
use crate::{files, ids};

pub struct PageBuilder {
    pub(crate) id: ids::Id<CorpusEntry>,
    pub(crate) title: String,
    pub(crate) contents: Vec<doctree::Element>,
    pub(crate) filepath: files::FilePath,
    pub(crate) notes: Definitions<doctree::FootnoteDefinition>,
    pub(crate) page_hrefs: Definitions<doctree::HrefDefinition>,
    pub(crate) when: Option<String>,
    pub(crate) tpl_name: String,
    pub(crate) meta: HashMap<String, Metadata>,
    pub(crate) summary: Option<doctree::Group>,
}

impl PageBuilder {
    pub fn new<F: Into<files::FilePath>>(id: ids::Id<CorpusEntry>, f: F) -> PageBuilder {
        PageBuilder {
            id,
            filepath: f.into(),
            title: Default::default(),
            contents: Default::default(),
            notes: Default::default(),
            page_hrefs: Default::default(),
            when: Default::default(),
            tpl_name: "page.html".to_owned(),
            meta: Default::default(),
            summary: Default::default(),
        }
    }

    pub fn with_title<S: Into<String>>(&mut self, title: S) -> &mut Self {
        self.title = title.into();
        self
    }

    pub fn written(&mut self, d: String) -> &mut Self {
        self.when = Some(d);
        self
    }

    pub fn content(&mut self, content: doctree::Element) -> &mut Self {
        self.contents.push(content);
        self
    }

    pub fn footnotes<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Definitions<doctree::FootnoteDefinition>),
    {
        f(&mut self.notes);
        self
    }

    pub fn hrefs<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Definitions<doctree::HrefDefinition>),
    {
        f(&mut self.page_hrefs);
        self
    }

    pub fn build(mut self) -> crate::Result<Page> {
        Ok(Page {
            id: self.id,
            meta: PageMetadata {
                title: self.title,
                origin: super::Origin(self.filepath),
                when: self.when.take(),
                tpl_name: self.tpl_name,
                meta: self.meta,
                summary: self.summary,
            },
            content: PageContents {
                content: self.contents,
                footnotes: self.notes,
                hrefs: self.page_hrefs,
            },
        })
    }
}
