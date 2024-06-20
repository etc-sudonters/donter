use crate::{content, doctree, files, Res};
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
    content: Option<doctree::Document>,
    filepath: Option<files::FilePath>,
    notes: Option<Definitions<doctree::FootnoteDefinition>>,
    links_and_images: Option<Definitions<doctree::HrefDefinition>>,
}

impl PageBuilder {
    pub fn new() -> PageBuilder {
        PageBuilder {
            content: None,
            filepath: None,
            notes: None,
            links_and_images: None,
        }
    }

    pub fn contents(&mut self, content: doctree::Document) -> &mut Self {
        self.content = Some(content);
        self
    }

    pub fn path(&mut self, f: files::FilePath) -> &mut Self {
        self.filepath = Some(f);
        self
    }

    pub fn footnotes(&mut self) -> &mut Definitions<doctree::FootnoteDefinition> {
        self.notes.get_or_insert_with(Default::default)
    }

    pub fn links(&mut self) -> &mut Definitions<doctree::HrefDefinition> {
        self.links_and_images.get_or_insert_with(Default::default)
    }

    pub fn build(&mut self) -> Option<Page> {
        if self.content.is_none() || self.filepath.is_none() {
            None
        } else {
            Some(Page {
                meta: PageMetadata {
                    path: self.filepath.take().unwrap(),
                },
                content: PageContents {
                    content: self.content.take().unwrap(),
                    footnotes: self.notes.take(),
                    links_and_images: self.links_and_images.take(),
                },
            })
        }
    }

    pub fn clear(&mut self) {
        self.content.take();
        self.filepath.take();
        self.notes.take();
        self.links_and_images.take();
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
}

#[derive(Debug)]
pub struct PageContents {
    content: doctree::Document,
    footnotes: Option<Definitions<doctree::FootnoteDefinition>>,
    links_and_images: Option<Definitions<doctree::HrefDefinition>>,
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
    fn accept(&mut self, path: &files::FilePath) -> Res<bool>;
    fn load(
        &mut self,
        path: &files::FilePath,
        corpus: &mut crate::content::Corpus,
        builder: &mut content::PageBuilder,
    ) -> crate::Res<()>;
}

#[derive(Debug)]
pub enum Error {}
impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
