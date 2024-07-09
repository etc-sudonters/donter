use std::{collections::HashMap, marker::PhantomData};

use crate::{
    content::{self, Metadata},
    files, site,
};

pub struct Tags;

impl site::Processor for Tags {
    fn page_render(
        &mut self,
        page: &content::Page,
        ctx: &mut crate::jinja::RenderContext,
    ) -> crate::Result<()> {
        if let Some(Metadata::List(lst)) = page.meta.meta.get("tags") {
            let mut tags = Vec::with_capacity(lst.len());
            for tag in lst.iter() {
                if let Metadata::Str(s) = tag {
                    tags.push(s)
                }
            }

            ctx.merge(minijinja::context! {tags => tags});
        }

        Ok(())
    }
}
