use super::page::Page;
use crate::files;
use std::{borrow::Borrow, collections::HashMap, path::Path};

#[derive(Debug)]
pub struct Corpus {
    lookup: HashMap<files::FilePath, usize>,
    entries: Vec<CorpusEntry>,
}

#[derive(Debug)]
pub enum CorpusEntry {
    Page(Page),
    StaticAsset(IncludedPath),
}

#[derive(Debug, Clone)]
pub struct IncludedPath(files::Path);

impl Corpus {
    pub fn get(&self, origin: files::FilePath) -> Option<&CorpusEntry> {
        self.lookup
            .get(&origin)
            .map(|idx| self.entries.get(*idx))
            .flatten()
    }

    pub fn add_page(&mut self, p: Page) {
        let idx = self.entries.len();
        let origin: files::FilePath = (*p.meta.origin).clone();
        self.lookup.insert((*p.meta.origin).clone(), idx);
        self.entries.push(CorpusEntry::Page(p));
    }

    pub fn include_asset(&mut self, p: files::Path) {
        self.entries.push(CorpusEntry::StaticAsset(IncludedPath(p)));
    }

    pub fn into_entries(self) -> impl Iterator<Item = CorpusEntry> {
        self.entries.into_iter()
    }

    pub fn entries(&self) -> impl Iterator<Item = &CorpusEntry> {
        self.entries.iter()
    }

    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        self.entries.iter().filter_map(|entry| match entry {
            CorpusEntry::Page(p) => Some(p),
            _ => None,
        })
    }
}

impl Default for Corpus {
    fn default() -> Self {
        Corpus {
            lookup: Default::default(),
            entries: Default::default(),
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
