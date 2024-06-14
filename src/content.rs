use crate::{content, doctree, files, Res};

use std::path::PathBuf;

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
}

impl PageBuilder {
    pub fn new() -> PageBuilder {
        PageBuilder {
            content: None,
            filepath: None,
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

    // would be nicer if this was mut self instead :|
    // could probably do some type level shit to do that but :shrug:
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
                },
            })
        }
    }

    pub fn clear(&mut self) {
        self.content.take();
        self.filepath.take();
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
