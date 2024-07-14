use super::RenderedPage;
use crate::files;

pub struct IncludedAsset {
    src: files::Path,
    dest: files::Path,
}

impl IncludedAsset {
    pub fn create<S: Into<files::Path>, D: Into<files::Path>>(src: S, dest: D) -> Self {
        Self {
            src: src.into(),
            dest: dest.into(),
        }
    }

    pub fn read(self) -> crate::Result<impl std::io::Read> {
        Ok(std::fs::File::open(self.src)?)
    }

    pub fn source(&self) -> &files::Path {
        &self.src
    }

    pub fn destination(&self) -> &files::Path {
        &self.dest
    }
}

pub enum Writable<'a> {
    Page(RenderedPage<'a>),
    Asset(IncludedAsset),
}
