use std::collections::{
    hash_map::{Entry, OccupiedEntry, VacantEntry},
    HashMap,
};

use crate::{
    content::{self, Metadata},
    files::{self, FilePath},
    site::{self, RenderedPage},
};

#[derive(Debug, serde::Serialize)]
pub struct ArchiveEntry {
    title: String,
    summary: Option<String>,
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
        buckets: &mut HashMap<String, Vec<ArchiveEntry>>,
    ) -> crate::Result<()> {
        if let Some(when) = &page.meta.when {
            buckets
                .entry(self.format_date(when))
                .or_insert_with(Vec::new)
                .push(ArchiveEntry {
                    title: page.meta.title.clone(),
                    summary: page
                        .meta
                        .meta
                        .get("summary")
                        .map(|m| {
                            if let Metadata::Str(s) = m {
                                Some(s.clone())
                            } else {
                                None
                            }
                        })
                        .flatten(),
                });
        }

        Ok(())
    }
}

impl Archivist for TagArchivist {
    fn archive_page(
        &mut self,
        page: &content::Page,
        buckets: &mut HashMap<String, Vec<ArchiveEntry>>,
    ) -> crate::Result<()> {
        if let Some(Metadata::List(tags)) = page.meta.meta.get("tags") {
            for tag in tags.iter().filter_map(|t| match t {
                Metadata::Str(s) => Some(s.to_lowercase()),
                _ => None,
            }) {
                buckets
                    .entry(tag)
                    .or_insert_with(Vec::new)
                    .push(ArchiveEntry {
                        title: page.meta.title.clone(),
                        summary: page
                            .meta
                            .meta
                            .get("summary")
                            .map(|m| {
                                if let Metadata::Str(s) = m {
                                    Some(s.clone())
                                } else {
                                    None
                                }
                            })
                            .flatten(),
                    });
            }
        }

        Ok(())
    }
}

pub struct Archive<'a, A>
where
    A: Archivist,
{
    buckets: HashMap<String, Vec<ArchiveEntry>>,
    archivist: A,
    tpl_name: &'a str,
}

pub trait Archivist {
    fn archive_page(
        &mut self,
        page: &content::Page,
        buckets: &mut HashMap<String, Vec<ArchiveEntry>>,
    ) -> crate::Result<()>;
}

impl<'a, A> Archive<'a, A>
where
    A: Archivist,
{
    pub fn new(archivist: A, tpl_name: &'a str) -> Self {
        Self {
            archivist,
            tpl_name,
            buckets: Default::default(),
        }
    }
}

impl<'a, A> site::Processor for Archive<'a, A>
where
    A: Archivist,
{
    fn process(&mut self, corpus: &mut content::Corpus) -> crate::Result<()> {
        println!("creating archives...");
        for page in corpus.pages() {
            self.archivist.archive_page(page, &mut self.buckets)?
        }

        Ok(())
    }

    fn site_render(
        &mut self,
        renderer: &mut minijinja::Environment<'_>,
        site: &mut site::RenderedSite,
    ) -> crate::Result<()> {
        let tpl = renderer.get_template(&self.tpl_name)?;
        let content =
            tpl.render(minijinja::context! { archive => Vec::from_iter(self.buckets.iter()) })?;
        println!("creating archive page...");
        site.add_page(
            unsafe { FilePath::new("tags.html") },
            RenderedPage::new(content.into_bytes()),
        );
        Ok(())
    }
}
