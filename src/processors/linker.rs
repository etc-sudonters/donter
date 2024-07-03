use std::collections::HashMap;
use url::Url;

use crate::{files, site};

#[derive(Debug, Clone, Copy)]
pub enum ArticleSlugStyle {
    Directory,
    Page,
}

pub struct Options {
    pub(crate) content_base: files::DirPath,
    pub(crate) site_base: Url,
    pub(crate) slug_style: ArticleSlugStyle,
    pub(crate) article_prefix: Option<String>,
}

impl Options {
    pub fn canonicalize(self) -> Options {
        Options {
            content_base: unsafe {
                files::DirPath::new(
                    std::fs::canonicalize(self.content_base)
                        .expect("cannot canonicalize content base"),
                )
            },
            ..self
        }
    }
}

// knows the origin filepath and the generated URL, updates intrasite hrefs to
// use these new URLs
pub struct Linker {
    opts: Options,
    entries: HashMap<files::FilePath, files::FilePath>,
}

impl site::Processor for Linker {
    fn page_load(&mut self, page: &mut crate::content::PageBuilder) -> crate::Result<()> {
        page.url_or(self.slug(page));
        Ok(())
    }
}

impl Linker {
    pub fn new(opts: Options) -> Linker {
        Self {
            opts: opts.canonicalize(),
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
            Some(pre) => unsafe { files::FilePath::new([pre.as_str(), name.as_str()].join("/")) },
        }
    }
}
