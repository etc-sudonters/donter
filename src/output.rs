use super::Result;
use crate::site::RenderedSite;

pub trait Sink {
    fn write(&mut self, site: RenderedSite) -> Result<()>;
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

pub struct S3 {}
pub struct Files {}
pub struct Tar {}

impl Sink for S3 {
    fn write(&mut self, site: RenderedSite) -> Result<()> {
        todo!()
    }
}
impl Sink for Files {
    fn write(&mut self, site: RenderedSite) -> Result<()> {
        todo!()
    }
}

impl Sink for Tar {
    fn write(&mut self, site: RenderedSite) -> Result<()> {
        todo!()
    }
}
