use crate::files;
use crate::{content, doctree};
use std::ops::{Deref, DerefMut};

use super::translate::{ParseError, ParseResult};

fn try_parse<'a, S: AsRef<str>>(s: S) -> ParseResult {
    match markdown::to_mdast(s.as_ref(), &Default::default()) {
        Ok(mdast) => doctree::Element::try_from(&mdast),
        _ => Err(ParseError),
    }
}

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
                self.0.contents(doctree::Document::from(tree));
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
