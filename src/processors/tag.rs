use std::collections::HashMap;

use crate::{content, site};

pub struct TagToken(u64);

pub struct Tags {
    entries: HashMap<TagToken, Vec<content::PageToken>>,
}

impl site::Processor for Tags {
    fn initialize(&mut self, corpus: &mut content::Corpus) -> crate::Result<()> {
        // 1. Add tag things to jinja
        Ok(())
    }

    fn page_load(
        &mut self,
        corpus: &mut content::Corpus,
        page: &content::Page,
    ) -> crate::Result<()> {
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

    fn site_render(&mut self) -> crate::Result<()> {
        // generate tag pages
        Ok(())
    }
}
