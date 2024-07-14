use std::{collections::HashMap, marker::PhantomData};

use crate::{
    content::{self, Metadata},
    files,
    site::{self, RenderingPage},
};

pub struct Tags;

impl site::Processor for Tags {
    fn page_render<'render, 'site>(
        &self,
        page: &content::Page,
        rendering: &mut RenderingPage<'render, 'site>,
    ) -> crate::Result<()>
    where
        'site: 'render,
    {
        if let Some(Metadata::List(lst)) = page.meta.meta.get("tags") {
            let mut tags = Vec::with_capacity(lst.len());
            for tag in lst.iter() {
                if let Metadata::Str(s) = tag {
                    tags.push(s)
                }
            }

            rendering.values().merge(minijinja::context! {tags => tags});
        }

        Ok(())
    }
}
