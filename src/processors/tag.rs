use std::collections::HashMap;

use crate::{content, site};

pub struct TagToken(u64);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Tag(String);

pub struct Tags;

impl site::Processor for Tags {
    fn initialize<'init, 'site>(
        &'init mut self,
        corpus: &'init mut site::Initializer<'site, '_>,
    ) -> crate::Result<()>
    where
        'site: 'init,
    {
        // 1. Add tag things to jinja
        Ok(())
    }

    fn page_load(&mut self, page: &mut content::PageBuilder) -> crate::Result<()> {
        // read tags off page and record them here
        // 1. How do we get the PageToken?
        Ok(())
    }

    fn page_render(
        &mut self,
        page: &content::Page,
        ctx: &mut crate::jinja::RenderContext,
    ) -> crate::Result<()> {
        // put tags into render context
        Ok(())
    }

    fn site_render(&mut self, site: &mut site::RenderedSite) -> crate::Result<()> {
        // generate tag pages
        Ok(())
    }
}
