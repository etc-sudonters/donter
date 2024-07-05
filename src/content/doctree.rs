use std::fmt::{Debug, Display};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct List {
    li: Vec<ListItem>,
}

impl Default for List {
    fn default() -> Self {
        List { li: vec![] }
    }
}

impl List {
    pub fn push(&mut self, item: ListItem) {
        self.li.push(item);
    }

    pub fn items(&self) -> impl Iterator<Item = &ListItem> {
        self.li.iter()
    }
}

#[derive(Debug)]
pub struct ListItem(Group);

impl ListItem {
    pub fn children(&self) -> &Group {
        &self.0
    }
}

impl From<Group> for ListItem {
    fn from(value: Group) -> Self {
        ListItem(value)
    }
}

#[derive(Debug)]
pub struct Group {
    kids: Vec<Element>,
}

impl Default for Group {
    fn default() -> Self {
        Self { kids: Vec::new() }
    }
}

impl Group {
    pub fn push(&mut self, elm: Element) {
        self.kids.push(elm)
    }

    pub fn children(&self) -> &Vec<Element> {
        &self.kids
    }
}

#[derive(Debug)]
pub struct CodeLiteral(String);

impl CodeLiteral {
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.0.lines().into_iter()
    }
}

impl From<String> for CodeLiteral {
    fn from(value: String) -> Self {
        CodeLiteral(value)
    }
}

impl AsRef<str> for CodeLiteral {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
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
}

impl Code {
    pub fn new(code: CodeLiteral, lang: Option<CodeLanguage>) -> Code {
        Code { code, lang }
    }

    pub fn lang(&self) -> &Option<CodeLanguage> {
        &self.lang
    }

    pub fn content(&self) -> &CodeLiteral {
        &self.code
    }
}

impl Display for CodeLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromIterator<Element> for Group {
    fn from_iter<T: IntoIterator<Item = Element>>(iter: T) -> Self {
        Group {
            kids: iter.into_iter().collect(),
        }
    }
}

impl From<Element> for Group {
    fn from(value: Element) -> Self {
        match value {
            Element::Group(g) => g,
            v @ _ => Group { kids: vec![v] },
        }
    }
}

impl From<Group> for Element {
    fn from(mut value: Group) -> Self {
        match value.kids.len() {
            0 => Element::Empty,
            1 => value.kids.remove(0),
            _ => Element::Group(value),
        }
    }
}

#[derive(Debug)]
pub struct Header {
    depth: u8,
    display: String,
    id: String,
}

impl Header {
    pub fn create(depth: u8, display: String, id: String) -> Self {
        Header { depth, display, id }
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }

    pub fn text(&self) -> &str {
        &self.display
    }

    pub fn label(&self) -> &str {
        &self.id
    }
}

pub struct Text(String);

impl Text {
    pub fn create(s: String) -> Text {
        Text(s)
    }

    pub fn inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
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

#[derive(Debug)]
pub struct HrefDefinition {
    id: String,
    href_: String,
}

impl HrefDefinition {
    pub fn create(id: String, href: String) -> Self {
        Self { id, href_: href }
    }

    pub fn href(&self) -> &String {
        &self.href_
    }
}

#[derive(Debug)]
pub struct HrefReference {
    content: Group,
    id: String,
    title: Option<String>,
}

impl HrefReference {
    pub fn create(id: String, content: Group, title: Option<String>) -> Self {
        Self { content, id, title }
    }

    pub fn children(&self) -> &Group {
        &self.content
    }
}

#[derive(Debug)]
pub struct Image {
    alt: String,
    href: String,
}

#[derive(Debug)]
pub struct ImageReference {
    href: String,
    alt: String,
}

impl ImageReference {
    pub fn create(href: String, alt: String) -> ImageReference {
        Self { href, alt }
    }
}

#[derive(Debug)]
pub struct Table {
    r: Vec<TableRow>,
}

impl Table {
    pub fn new() -> Table {
        Self {
            r: Default::default(),
        }
    }

    pub fn push(&mut self, row: TableRow) {
        self.r.push(row);
    }

    pub fn rows(&self) -> impl Iterator<Item = &TableRow> {
        self.r.iter()
    }
}

#[derive(Debug)]
pub struct TableRow {
    c: Vec<TableCell>,
}

impl TableRow {
    pub fn new() -> TableRow {
        Self { c: vec![] }
    }

    pub fn push(&mut self, cell: TableCell) {
        self.c.push(cell);
    }

    pub fn cells(&self) -> impl Iterator<Item = &TableCell> {
        self.c.iter()
    }
}

#[derive(Debug)]
pub struct TableCell(Group);

impl TableCell {
    pub fn create(g: Group) -> Self {
        Self(g)
    }

    pub fn children(&self) -> &Group {
        &self.0
    }
}

#[derive(Debug)]
pub struct FootnoteReference(String);

impl Display for FootnoteReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
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

#[derive(Debug)]
pub struct FootnoteDefinition {
    id: String,
    content: Group,
}

impl FootnoteDefinition {
    pub fn create<G: Into<Group>>(id: String, content: G) -> Self {
        Self {
            id,
            content: content.into(),
        }
    }

    pub fn children(&self) -> &Group {
        &self.content
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

pub trait Reference {
    fn label(&self) -> &String;
}

impl Reference for &HrefReference {
    fn label(&self) -> &String {
        &self.id
    }
}

impl Reference for &ImageReference {
    fn label(&self) -> &String {
        &self.href
    }
}

impl Reference for &FootnoteReference {
    fn label(&self) -> &String {
        &self.0
    }
}

pub trait DefinitionLookup<R, T>
where
    T: Definition,
    R: Reference,
{
    fn lookup(&self, reference: R) -> Option<&T>;
}

impl Code {
    pub fn block(self) -> Element {
        Element::CodeBlock(self)
    }
}
