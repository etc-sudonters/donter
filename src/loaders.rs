use crate::content::Loader;

pub fn default() -> Vec<Box<dyn Loader>> {
    vec![Box::new(md::Loader::default())]
}

pub mod md {
    use crate::files;
    use crate::{content, doctree, Res};
    use markdown;
    use std::ops::{Deref, DerefMut};

    pub struct Loader {}

    impl Default for Loader {
        fn default() -> Self {
            Loader {}
        }
    }

    impl content::Loader for Loader {
        fn accept(&mut self, path: &files::FilePath) -> crate::Res<bool> {
            match path.as_ref().extension() {
                None => Ok(false),
                Some(ext) => Ok("md" == ext),
            }
        }
        fn load(
            &mut self,
            path: &files::FilePath,
            corpus: &mut crate::content::Corpus,
            builder: &mut content::PageBuilder,
        ) -> crate::Res<()> {
            let mut builder = PageBuilder::from(builder);
            corpus.push_page(
                builder
                    // order matters because we're wrapping and only intercept some calls into ourself
                    .contents(std::fs::read_to_string(path)?)
                    .path(path.to_owned())
                    .build()
                    .unwrap(),
            );

            Ok(())
        }
    }

    // don't carry _ANY_ state of our own b/c it'll be lost
    struct PageBuilder<'a>(&'a mut content::PageBuilder);

    impl<'a> PageBuilder<'a> {
        fn contents(&mut self, c: String) -> &mut PageBuilder<'a> {
            match try_parse(c) {
                Ok(tree) => {
                    self.0.contents(tree);
                }
                _ => {}
            }

            self
        }
    }

    impl<'a> From<&'a mut content::PageBuilder> for PageBuilder<'a> {
        fn from(value: &'a mut content::PageBuilder) -> Self {
            PageBuilder(value)
        }
    }

    impl<'a> Deref for PageBuilder<'a> {
        type Target = content::PageBuilder;
        fn deref(&self) -> &Self::Target {
            self.0
        }
    }

    impl<'a> DerefMut for PageBuilder<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.0
        }
    }

    fn try_parse<'a, S: AsRef<str>>(s: S) -> Res<doctree::Document> {
        match markdown::to_mdast(s.as_ref(), &Default::default()) {
            Ok(mdast) => match convert_md_to_doctree(&mdast) {
                Some(doc) => Ok(doc.into()),
                None => panic!("Could not parse markdown to document!"),
            },
            _ => panic!(),
        }
    }

    fn convert_md_to_doctree(md: &markdown::mdast::Node) -> Option<doctree::Element> {
        use doctree::Element;
        use markdown::mdast::Node;
        match md {
            Node::Root(r) => Some(Element::Group(
                r.children
                    .iter()
                    .filter_map(convert_md_to_doctree)
                    .collect(),
            )),
            Node::Code(code) => {
                // TODO get rid of clones -- mem::replace would be nice if possible
                let content = doctree::CodeLiteral::from(code.value.clone());
                let lang = code.lang.clone().map(|l| doctree::CodeLanguage::from(l));
                let meta = code.meta.clone();
                Some(Element::Code(doctree::CodeBlock::new(content, lang, meta)))
            }
            Node::BlockQuote(_) => Some(Element::BlockQuote),
            Node::Break(_) => Some(Element::Break),
            Node::Emphasis(_) => Some(Element::Emphasis),
            Node::FootnoteReference(_) => Some(Element::FootnoteReference),
            Node::FootnoteDefinition(_) => Some(Element::FootnoteDefinition),
            Node::Heading(_) => Some(Element::Heading),
            Node::Image(_) => Some(Element::Image),
            Node::ImageReference(_) => Some(Element::ImageReference),
            Node::InlineCode(_) => Some(Element::InlineCode),
            Node::Link(_) => Some(Element::Link),
            Node::Definition(_) => Some(Element::LinkDefinition),
            Node::LinkReference(_) => Some(Element::LinkReference),
            Node::Paragraph(_) => Some(Element::Paragraph),
            Node::Strong(_) => Some(Element::Strong),
            Node::Table(_) => Some(Element::Table),
            Node::Text(_) => Some(Element::Text),
            Node::ThematicBreak(_) => Some(Element::ThematicBreak),
            _ => None,
        }
    }
}
