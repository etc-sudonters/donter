use std::fmt::Debug;

#[derive(Debug, serde::Serialize)]
pub enum Element {
    BlockQuote(Group),
    CodeBlock(Code),
    Delete(Group),
    Emphasis(Group),
    Empty,
    FootnoteReference(FootnoteReference),
    Group(Group),
    Heading(Header),
    HrefReference(HrefReference),
    ImageReference(ImageReference),
    InlineCode(Code),
    List(List),
    Paragraph(Group),
    Strong(Group),
    Table(Table),
    Text(Text),
}

#[derive(Debug, serde::Serialize)]
pub struct List {
    items: Vec<ListItem>,
}

impl Default for List {
    fn default() -> Self {
        List { items: vec![] }
    }
}

impl List {
    pub fn push(&mut self, item: ListItem) {
        self.items.push(item);
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ListItem(Group);

impl From<Group> for ListItem {
    fn from(value: Group) -> Self {
        ListItem(value)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Group {
    children: Vec<Element>,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Group {
    pub fn push(&mut self, elm: Element) {
        self.children.push(elm)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct CodeLiteral(String);

impl From<String> for CodeLiteral {
    fn from(value: String) -> Self {
        CodeLiteral(value)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct CodeLanguage(String);

impl From<String> for CodeLanguage {
    fn from(value: String) -> Self {
        CodeLanguage(value)
    }
}

#[derive(Debug, serde::Serialize)]
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

#[derive(Debug, serde::Serialize)]
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

#[derive(serde::Serialize)]
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

#[derive(Debug, serde::Serialize)]
pub struct Link {
    href: String,
    content: Box<Element>,
    title: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct HrefDefinition {
    id: String,
    href: String,
}

impl HrefDefinition {
    pub fn create(id: String, href: String) -> Self {
        Self { id, href }
    }
}

#[derive(Debug, serde::Serialize)]
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

#[derive(Debug, serde::Serialize)]
pub struct Image {
    alt: String,
    href: String,
}

impl Image {
    pub fn create(href: String, alt: String) -> Self {
        Self { alt, href }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ImageReference {
    href: String,
    alt: String,
}

impl ImageReference {
    pub fn create(href: String, alt: String) -> ImageReference {
        Self { href, alt }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct Table {
    rows: Vec<TableRow>,
}

impl Table {
    pub fn new() -> Table {
        Self {
            rows: Default::default(),
        }
    }

    pub fn push(&mut self, row: TableRow) {
        self.rows.push(row);
    }
}

#[derive(Debug, serde::Serialize)]
pub struct TableRow {
    cells: Vec<TableCell>,
}

impl TableRow {
    pub fn new() -> TableRow {
        Self { cells: vec![] }
    }

    pub fn push(&mut self, cell: TableCell) {
        self.cells.push(cell);
    }
}

#[derive(Debug, serde::Serialize)]
pub struct TableCell(Box<Element>);

impl TableCell {
    pub fn create(elm: Element) -> Self {
        Self(Box::new(elm))
    }
}

#[derive(Debug, serde::Serialize)]
pub struct FootnoteReference(String);

impl FootnoteReference {
    pub fn create(id: String) -> Self {
        Self(id)
    }
}

impl From<&String> for FootnoteReference {
    fn from(value: &String) -> Self {
        FootnoteReference(value.clone())
    }
}

impl From<FootnoteReference> for Element {
    fn from(value: FootnoteReference) -> Self {
        Element::FootnoteReference(value)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct FootnoteDefinition {
    id: String,
    content: Box<Element>,
}

impl FootnoteDefinition {
    pub fn create<E: Into<Element>>(id: String, content: E) -> Self {
        Self {
            id,
            content: Box::new(content.into()),
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

impl Code {
    pub fn block(self) -> Element {
        Element::CodeBlock(self)
    }

    pub fn inline(self) -> Element {
        Element::InlineCode(self)
    }
}
