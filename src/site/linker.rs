use std::{borrow::Cow, collections::HashMap};
use url::Url;

use crate::files;

#[derive(Debug, Clone, Copy)]
pub enum ArticleSlugStyle {
    Directory,
    Page,
}

pub enum ArticleSlugSource {
    Filename,
    Title,
}

pub struct Options<'a> {
    pub(crate) page_root: Option<files::DirPath>,
    pub(crate) site_base: Cow<'a, Url>,
    pub(crate) slug_source: ArticleSlugSource,
    pub(crate) slug_style: ArticleSlugStyle,
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Self {
            page_root: None,
            site_base: Cow::Owned(Url::parse("http://localhost:1312").unwrap()),
            slug_source: ArticleSlugSource::Filename,
            slug_style: ArticleSlugStyle::Page,
        }
    }
}

pub struct Linker<'a> {
    opts: Options<'a>,
    // origin -> destination
    entries: HashMap<files::FilePath, files::FilePath>,
}

impl<'a> Linker<'a> {
    pub fn new(opts: Options<'a>) -> Linker {
        Self {
            opts,
            entries: Default::default(),
        }
    }

    pub fn slug(&self, origin: &files::FilePath) -> files::FilePath {
        let ext = format!(
            ".{}",
            origin
                .extension()
                .map(|ext| ext.to_str())
                .flatten()
                .unwrap()
        );
        let name = origin
            .file_name()
            .map(|path| {
                path.to_str()
                    .map(|p| p.strip_suffix(&ext).map(|p| p.to_string()))
                    .flatten()
            })
            .flatten()
            .unwrap();

        let stem = unsafe {
            files::FilePath::new(match self.opts.slug_style {
                ArticleSlugStyle::Page => format!("{name}.html"),
                ArticleSlugStyle::Directory => format!("{name}/index.html"),
            })
        };

        match &self.opts.page_root {
            None => stem,
            Some(pre) => unsafe { files::FilePath::new(pre.join(stem)) },
        }
    }
}
