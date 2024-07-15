use crate::{content::CorpusEntry, ids, jinja};
use std::{
    collections::{
        hash_map::{Entry, OccupiedEntry, VacantEntry},
        HashMap,
    },
    hash::Hash,
};

use crate::{
    content::{self, Metadata},
    files::{self, FilePath},
    site::{self, PageTemplate, RenderedPage, RenderedPageMetadata},
};

pub struct Buckets<K, V>(HashMap<K, Vec<V>>)
where
    K: Eq + Hash;

impl<K, V> Default for Buckets<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K, V> Buckets<K, V>
where
    K: Eq + Hash,
{
    pub fn with_capacity(cap: usize) -> Self {
        Self(HashMap::with_capacity(cap))
    }

    pub fn push(&mut self, key: K, value: V) {
        self.0.entry(key).or_insert_with(Vec::new).push(value);
    }

    pub fn insert(&mut self, key: K, mut value: Vec<V>) {
        match self.0.entry(key) {
            Entry::Vacant(v) => {
                v.insert(value);
            }
            Entry::Occupied(o) => {
                o.into_mut().append(&mut value);
            }
        }
    }

    pub fn buckets(&self) -> impl Iterator<Item = (&K, &Vec<V>)> {
        self.0.iter()
    }

    pub fn into_buckets(self) -> impl Iterator<Item = (K, Vec<V>)> {
        self.0.into_iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ArchiveEntry<'a> {
    summary: Option<&'a str>,
    title: &'a str,
}

impl<'a> From<&'a site::RenderedPage<'a>> for ArchiveEntry<'a> {
    fn from(page: &'a site::RenderedPage<'a>) -> Self {
        Self {
            title: &page.metadata().title,
            summary: page.metadata().summary.as_deref(),
        }
    }
}

pub struct DateArchivist<'a>(pub &'a str);
pub enum TagSorting {
    Alphabetical,
}
pub struct TagArchivist(pub TagSorting);

impl<'a> DateArchivist<'a> {
    fn format_date<S: AsRef<str>>(&self, written: S) -> String {
        written.as_ref().to_owned()
    }
}

impl<'a> Archivist for DateArchivist<'a> {
    fn archive_page(
        &mut self,
        page: &content::Page,
        buckets: &mut Buckets<String, ids::Id<content::CorpusEntry>>,
    ) -> crate::Result<()> {
        if let Some(when) = &page.meta.when {
            buckets.push(self.format_date(when), page.id.clone());
        }

        Ok(())
    }
}

impl Archivist for TagArchivist {
    fn archive_page(
        &mut self,
        page: &content::Page,
        buckets: &mut Buckets<String, ids::Id<content::CorpusEntry>>,
    ) -> crate::Result<()> {
        if let Some(Metadata::List(tags)) = page.meta.meta.get("tags") {
            for tag in tags.iter().filter_map(|t| match t {
                Metadata::Str(s) => Some(s.to_lowercase()),
                _ => None,
            }) {
                buckets.push(tag, page.id.clone());
            }
        }

        Ok(())
    }
}

pub struct Archive<'a, A>
where
    A: Archivist,
{
    buckets: Buckets<String, ids::Id<content::CorpusEntry>>,
    archivist: A,
    template: PageTemplate<'a>,
}

pub trait Archivist {
    fn archive_page(
        &mut self,
        page: &content::Page,
        buckets: &mut Buckets<String, ids::Id<content::CorpusEntry>>,
    ) -> crate::Result<()>;
}

impl<'a, A> Archive<'a, A>
where
    A: Archivist,
{
    pub fn new(archivist: A, metadata: PageTemplate<'a>) -> Self {
        Self {
            archivist,
            template: metadata,
            buckets: Default::default(),
        }
    }
}

impl<'archive, A> site::Processor for Archive<'archive, A>
where
    A: Archivist,
{
    fn site_loaded(&mut self, corpus: &mut content::Corpus) -> crate::Result<()> {
        for page in corpus.pages() {
            self.archivist.archive_page(page, &mut self.buckets)?
        }

        Ok(())
    }

    fn site_rendering<'site>(
        &self,
        _: &'site content::Corpus,
        site: &mut site::RenderingSite<'_, 'site, '_>,
    ) -> crate::Result<()> {
        let mut page = site.page(&self.template.template);
        let mut buckets = Buckets::with_capacity(self.buckets.len());

        for (k, v) in self.buckets.buckets() {
            buckets.insert(
                k.as_str(),
                v.iter()
                    .filter_map(|o| match site.get_by_origin(o) {
                        Some(site::Writable::Page(page)) => Some(ArchiveEntry {
                            title: &page.metadata().title,
                            summary: page.metadata().summary.as_deref(),
                        }),
                        _ => None,
                    })
                    .collect(),
            );
        }

        let archives = Vec::from_iter(buckets.into_buckets());
        page.values()
            .merge(minijinja::context! { archive =>  archives });
        let meta = self.template.stamp();
        site.render_page(meta, page)?;

        Ok(())
    }
}
