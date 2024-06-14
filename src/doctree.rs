#[derive(Debug)]
pub struct Document {
    content: Vec<Element>,
}

impl FromIterator<Element> for Document {
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        Document {
            content: iter
                .into_iter()
                // hoist root to top if we find it
                .flat_map(|elm| match elm {
                    Element::Root(r) => r.children,
                    _ => vec![elm],
                })
                .collect(),
        }
    }
}

impl From<Element> for Document {
    fn from(value: Element) -> Self {
        Document {
            content: match value {
                Element::Root(r) => r.children,
                _ => vec![value],
            },
        }
    }
}

#[derive(Debug)]
pub enum Element {
    Root(Root),
    Empty,

    BlockQuote,
    Break,
    Code,
    Emphasis,
    FootnoteDefinition,
    FootnoteReference,
    Heading,
    Image,
    ImageReference,
    InlineCode,
    Link,
    LinkDefinition,
    LinkReference,
    Paragraph,
    Strong,
    Table,
    Text,
    ThematicBreak,
}

#[derive(Debug)]
pub struct Root {
    children: Vec<Element>,
}

impl FromIterator<Element> for Root {
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        Root {
            children: iter.into_iter().collect(),
        }
    }
}

impl From<Element> for Root {
    fn from(value: Element) -> Self {
        Root {
            children: vec![value],
        }
    }
}
