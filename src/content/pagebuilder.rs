use super::definitions::Definitions;
use super::page::{Page, PageContents, PageMetadata, PageStatus};
use super::Date;
use super::{doctree, meta};
use crate::files;

pub struct PageBuilder {
    pub(crate) title: String,
    pub(crate) contents: Vec<doctree::Element>,
    pub(crate) filepath: files::FilePath,
    pub(crate) url_path: Option<files::FilePath>,
    pub(crate) notes: Definitions<doctree::FootnoteDefinition>,
    pub(crate) page_hrefs: Definitions<doctree::HrefDefinition>,
    pub(crate) when: Option<Date>,
    pub(crate) page_status: PageStatus,
    pub(crate) tpl_name: String,
    pub(crate) meta: Option<meta::Metadata>,
}

impl PageBuilder {
    pub fn new<F: Into<files::FilePath>>(f: F) -> PageBuilder {
        PageBuilder {
            title: Default::default(),
            contents: Default::default(),
            filepath: f.into(),
            url_path: Default::default(),
            notes: Default::default(),
            page_hrefs: Default::default(),
            when: Default::default(),
            page_status: PageStatus::Published,
            tpl_name: "page.html".to_owned(),
            meta: Default::default(),
        }
    }

    pub fn written(&mut self, d: Date) -> &mut Self {
        self.when = Some(d);
        self
    }

    pub fn status(&mut self, s: PageStatus) -> &mut Self {
        self.page_status = s;
        self
    }

    pub fn content(&mut self, content: doctree::Element) -> &mut Self {
        self.contents.push(content);
        self
    }

    pub fn url(&mut self, d: files::FilePath) -> &mut Self {
        self.url_path = Some(d);
        self
    }

    pub fn url_or(&mut self, d: files::FilePath) -> &mut Self {
        if self.url_path.is_none() {
            self.url_path = Some(d);
        }
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
            meta: PageMetadata {
                origin: super::Origin::new(self.filepath),
                url: self.url_path.unwrap(),
                when: self.when.take(),
                status: self.page_status,
                tpl_name: self.tpl_name,
            },
            content: PageContents {
                content: self.contents,
                footnotes: self.notes,
                hrefs: self.page_hrefs,
            },
        })
    }
}
