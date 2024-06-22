use crate::files::DirPath;
use crate::Result;
use std::env;

#[derive(Clone, Debug)]
pub struct Configuration {
    content_: Content,
    site_: Site,
}

pub fn load(args: env::Args) -> Result<Configuration> {
    let content_base = args.into_iter().nth(1).expect("content path is required");

    Ok(Configuration {
        content_: Content {
            base: unsafe { DirPath::new(content_base) },
        },
        site_: Site {},
    })
}

impl Configuration {
    pub fn content(&mut self) -> Content {
        self.content_.clone()
    }
    pub fn site(&mut self) -> Site {
        self.site_.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Content {
    base: DirPath,
}
impl Content {
    pub fn base(&self) -> DirPath {
        self.base.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Site {}
