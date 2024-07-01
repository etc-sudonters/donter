use url::Url;

use crate::{doctree, files, Result};
use std::collections::HashMap;
use std::fmt::Display;
use std::path::Path;

pub struct PageToken;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Tag(String);

pub struct PageBuilder {
    pub(crate) contents: Vec<doctree::Element>,
    pub(crate) filepath: files::FilePath,
    pub(crate) url_path: Option<Url>,
    pub(crate) notes: Definitions<doctree::FootnoteDefinition>,
    pub(crate) page_hrefs: Definitions<doctree::HrefDefinition>,
    pub(crate) when: Option<Date>,
    pub(crate) page_status: PageStatus,
    pub(crate) tpl_name: String,
}

impl PageBuilder {
    pub fn new<F: Into<files::FilePath>>(f: F) -> PageBuilder {
        PageBuilder {
            contents: Default::default(),
            filepath: f.into(),
            url_path: Default::default(),
            notes: Default::default(),
            page_hrefs: Default::default(),
            when: Default::default(),
            page_status: PageStatus::Published,
            tpl_name: "page.html".to_owned(),
        }
    }

    #[allow(dead_code)]
    pub fn written(&mut self, d: Date) -> &mut Self {
        self.when = Some(d);
        self
    }

    #[allow(dead_code)]
    pub fn status(&mut self, s: PageStatus) -> &mut Self {
        self.page_status = s;
        self
    }

    pub fn content(&mut self, content: doctree::Element) -> &mut Self {
        self.contents.push(content);
        self
    }

    #[allow(dead_code)]
    pub fn url(&mut self, d: Url) -> &mut Self {
        self.url_path = Some(d);
        self
    }

    pub fn url_or(&mut self, d: Url) -> &mut Self {
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

    pub fn build(mut self) -> Result<Page> {
        Ok(Page {
            meta: PageMetadata {
                origin: Origin(self.filepath),
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Page {
    pub(crate) meta: PageMetadata,
    pub(crate) content: PageContents,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PageMetadata {
    pub(crate) origin: Origin,
    pub(crate) url: Url,
    when: Option<Date>,
    status: PageStatus,
    pub(crate) tpl_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Date(String);

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Definitions<T: doctree::Definition> {
    labels: Vec<String>,
    // label idx -> definition
    definitions: HashMap<usize, T>,
}

impl<R: doctree::Reference, T: doctree::Definition> doctree::DefinitionLookup<R, T>
    for Definitions<T>
{
    fn lookup(&self, id: R) -> Option<&T> {
        let id = self.labels.iter().enumerate().find(|&x| x.1 == id.label());
        id.map(|x| self.definitions.get(&x.0)).flatten()
    }
}

impl<T: doctree::Definition> Default for Definitions<T> {
    fn default() -> Self {
        Definitions {
            labels: Vec::new(),
            definitions: HashMap::new(),
        }
    }
}

impl<T: doctree::Definition> Definitions<T> {
    pub fn add_label(&mut self, key: &String) {
        self.get_or_insert(key);
    }

    pub fn define(&mut self, key: &String, value: T) {
        let id = self.get_or_insert(key);
        let entry = self.definitions.entry(id);
        entry.or_insert(value);
    }

    fn get_or_insert(&mut self, key: &String) -> usize {
        match self.labels.iter().position(|r| key == r) {
            Some(id) => id,
            None => {
                let id = self.labels.len();
                self.labels.push(key.to_owned());
                id
            }
        }
    }
}

pub fn default() -> Corpus {
    Default::default()
}

#[derive(Debug)]
pub struct Corpus {
    pages: Vec<Page>,
    included: Vec<IncludedPath>,
}

pub enum CorpusEntry {
    Page(Page),
    StaticAsset(IncludedPath),
}

#[derive(Debug)]
pub struct IncludedPath(files::Path);

impl Into<files::Path> for IncludedPath {
    fn into(self) -> files::Path {
        self.0
    }
}

impl AsRef<Path> for IncludedPath {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl Corpus {
    pub fn add_page(&mut self, p: Page) {
        self.pages.push(p);
    }

    #[allow(dead_code, unused_variables)]
    pub fn include_asset(&mut self, p: files::Path) {
        todo!();
    }

    pub fn entries(self) -> CorpusEntries {
        let (pages, included) = (self.pages, self.included);

        let entries = pages.into_iter().map(|p| CorpusEntry::Page(p));
        let includes = included.into_iter().map(|p| CorpusEntry::StaticAsset(p));

        CorpusEntries(entries.chain(includes).collect())
    }
}

pub struct CorpusEntries(Vec<CorpusEntry>);

impl Iterator for CorpusEntries {
    type Item = CorpusEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl Default for Corpus {
    fn default() -> Self {
        Corpus {
            pages: Vec::new(),
            included: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    PageLoad(files::FilePath, Box<dyn std::error::Error>),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Origin(files::FilePath);

impl std::ops::Deref for Origin {
    type Target = files::FilePath;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Origin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl serde::Serialize for Origin {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_path().to_str().unwrap())
    }
}

impl<'de> serde::Deserialize<'de> for Origin {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
