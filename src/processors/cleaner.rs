use crate::{files, site};
use std::fs;

pub struct Cleaner(files::Path);

impl Cleaner {
    pub fn new<P: Into<files::Path>>(p: P) -> Self {
        Cleaner(p.into())
    }
}

impl site::Processor for Cleaner {
    fn initialize<'call, 'init>(
        &'call mut self,
        site: &'call mut site::Initializer<'init, '_>,
    ) -> crate::Result<()>
    where
        'init: 'call,
    {
        use files::Path::*;

        match &self.0 {
            File(f) => {
                fs::remove_file(f)?;
            }
            Dir(d) => {
                fs::remove_dir_all(d)?;
                fs::create_dir_all(d);
            }
        };

        Ok(())
    }
}
