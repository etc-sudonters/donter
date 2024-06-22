use crate::{content, doctree, files, Result};
use std::{collections::HashMap, fmt::Display, path::PathBuf};

pub fn default() -> Corpus {
    Default::default()
}

#[derive(Debug)]
pub struct Corpus {
    pages: Vec<Page>,
    included: Vec<IncludedPath>,
}

impl Corpus {
    pub fn push_page(&mut self, p: Page) {
        self.pages.push(p);
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

pub struct PageBuilder {
    content: Option<doctree::Element>,
    filepath: Option<files::FilePath>,
    notes: Option<Definitions<doctree::FootnoteDefinition>>,
    hrefs: Option<Definitions<doctree::HrefDefinition>>,
}

impl PageBuilder {
    pub fn new() -> PageBuilder {
        PageBuilder {
            content: None,
            filepath: None,
            notes: None,
            hrefs: None,
        }
    }

    pub fn contents(mut self, content: doctree::Element) -> Self {
        self.content = Some(content);
        self
    }

    pub fn path(mut self, f: &files::FilePath) -> Self {
        self.filepath = Some(f.to_owned());
        self
    }

    pub fn footnotes(&mut self) -> &mut Definitions<doctree::FootnoteDefinition> {
        self.notes.get_or_insert_with(Default::default)
    }

    pub fn hrefs(&mut self) -> &mut Definitions<doctree::HrefDefinition> {
        self.hrefs.get_or_insert_with(Default::default)
    }

    // returns the built page, or the builder and a description of the error
    pub fn try_build(mut self) -> crate::Result<Page> {
        if self.content.is_none() || self.filepath.is_none() {
            Err(Box::new(Error::IncompleteLoader))
        } else {
            Ok(Page {
                meta: PageMetadata {
                    path: self.filepath.take().unwrap(),
                    tags: vec![],
                    when: Date("".to_owned()),
                    status: PageStatus::Draft,
                },
                content: PageContents {
                    content: self.content.take().unwrap(),
                    footnotes: self.notes.take(),
                    hrefs: self.hrefs.take(),
                },
            })
        }
    }
}

#[derive(Debug)]
pub struct Page {
    meta: PageMetadata,
    content: PageContents,
}

#[derive(Debug)]
pub struct PageMetadata {
    path: files::FilePath,
    tags: Vec<Tag>,
    when: Date,
    status: PageStatus,
}

#[derive(Debug)]
pub struct Tag(String);

#[derive(Debug)]
pub struct Date(String);

#[derive(Debug)]
pub enum PageStatus {
    Published,
    Draft,
}

#[derive(Debug)]
pub struct PageContents {
    content: doctree::Element,
    footnotes: Option<Definitions<doctree::FootnoteDefinition>>,
    hrefs: Option<Definitions<doctree::HrefDefinition>>,
}

#[derive(Debug)]
pub struct Definitions<T: doctree::Definition> {
    labels: Vec<String>,
    // label idx -> definition
    definitions: HashMap<usize, T>,
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
    pub fn gather(&mut self, definitions: Vec<T>) {
        for definition in definitions.into_iter() {
            self.define(&definition.label(), definition);
        }
    }

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

#[derive(Debug)]
pub struct IncludedPath(PathBuf);

pub trait Loader {
    fn accept(&mut self, path: &files::FilePath) -> Result<bool>;
    fn load(
        &mut self,
        content: Box<dyn std::io::Read>,
        corpus: &mut crate::content::Corpus,
        builder: content::PageBuilder,
    ) -> crate::Result<()>;
}

#[derive(Debug)]
pub enum Error {
    PageLoad(files::FilePath, Box<dyn std::error::Error>),
    IncompleteLoader,
}
impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
