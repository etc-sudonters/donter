use std::marker::PhantomData;

use crate::{
    content::doctree::Element,
    site::{self, PageTemplate, RenderingPage},
};

pub struct Toc {
    pub(crate) depth: u8,
}

impl site::Processor for Toc {
    fn page_rendering<'render, 'site>(
        &self,
        _: &crate::content::Page,
        _: &mut RenderingPage<'render, 'site>,
    ) -> crate::Result<()>
    where
        'site: 'render,
    {
        Ok(())
    }
}

impl Toc {}

struct Heading<'render> {
    depth: u8,
    display: &'render str,
    id: &'render str,
}
