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
            base: unsafe { DirPath::new(these_args.get(1).unwrap()) },
        },
        site: Site {
            templates: unsafe { DirPath::new(these_args.get(2).unwrap()) },
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
}
