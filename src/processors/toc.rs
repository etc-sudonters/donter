use crate::{
    content::doctree::Element,
    site::{self, RenderingPage},
};

pub struct TableOfContents;

impl site::Processor for TableOfContents {
    fn page_render<'render, 'site>(
        &self,
        page: &crate::content::Page,
        rendering: &mut RenderingPage<'render, 'site>,
    ) -> crate::Result<()>
    where
        'site: 'render,
    {
        todo!()
    }
}

impl TableOfContents {
    fn extract<'a>(page: &'a crate::content::PageContents) -> Vec<Heading<'a>> {
        let mut heads = Vec::new();

        for elm in page.content.iter() {
            if let Element::Heading(h) = elm {
                heads.push(Heading {
                    depth: h.depth(),
                    id: h.label(),
                    display: todo!(),
                });
            }
        }

        heads
    }
}

struct Heading<'a> {
    depth: u8,
    display: &'a str,
    id: &'a str,
}
