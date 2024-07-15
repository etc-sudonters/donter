use std::borrow::Cow;

use crate::{
    content::{self, IncludedPath},
    jinja, site,
};

pub struct StaticFiles(pub(crate) IncludedPath);

impl site::Processor for StaticFiles {
    fn initialize<'call, 'init>(
        &'call mut self,
        _: &'call mut site::initializer::Initializer<'init, '_>,
    ) -> crate::Result<()>
    where
        'init: 'call,
    {
        println!("let's pretend I'm loading assets from {:?}", self.0);
        Ok(())
    }

    fn site_rendering<'site>(
        &self,
        _: &'site content::Corpus,
        _: &mut site::rendered::RenderingSite<'_, 'site, '_>,
    ) -> crate::Result<()> {
        Ok(())
    }
}
