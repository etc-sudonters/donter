use std::collections::HashMap;
use url::Url;

use crate::{
    content::{self, doctree::Href},
    files, site,
};

#[derive(Debug, Clone, Copy)]
pub enum ArticleSlugStyle {
    Directory,
    Page,
}

pub struct Options<'a> {
    pub(crate) content_base: &'a files::DirPath,
    pub(crate) site_base: &'a Url,
    pub(crate) slug_style: ArticleSlugStyle,
    pub(crate) article_prefix: Option<String>,
}

pub struct Linker<'a> {
    opts: Options<'a>,
    // origin -> destination
    entries: HashMap<files::FilePath, files::FilePath>,
}

impl<'a> site::Processor for Linker<'a> {
    fn page_load(&mut self, page: &mut crate::content::PageBuilder) -> crate::Result<()> {
        page.url_or(self.slug(page));
        self.entries
            .insert(page.filepath.clone(), page.url_path.clone().unwrap());
        Ok(())
    }

    fn process(&mut self, corpus: &mut crate::content::Corpus) -> crate::Result<()> {
        Ok(())
    }
}

impl<'a> Linker<'a> {
    pub fn new(opts: Options<'a>) -> Linker {
        Self {
            opts,
            entries: HashMap::new(),
        }
    }

    fn slug(&self, page: &crate::content::PageBuilder) -> files::FilePath {
        let origin = &page.filepath;
        let ext = origin.extension().unwrap().to_string_lossy().to_string();
        let name = origin
            .file_name()
            .map(|s| {
                s.to_string_lossy()
                    .into_owned()
                    .strip_suffix(format!(".{ext}").as_str())
                    .unwrap()
                    .to_owned()
            })
            .unwrap();

        let name = match self.opts.slug_style {
            ArticleSlugStyle::Page => format!("{name}.html"),
            ArticleSlugStyle::Directory => format!("{name}/index.html"),
        };

        match &self.opts.article_prefix {
            None => unsafe { files::FilePath::new(name) },
            Some(pre) => unsafe { files::FilePath::new([pre, name.as_str()].join("/")) },
        }
    }
}
