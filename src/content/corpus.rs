use super::page::Page;
use crate::files;
use std::path::Path;

#[derive(Debug)]
pub struct Corpus {
    pages: Vec<Page>,
    included: Vec<IncludedPath>,
}

pub struct CorpusEntries(Vec<CorpusEntry>);
pub enum CorpusEntry {
    Page(Page),
    StaticAsset(IncludedPath),
}

#[derive(Debug, Clone)]
pub struct IncludedPath(files::Path);

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

impl Default for Corpus {
    fn default() -> Self {
        Corpus {
            pages: Vec::new(),
            included: Vec::new(),
        }
    }
}

impl Iterator for CorpusEntries {
    type Item = CorpusEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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
