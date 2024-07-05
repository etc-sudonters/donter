use std::collections::HashMap;

use crate::{
    content::{self, Metadata},
    files, site,
};

pub struct Tags(HashMap<files::FilePath, Vec<String>>);

impl Tags {
    pub fn new() -> Tags {
        Self(Default::default())
    }
}

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
        if let Some(Metadata::List(lst)) = page.meta.get("tags") {
            let mut tags = Vec::with_capacity(lst.len());
            for tag in lst.iter() {
                if let Metadata::Str(s) = tag {
                    tags.push(s.clone());
                }
            }

            self.0.insert(page.filepath.clone(), tags);
        }

        Ok(())
    }

    fn page_render(
        &mut self,
        page: &content::Page,
        ctx: &mut crate::jinja::RenderContext,
    ) -> crate::Result<()> {
        if let Some(tags) = self.0.get(&page.meta.origin.0) {
            ctx.merge(minijinja::context! {tags => tags});
        }

        Ok(())
    }

    fn site_render(
        &mut self,
        renderer: &mut minijinja::Environment<'_>,
        site: &mut site::RenderedSite,
    ) -> crate::Result<()> {
        // generate tag pages
        Ok(())
    }
}
