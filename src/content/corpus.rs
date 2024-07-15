use super::page::Page;
use super::PageBuilder;
use crate::files;
use crate::ids;
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
pub struct Corpus {
    corpus: HashMap<ids::Id<CorpusEntry>, CorpusEntry>,
    ids: ids::IdPool<CorpusEntry>,
}

#[derive(Debug)]
pub enum CorpusEntry {
    Page(Page),
    StaticAsset(IncludedPath),
}

#[derive(Debug, Clone)]
pub struct IncludedPath(files::Path);

impl From<&IncludedPath> for files::Path {
    fn from(value: &IncludedPath) -> Self {
        value.0.clone()
    }
}

impl From<files::FilePath> for IncludedPath {
    fn from(value: files::FilePath) -> Self {
        Self(value.into())
    }
}

impl From<files::DirPath> for IncludedPath {
    fn from(value: files::DirPath) -> Self {
        Self(value.into())
    }
}

#[allow(unused)]
impl Corpus {
    pub fn create(nonce: u64) -> Self {
        Self {
            corpus: Default::default(),
            ids: ids::IdPool::new(nonce),
        }
    }

    pub fn make_page<F: Into<files::FilePath>>(&mut self, f: F) -> PageBuilder {
        PageBuilder::new(self.ids.next(), f)
    }

    pub fn add_page(&mut self, page: PageBuilder) -> crate::Result<()> {
        self.corpus
            .insert(page.id.clone(), CorpusEntry::Page(page.build()?));
        Ok(())
    }

    pub fn include_asset<P: Into<files::Path>>(&mut self, p: P) -> crate::Result<()> {
        self.corpus.insert(
            self.ids.next(),
            CorpusEntry::StaticAsset(IncludedPath(p.into())),
        );
        Ok(())
    }

    pub fn into_entries(self) -> impl Iterator<Item = CorpusEntry> {
        self.corpus.into_values()
    }

    pub fn entries(&self) -> impl Iterator<Item = &CorpusEntry> {
        self.corpus.values()
    }

    pub fn entries_mut(&mut self) -> impl Iterator<Item = &mut CorpusEntry> {
        self.corpus.values_mut()
    }

    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        self.entries().filter_map(|entry| match entry {
            CorpusEntry::Page(p) => Some(p),
            _ => None,
        })
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
