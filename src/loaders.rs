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
            Ok(mdast) => Ok(convert_md_to_doctree(&mdast).into()),
            _ => panic!(),
        }
    }

    fn convert_md_to_doctree(md: &markdown::mdast::Node) -> doctree::Element {
        use doctree::Element;
        use markdown::mdast::Node;
        match md {
            Node::Root(r) => Element::Root(r.children.iter().map(convert_md_to_doctree).collect()),
            Node::Code(_) => Element::Code,
            Node::BlockQuote(_) => Element::BlockQuote,
            Node::Break(_) => Element::Break,
            Node::Emphasis(_) => Element::Emphasis,
            Node::FootnoteReference(_) => Element::FootnoteReference,
            Node::FootnoteDefinition(_) => Element::FootnoteDefinition,
            Node::Heading(_) => Element::Heading,
            Node::Image(_) => Element::Image,
            Node::ImageReference(_) => Element::ImageReference,
            Node::InlineCode(_) => Element::InlineCode,
            Node::Link(_) => Element::Link,
            Node::Definition(_) => Element::LinkDefinition,
            Node::LinkReference(_) => Element::LinkReference,
            Node::Paragraph(_) => Element::Paragraph,
            Node::Strong(_) => Element::Strong,
            Node::Table(_) => Element::Table,
            Node::Text(_) => Element::Text,
            Node::ThematicBreak(_) => Element::ThematicBreak,
            _ => Element::Empty,
        }
    }
}
