use crate::files::DirPath;
use crate::Res;
use std::env;

#[derive(Clone, Debug)]
pub struct Configuration {
    content_: Content,
    site_: Site,
}

pub fn load(_args: env::Args) -> Res<Configuration> {
    Ok(Configuration {
        content_: Content {
            base: unsafe { DirPath::new("/mnt/anr/newlondo/source/blog/content/articles") },
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
