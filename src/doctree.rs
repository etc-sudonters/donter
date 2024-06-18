#[derive(Debug)]
pub struct Document {
    content: Vec<Element>,
}

impl Default for Document {
    fn default() -> Self {
        Document { content: vec![] }
    }
}

impl FromIterator<Element> for Document {
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        Document {
            content: iter
                .into_iter()
                // hoist root to top if we find it
                .flat_map(|elm| match elm {
                    Element::Group(r) => r.children,
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
                Element::Group(r) => r.children,
                _ => vec![value],
            },
        }
    }
}

#[derive(Debug)]
pub enum Element {
    Group(Group),
    //
    BlockQuote,
    Break,
    Code(CodeBlock),
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
pub struct Group {
    children: Vec<Element>,
}

#[derive(Debug)]
pub struct CodeLiteral(String);

impl From<String> for CodeLiteral {
    fn from(value: String) -> Self {
        CodeLiteral(value)
    }
}

#[derive(Debug)]
pub struct CodeLanguage(String);

impl From<String> for CodeLanguage {
    fn from(value: String) -> Self {
        CodeLanguage(value)
    }
}

#[derive(Debug)]
pub struct CodeBlock {
    code: CodeLiteral,
    lang: Option<CodeLanguage>,
    // freeform string -- it's not of use here but transformers may want it
    meta: Option<String>,
}

impl CodeBlock {
    pub fn new(code: CodeLiteral, lang: Option<CodeLanguage>, meta: Option<String>) -> CodeBlock {
        CodeBlock { code, lang, meta }
    }
}

impl FromIterator<Element> for Group {
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        Group {
            children: iter.into_iter().collect(),
        }
    }
}

impl From<Element> for Group {
    fn from(value: Element) -> Self {
        Group {
            children: vec![value],
        }
    }
}
