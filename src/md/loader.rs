use std::io::Read;

use crate::{content, files, site};

pub struct Loader {}

impl Default for Loader {
    fn default() -> Self {
        Loader {}
    }
}

impl site::Loader for Loader {
    fn accept(&mut self, path: &files::FilePath) -> crate::Result<bool> {
        match path.as_ref().extension() {
            None => Ok(false),
            Some(ext) => Ok("md" == ext),
        }
    }

    fn load(
        &mut self,
        mut content: files::NamedReader,
        mut builder: content::PageBuilder,
    ) -> crate::Result<content::Page> {
        use super::Error;
        use crate::md::walker::MarkdownPageBuilder;
        use markdown;
        let mut buf = String::new();
        content.read_to_string(&mut buf)?;

        let opts = markdown::ParseOptions {
            constructs: markdown::Constructs {
                frontmatter: true,
                ..markdown::Constructs::gfm()
            },
            ..markdown::ParseOptions::gfm()
        };

        let node = markdown::to_mdast(&buf, &markdown::ParseOptions::gfm())
            .map_err(|e| Error::ParseError(e))?;
        builder.path(&content.path());
        MarkdownPageBuilder::new(builder).build(&node)
    }
}
