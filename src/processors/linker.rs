use std::collections::HashMap;

use crate::{files, urls};

// knows the origin filepath and the generated URL, updates intrasite hrefs to
// use these new URLs
pub struct Linker {
    entries: HashMap<files::FilePath, urls::Url>,
}

pub trait Slug {
    fn slug(&self) -> urls::Url;
}
