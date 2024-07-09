use crate::{
    content::{self, IncludedPath},
    site,
};

pub struct StaticFiles(Vec<IncludedPath>);

impl site::Processor for StaticFiles {
    fn site_render(
        &mut self,
        renderer: &mut minijinja::Environment<'_>,
        corpus: &content::Corpus,
        site: &mut site::RenderedSite,
    ) -> crate::Result<()> {
        Ok(())
    }
}
