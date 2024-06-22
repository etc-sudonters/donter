use crate::files;
use crate::{content, doctree};

use super::translate::ParseError;

fn try_parse<'a, S: AsRef<str>>(s: S) -> Result<doctree::Document, ParseError> {
    use markdown::mdast::Node;
    let mut doc = doctree::Document::default();
    match markdown::to_mdast(s.as_ref(), &Default::default()) {
        Err(e) => panic!("err: {}", e),
        Ok(node) => match node {
            Node::Root(root) => {
                for node in root.children.iter() {
                    doc.push(node.try_into()?);
                }
            }
            // root must be first node
            _ => panic!("expected root node"),
        },
    };

    Ok(doc)
}

pub struct Loader {}

impl Default for Loader {
    fn default() -> Self {
        Loader {}
    }
}

impl content::Loader for Loader {
    fn accept(&mut self, path: &files::FilePath) -> crate::Result<bool> {
        match path.as_ref().extension() {
            None => Ok(false),
            Some(ext) => Ok("md" == ext),
        }
    }

    fn load(
        &mut self,
        mut content: Box<dyn std::io::Read>,
        corpus: &mut crate::content::Corpus,
        mut builder: content::PageBuilder,
    ) -> crate::Result<()> {
        let mut text = String::new();
        let _ = content.read_to_string(&mut text)?;
        corpus.push_page(
            PageBuilder(builder)
                // order matters because we're wrapping and only intercept some calls into ourself
                .contents(text)?
                .try_build()?,
        );

        Ok(())
    }
}

// don't carry _ANY_ state of our own b/c it'll be lost
struct PageBuilder(content::PageBuilder);

impl PageBuilder {
    fn contents(self, c: String) -> crate::Result<content::PageBuilder> {
        match try_parse(c) {
            Ok(doc) => {
                let (content, footnotes, hrefs) = doc.destruct();
                let mut builder = self.0.contents(content);
                builder.footnotes().gather(footnotes);
                builder.hrefs().gather(hrefs);
                Ok(builder)
            }
            Err(any) => Err(Box::new(any)),
        }
    }
}
