use crate::{
    content::{self, IncludedPath},
    jinja, site,
};

pub struct StaticFiles(Vec<IncludedPath>);

impl site::Processor for StaticFiles {
    fn site_rendering<'site>(
        &self,
        corpus: &'site content::Corpus,
        site: &mut site::rendered::RenderingSite<'_, 'site, '_>,
    ) -> crate::Result<()> {
        Ok(())
    }
}
