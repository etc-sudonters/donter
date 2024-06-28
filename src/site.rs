use crate::{content, files, jinja, Result};

pub trait Loader {
    fn accept(&mut self, path: &files::FilePath) -> Result<bool>;
    fn load(
        &mut self,
        content: files::NamedReader,
        builder: content::PageBuilder,
    ) -> crate::Result<content::Page>;
}

pub struct Processors(Vec<Box<dyn Processor>>);

impl Default for Processors {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Processors {
    pub fn new(p: Vec<Box<dyn Processor>>) -> Self {
        Self(p)
    }

    pub fn push(&mut self, p: Box<dyn Processor>) {
        self.0.push(p)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn Processor>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Processor>> {
        self.0.iter_mut()
    }
}

pub trait Processor {
    fn initialize(&mut self, corpus: &mut content::Corpus) -> Result<()> {
        Ok(())
    }

    fn page_load(&mut self, corpus: &mut content::Corpus, page: &content::Page) -> Result<()> {
        Ok(())
    }

    fn generate(&mut self, corpus: &mut content::Corpus) -> Result<()> {
        Ok(())
    }

    fn page_render(&mut self, page: &content::Page, ctx: &mut jinja::RenderContext) -> Result<()> {
        Ok(())
    }

    fn site_render(&mut self) -> Result<()> {
        Ok(())
    }

    fn finalize(&mut self) -> Result<()> {
        Ok(())
    }
}

trait Writer {
    fn write(&mut self, site: Site) -> Result<()>;
}

pub struct Site {}
