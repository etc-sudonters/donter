use super::IncludedAsset;
use super::SiteError;
use super::Writable;
use crate::content;
use crate::content::CorpusEntry;
use crate::files;
use crate::ids;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::marker::PhantomData;

pub struct RenderedPage<'a> {
    content: VecDeque<u8>,
    meta: RenderedPageMetadata<'a>,
}

pub struct PageTemplate<'a> {
    pub(crate) title: &'a str,
    pub(crate) url: files::FilePath,
    pub(crate) template: &'a str,
}

impl<'a> PageTemplate<'a> {
    pub fn stamp<'b>(&'b self) -> RenderedPageMetadata<'b>
    where
        'a: 'b,
    {
        RenderedPageMetadata {
            title: &self.title,
            url: &self.url,
            when: None,
            summary: None,
            origin: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderedPageMetadata<'a> {
    pub(crate) origin: Option<ids::Id<CorpusEntry>>,
    pub(crate) title: &'a str,
    pub(crate) url: &'a files::FilePath,
    pub(crate) when: Option<&'a str>,
    pub(crate) summary: Option<String>,
}

impl<'a> RenderedPage<'a> {
    pub fn new<I: Into<VecDeque<u8>>>(content: I, meta: RenderedPageMetadata<'a>) -> Self {
        RenderedPage {
            content: content.into(),
            meta,
        }
    }

    pub fn read(self) -> impl std::io::Read {
        self.content
    }

    pub fn size(&self) -> u64 {
        self.content.len() as u64
    }

    pub fn metadata(&self) -> &RenderedPageMetadata {
        &self.meta
    }
}

pub struct RenderedSite<'a> {
    writables: HashMap<ids::Id<RenderedSite<'a>>, Writable<'a>>,
    origins: HashMap<ids::Id<CorpusEntry>, ids::Id<RenderedSite<'a>>>,
    ids: ids::IdPool<RenderedSite<'a>>,
}

impl<'a> RenderedSite<'a> {
    pub fn new() -> Self {
        Self {
            writables: Default::default(),
            origins: Default::default(),
            ids: ids::IdPool::new(1312),
        }
    }

    pub fn add_page(&mut self, page: RenderedPage<'a>) {
        let idx = self.ids.next();
        if let Some(ref origin) = page.meta.origin {
            self.origins.insert(origin.clone(), idx.clone());
        }
        self.writables.insert(idx, Writable::Page(page));
    }

    pub fn add_asset(&mut self, asset: IncludedAsset) {
        self.writables
            .insert(self.ids.next(), Writable::Asset(asset));
    }

    pub fn entries(
        self,
    ) -> impl std::iter::Iterator<Item = (ids::Id<RenderedSite<'a>>, Writable<'a>)> {
        self.writables.into_iter()
    }

    pub fn get<K>(&self, id: K) -> Option<&Writable<'a>>
    where
        K: std::borrow::Borrow<ids::Id<RenderedSite<'a>>>,
    {
        self.writables.get(id.borrow())
    }

    pub fn get_by_origin<K>(&self, origin: K) -> Option<&Writable<'a>>
    where
        K: std::borrow::Borrow<ids::Id<CorpusEntry>>,
    {
        match self.origins.get(origin.borrow()) {
            None => None,
            Some(dest) => self.writables.get(dest), // this will always be Some(...)
        }
    }
}
