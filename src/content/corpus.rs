use super::page::Page;
use crate::files;
use std::path::Path;

#[derive(Debug)]
pub struct Corpus {
    pages: Vec<Page>,
    included: Vec<IncludedPath>,
}

pub enum CorpusEntry {
    Page(Page),
    StaticAsset(IncludedPath),
}

pub enum CorpusEntryRef<'a> {
    Page(&'a Page),
    StaticAsset(&'a IncludedPath),
}

#[derive(Debug, Clone)]
pub struct IncludedPath(files::Path);

impl Corpus {
    pub fn add_page(&mut self, p: Page) {
        self.pages.push(p);
    }

    pub fn include_asset(&mut self, p: files::Path) {
        self.included.push(IncludedPath(p))
    }

    pub fn into_entries(self) -> impl Iterator<Item = CorpusEntry> {
        let entries = self.pages.into_iter().map(|p| CorpusEntry::Page(p));
        let includes = self
            .included
            .into_iter()
            .map(|p| CorpusEntry::StaticAsset(p));

        entries.chain(includes)
    }

    pub fn entries(&self) -> impl Iterator<Item = CorpusEntryRef<'_>> {
        let entries = self.pages.iter().map(|p| CorpusEntryRef::Page(p));
        let includes = self.included.iter().map(|p| CorpusEntryRef::StaticAsset(p));
        entries.chain(includes)
    }

    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        self.pages.iter()
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
