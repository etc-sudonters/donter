use url::Url;

use crate::{files, site, writers};
use std::fs;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub(crate) content: Content,
    pub(crate) site: Site,
    pub(crate) output: Output,
    pub(crate) rendering: Rendering,
}

#[derive(Clone, Debug)]
pub struct Content {
    pub(crate) base: files::DirPath,
    pub(crate) assets: Option<files::DirPath>,
}
impl Content {
    pub fn base(&self) -> files::DirPath {
        self.base.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Site {
    pub(crate) templates: files::DirPath,
    pub(crate) base_url: Url,
}

#[derive(Clone, Debug)]
pub struct Rendering {
    pub(crate) slug_style: site::ArticleSlugStyle,
    pub(crate) page_root: Option<files::DirPath>,
}

#[derive(Clone, Debug)]
pub struct Output {
    pub(crate) output: files::Path,
    pub(crate) clean: bool,
}

impl Output {
    pub fn writer(&self) -> crate::Result<Box<dyn site::Writer>> {
        match &self.output {
            files::Path::Dir(d) => {
                Ok(Box::new(writers::Files::create(d.clone())?) as Box<dyn site::Writer>)
            }
            files::Path::File(p) => {
                Ok(Box::new(writers::Tar::new(fs::File::create(p)?)) as Box<dyn site::Writer>)
            }
        }
    }
}
