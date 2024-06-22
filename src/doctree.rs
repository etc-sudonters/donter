use std::fmt::Debug;

#[derive(Debug)]
pub struct Document {
    content: Vec<Element>,
    footnotes: Vec<FootnoteDefinition>,
    hrefs: Vec<HrefDefinition>,
}

impl Document {
    pub fn push(&mut self, part: DocumentPart) {
        match part {
            DocumentPart::Href(p) => self.hrefs.push(p),
            DocumentPart::Element(p) => self.content.push(p),
            DocumentPart::Footnote(p) => self.footnotes.push(p),
        }
    }

    pub fn destruct(self) -> (Element, Vec<FootnoteDefinition>, Vec<HrefDefinition>) {
        (
            Element::Group(Group::from_iter(self.content)),
            self.footnotes,
            self.hrefs,
        )
    }
}

impl Default for Document {
    fn default() -> Self {
        Document {
            content: vec![],
            footnotes: vec![],
            hrefs: vec![],
        }
    }
}

impl FromIterator<DocumentPart> for Document {
    fn from_iter<T: IntoIterator<Item = DocumentPart>>(iter: T) -> Self {
        let mut doc = Document::default();

        for part in iter.into_iter() {
            match part {
                DocumentPart::Element(elm) => {
                    doc.content.push(elm);
                }
                DocumentPart::Href(href) => {
                    doc.hrefs.push(href);
                }
                DocumentPart::Footnote(ftn) => {
                    doc.footnotes.push(ftn);
                }
            }
        }

        doc
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
            ..Default::default()
        }
    }
}

impl From<DocumentPart> for Document {
    fn from(value: DocumentPart) -> Self {
        Document::from_iter(vec![value])
    }
}

impl From<Element> for Document {
    fn from(value: Element) -> Self {
        Document {
            content: match value {
                Element::Group(r) => r.children,
                _ => vec![value],
            },
            ..Default::default()
        }
    }
}

pub enum DocumentPart {
    Element(Element),
    Footnote(FootnoteDefinition),
    Href(HrefDefinition),
}

impl From<Element> for DocumentPart {
    fn from(value: Element) -> Self {
        DocumentPart::Element(value)
    }
}

impl From<FootnoteDefinition> for DocumentPart {
    fn from(value: FootnoteDefinition) -> Self {
        DocumentPart::Footnote(value)
    }
}

impl From<HrefDefinition> for DocumentPart {
    fn from(value: HrefDefinition) -> Self {
        DocumentPart::Href(value)
    }
}

#[derive(Debug)]
pub enum Element {
    Group(Group),
    Empty,
    //
    BlockQuote(Group),
    Break,
    CodeBlock(Code),
    Emphasis(Group),
    FootnoteReference(FootnoteReference),
    Heading(Header),
    Image(Image),
    ImageReference(HrefReference),
    InlineCode(Code),
    Link(Link),
    HrefReference(HrefReference),
    Paragraph(Group),
    Strong(Group),
    Table(Table),
    Text(Text),
    ThematicBreak,
    List(List),
}

#[derive(Debug)]
pub struct List(Vec<ListItem>, ListStyle);

impl List {
    pub fn style(&mut self, style: ListStyle) {
        self.1 = style;
    }
}

#[derive(Debug)]
pub enum ListStyle {
    Ordered,
    Unordered,
}

impl Default for List {
    fn default() -> Self {
        List(vec![], ListStyle::Unordered)
    }
}

impl List {
    pub fn push(&mut self, item: ListItem) {
        self.0.push(item);
    }
}

#[derive(Debug)]
pub struct ListItem(Group, ItemStyle);

#[derive(Debug)]
pub enum ItemStyle {
    Checked,
    Unchecked,
    Plain,
}

impl From<Group> for ListItem {
    fn from(value: Group) -> Self {
        ListItem(value, ItemStyle::Plain)
    }
}

impl ListItem {
    pub fn style(&mut self, style: ItemStyle) {
        self.1 = style;
    }
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
pub struct Code {
    code: CodeLiteral,
    lang: Option<CodeLanguage>,
    // freeform string -- it's not of use here but transformers may want it
    meta: Option<String>,
}

impl Code {
    pub fn new(code: CodeLiteral, lang: Option<CodeLanguage>, meta: Option<String>) -> Code {
        Code { code, lang, meta }
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
        match value {
            Element::Group(g) => g,
            v @ _ => Group { children: vec![v] },
        }
    }
}

impl From<Group> for Element {
    fn from(mut value: Group) -> Self {
        match value.children.len() {
            0 => Element::Empty,
            1 => value.children.remove(0),
            _ => Element::Group(value),
        }
    }
}

#[derive(Debug)]
pub struct Header {
    depth: u8,
    children: Box<Element>,
}

impl Header {
    pub fn create(depth: u8, children: Element) -> Self {
        Header {
            depth,
            children: Box::new(children),
        }
    }
}

pub struct Text(String);

impl Text {
    pub fn create(s: String) -> Text {
        Text(s)
    }
}

impl Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Text(...)")
    }
}

#[derive(Debug)]
pub struct Link {
    href: String,
    content: Box<Element>,
    title: Option<String>,
}

impl Link {
    pub fn create(href: String, content: Element, title: Option<String>) -> Self {
        Self {
            href,
            content: Box::new(content),
            title,
        }
    }
}

#[derive(Debug)]
pub struct HrefDefinition {
    id: String,
    href: String,
}

impl HrefDefinition {
    pub fn create(id: String, href: String) -> Self {
        Self { id, href }
    }
}

#[derive(Debug)]
pub struct HrefReference {
    content: Box<Element>,
    id: String,
    title: Option<String>,
}

impl HrefReference {
    pub fn create(id: String, content: Element, title: Option<String>) -> Self {
        Self {
            content: Box::new(content),
            id,
            title,
        }
    }
}

#[derive(Debug)]
pub struct Image {
    alt: String,
    href: String,
}

impl Image {
    pub fn create(href: String, alt: String) -> Self {
        Self { alt, href }
    }
}

#[derive(Debug)]
pub struct TableRow {
    cells: Vec<TableCell>,
}

impl FromIterator<TableCell> for TableRow {
    fn from_iter<T: IntoIterator<Item = TableCell>>(iter: T) -> Self {
        Self {
            cells: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug)]
pub struct TableCell(Box<Element>);

impl TableCell {
    pub fn create(elm: Element) -> Self {
        Self(Box::new(elm))
    }
}

#[derive(Debug)]
pub struct Table {
    rows: Vec<TableRow>,
}

impl FromIterator<TableRow> for Table {
    fn from_iter<T: IntoIterator<Item = TableRow>>(iter: T) -> Self {
        Self {
            rows: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug)]
pub struct FootnoteReference(String);

impl FootnoteReference {
    pub fn create(id: String) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
pub struct FootnoteDefinition {
    id: String,
    content: Box<Element>,
}

impl FootnoteDefinition {
    pub fn create(id: String, content: Element) -> Self {
        Self {
            id,
            content: Box::new(content),
        }
    }
}

// marker to use as trait bound
pub trait Definition {
    fn label(&self) -> String;
}
impl Definition for FootnoteDefinition {
    fn label(&self) -> String {
        self.id.clone()
    }
}
impl Definition for HrefDefinition {
    fn label(&self) -> String {
        self.id.clone()
    }
}
