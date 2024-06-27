use crate::{content, files, jinja, Result};

pub trait Loader {
    fn accept(&mut self, path: &files::FilePath) -> Result<bool>;
    fn load(
        &mut self,
        content: files::NamedReader,
        builder: content::PageBuilder,
    ) -> crate::Result<content::Page>;
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
