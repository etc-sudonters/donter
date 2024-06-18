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
    notes: Option<References>,
    links_and_images: Option<References>,
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

    pub fn footnotes(&mut self) -> &mut References {
        self.notes.get_or_insert_with(Default::default)
    }

    pub fn links(&mut self) -> &mut References {
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
    footnotes: Option<References>,
    links_and_images: Option<References>,
}

#[derive(Debug)]
pub struct References {
    references: Vec<String>,
    definitions: HashMap<usize, doctree::Document>,
}

impl Default for References {
    fn default() -> Self {
        References {
            references: Vec::new(),
            definitions: HashMap::new(),
        }
    }
}

impl References {
    pub fn add_reference(&mut self, key: &String) {
        self.get_or_insert(key);
    }

    fn get_or_insert(&mut self, key: &String) -> usize {
        match self.references.iter().position(|r| key == r) {
            Some(id) => id,
            None => {
                let id = self.references.len();
                self.references.push(key.to_owned());
                id
            }
        }
    }

    pub fn define(&mut self, key: &String, value: doctree::Document) {
        let id = self.get_or_insert(key);
        let entry = self.definitions.entry(id);
        entry.or_insert(value);
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
