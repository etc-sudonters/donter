use url::Url;

use crate::files::DirPath;
use crate::Result;
use std::env;

#[derive(Clone, Debug)]
pub struct Configuration {
    pub(crate) content: Content,
    pub(crate) site: Site,
}

pub fn load(args: env::Args) -> Result<Configuration> {
    let these_args: Vec<String> = args.into_iter().collect();
    Ok(Configuration {
        content: Content {
            base: unsafe { DirPath::new(these_args.get(1).expect("content base must be arg 1")) },
        },
        site: Site {
            templates: unsafe {
                DirPath::new(these_args.get(2).expect("template base must be arg 2"))
            },
            base_url: Url::parse(these_args.get(3).expect("base url must be arg 3"))
                .expect("invalid url base provided"),
        },
    })
}

#[derive(Clone, Debug)]
pub struct Content {
    pub(crate) base: DirPath,
}
impl Content {
    pub fn base(&self) -> DirPath {
        self.base.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Site {
    pub(crate) templates: DirPath,
    pub(crate) base_url: Url,
}
