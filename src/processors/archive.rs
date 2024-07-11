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
    site::{self, RenderedPage, RenderedPageMetadata},
};

#[derive(Debug, serde::Serialize)]
pub struct ArchiveEntry<'a> {
    summary: &'a str,
    title: &'a str,
}

pub struct DateArchivist<'a>(pub &'a str);
pub enum TagSorting {
    Alphabetical,
    Popularity,
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
        buckets: &mut HashMap<String, Vec<content::Origin>>,
    ) -> crate::Result<()> {
        if let Some(when) = &page.meta.when {
            buckets
                .entry(self.format_date(when))
                .or_insert_with(Vec::new)
                .push(page.meta.origin.clone())
        }

        Ok(())
    }
}

impl Archivist for TagArchivist {
    fn archive_page(
        &mut self,
        page: &content::Page,
        buckets: &mut HashMap<String, Vec<content::Origin>>,
    ) -> crate::Result<()> {
        if let Some(Metadata::List(tags)) = page.meta.meta.get("tags") {
            for tag in tags.iter().filter_map(|t| match t {
                Metadata::Str(s) => Some(s.to_lowercase()),
                _ => None,
            }) {
                buckets
                    .entry(tag)
                    .or_insert_with(Vec::new)
                    .push(page.meta.origin.clone())
            }
        }

        Ok(())
    }
}

pub struct Archive<A>
where
    A: Archivist,
{
    buckets: HashMap<String, Vec<content::Origin>>,
    archivist: A,
    metadata: RenderedPageMetadata,
}

pub trait Archivist {
    fn archive_page(
        &mut self,
        page: &content::Page,
        buckets: &mut HashMap<String, Vec<content::Origin>>,
    ) -> crate::Result<()>;
}

impl<A> Archive<A>
where
    A: Archivist,
{
    pub fn new(archivist: A, metadata: RenderedPageMetadata) -> Self {
        Self {
            archivist,
            metadata,
            buckets: Default::default(),
        }
    }

    fn create_renderable_archive<'call, 'render>(
        &'call self,
        corpus: &'render content::Corpus,
        site: &'render site::RenderedSite,
    ) -> HashMap<&str, Vec<ArchiveEntry<'_>>>
    where
        'render: 'call,
    {
        let mut buckets = HashMap::with_capacity(self.buckets.len());

        for (k, v) in self.buckets.iter() {
            buckets.insert(
                k.as_str(),
                v.iter()
                    .map(|o| {
                        let page = site.get_page_by_origin(o).unwrap();
                        ArchiveEntry {
                            title: &page.metadata().title,
                            summary: &page.metadata().summary,
                        }
                    })
                    .collect(),
            );
        }

        buckets
    }
}

impl<A> site::Processor for Archive<A>
where
    A: Archivist,
{
    fn process(&mut self, corpus: &mut content::Corpus) -> crate::Result<()> {
        for page in corpus.pages() {
            self.archivist.archive_page(page, &mut self.buckets)?
        }

        Ok(())
    }

    fn site_render(
        &mut self,
        renderer: &mut minijinja::Environment<'_>,
        corpus: &content::Corpus,
        site: &mut site::RenderedSite,
    ) -> crate::Result<()> {
        let tpl = renderer.get_template(&self.metadata.tpl_name)?;

        let content = tpl.render(
            minijinja::context! { archive => Vec::from_iter(self.create_renderable_archive(corpus, site).iter()) }
        )?;
        site.add_page(
            unsafe { FilePath::new("tags.html") },
            RenderedPage::new(content.into_bytes(), self.metadata.clone()),
        );
        Ok(())
    }
}
