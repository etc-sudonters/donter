use crate::{content, site};

fn default() -> Renderer {
    Renderer::create()
}

#[derive(Default)]
pub struct Renderer {}

impl Renderer {
    pub fn create() -> Renderer {
        Default::default()
    }
}

impl site::Renderer for Renderer {
    fn render(&mut self, mut corpus: content::Corpus) -> crate::Result<site::RenderedSite> {
        todo!()
    }
}
