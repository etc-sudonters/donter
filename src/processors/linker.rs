use std::collections::HashMap;
use url::Url;

use crate::{files, site};

// knows the origin filepath and the generated URL, updates intrasite hrefs to
// use these new URLs
pub struct Linker {
    entries: HashMap<files::FilePath, Url>,
}

impl site::Processor for Linker {
    fn page_load(&mut self, page: &mut crate::content::PageBuilder) -> crate::Result<()> {
        page.url_or(self.slug(&page.filepath));
        Ok(())
    }
}

impl Linker {
    pub fn new() -> Linker {
        Self {
            entries: HashMap::new(),
        }
    }

    fn slug(&self, origin: &files::FilePath) -> Url {
        let path = std::fs::canonicalize(origin)
            .map(|p| p.into_os_string().into_string().unwrap())
            .unwrap();
        Url::from_file_path(&path).unwrap()
    }
}
