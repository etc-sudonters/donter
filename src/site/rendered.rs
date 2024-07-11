use super::SiteError;
use crate::content;
use crate::files;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct RenderedPage {
    content: VecDeque<u8>,
    meta: RenderedPageMetadata,
}

#[derive(Debug, Clone)]
pub struct RenderedPageMetadata {
    pub(crate) title: String,
    pub(crate) origin: content::Origin,
    pub(crate) url: files::FilePath,
    pub(crate) when: Option<String>,
    pub(crate) status: content::PageStatus,
    pub(crate) tpl_name: String,
    pub(crate) summary: String,
}

impl RenderedPage {
    pub fn new<I: Into<VecDeque<u8>>>(content: I, meta: RenderedPageMetadata) -> Self {
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

pub struct IncludedAsset(files::Path);

impl IncludedAsset {
    pub fn read(self) -> crate::Result<impl std::io::Read> {
        Ok(std::fs::File::open(self.0)?)
    }

    pub fn path(self) -> files::Path {
        self.0
    }
}

pub enum Writable {
    Page(RenderedPage),
    Asset(IncludedAsset),
}

pub struct RenderedSite {
    writables: HashMap<files::Path, Writable>,
    origins: HashMap<content::Origin, files::Path>,
}

impl RenderedSite {
    pub fn new() -> RenderedSite {
        Self {
            writables: Default::default(),
            origins: Default::default(),
        }
    }

    pub fn entries(self) -> impl std::iter::Iterator<Item = (files::Path, Writable)> {
        self.writables.into_iter()
    }

    pub fn get_page_by_origin(&self, origin: &content::Origin) -> Option<&RenderedPage> {
        match self.origins.get(origin) {
            None => None,
            Some(dest) => match self.writables.get(dest).unwrap() {
                Writable::Page(p) => Some(p),
                _ => None,
            },
        }
    }

    pub fn add_page(&mut self, path: files::FilePath, content: RenderedPage) -> crate::Result<()> {
        match self.writables.entry(path.clone().into()) {
            Entry::Vacant(v) => {
                let origin = &content.meta.origin;
                self.origins.insert(origin.clone(), v.key().clone());

                v.insert(Writable::Page(content));
                Ok(())
            }
            Entry::Occupied(o) => Err(Box::new(SiteError::AlreadyOccupied(o.key().clone()))),
        }
    }

    pub fn add_static_asset(
        &mut self,
        path: files::Path,
        content: &content::IncludedPath,
    ) -> crate::Result<()> {
        match self.writables.entry(path) {
            Entry::Vacant(v) => {
                v.insert(Writable::Asset(IncludedAsset(content.clone().into())));
                Ok(())
            }
            Entry::Occupied(o) => Err(Box::new(SiteError::AlreadyOccupied(o.key().clone()))),
        }
    }
}
