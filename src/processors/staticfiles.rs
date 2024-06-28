use crate::{content::IncludedPath, site::Processor};

pub struct StaticFiles(Vec<IncludedPath>);

impl Processor for StaticFiles {
    fn site_render(&mut self) -> crate::Result<()> {
        Ok(())
    }
}
