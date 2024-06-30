use crate::{content::IncludedPath, site};

pub struct StaticFiles(Vec<IncludedPath>);

impl site::Processor for StaticFiles {
    fn site_render(&mut self, site: &mut site::RenderedSite) -> crate::Result<()> {
        Ok(())
    }
}
