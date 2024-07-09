use crate::{content, files, site};
use std::io::Read;

pub struct Loader {}

impl Default for Loader {
    fn default() -> Self {
        Loader {}
    }
}

impl site::Loader for Loader {
    fn accept(&mut self, path: &files::FilePath) -> crate::Result<bool> {
        match path.as_path().extension() {
            None => Ok(false),
            Some(ext) => Ok("md" == ext),
        }
    }

    fn load(
        &mut self,
        mut content: Box<dyn Read>,
        builder: &mut content::PageBuilder,
    ) -> crate::Result<()> {
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

        let node = markdown::to_mdast(&buf, &opts).map_err(|e| Error::ParseError(e))?;
        MarkdownPageBuilder::new(builder, &opts).build(&node)
    }
}
