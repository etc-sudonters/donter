use std::collections::HashMap;
use url::Url;

use crate::{files, site};

pub struct Options {
    pub(crate) content_base: files::DirPath,
    pub(crate) site_base: Url,
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
    entries: HashMap<files::FilePath, Url>,
}

impl site::Processor for Linker {
    fn page_load(&mut self, page: &mut crate::content::PageBuilder) -> crate::Result<()> {
        page.url_or(self.slug(&page.filepath));
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

    fn slug(&self, origin: &files::FilePath) -> Url {
        let path = std::fs::canonicalize(origin).expect("could not canonicalize origin");
        let path = path
            .strip_prefix(&self.opts.content_base)
            .expect("could not strip content base")
            .to_str()
            .unwrap();
        let ext = origin.extension().unwrap().to_string_lossy().to_string();

        let url = self
            .opts
            .site_base
            .join(path.replace(ext.as_str(), "html").as_str());
        println!("converted origin to: {:?} and {:?}", path, url);
        url.unwrap()
    }
}
