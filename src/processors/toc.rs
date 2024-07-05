use crate::{content::doctree::Element, site};

pub struct TableOfContents;

impl site::Processor for TableOfContents {
    fn page_render(
        &mut self,
        page: &crate::content::Page,
        ctx: &mut crate::jinja::RenderContext,
    ) -> crate::Result<()> {
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
