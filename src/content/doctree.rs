use std::fmt::{Debug, Display};

use url::Url;

use crate::files;

#[derive(Debug)]
#[allow(unused)]
pub enum Href {
    Unparsed(String),
    Url(Url),
    LocalFile(files::Path),
}

#[allow(unused)]
impl Href {
    pub fn unparsed<S: Into<String>>(unparsed: S) -> Self {
        Href::Unparsed(unparsed.into())
    }
    pub fn parse<'a>(unparsed: &'a str) -> crate::Result<Href> {
        if let Ok(p) = files::Path::parse(unparsed) {
            return Ok(Href::LocalFile(p));
        }

        if let Ok(url) = url::Url::parse(&unparsed) {
            return Ok(Href::Url(url));
        }

        Err(Box::new(HrefError::Unsupported(unparsed.to_owned())))
    }
}

impl Display for Href {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Href::Url(url) => write!(f, "{url}"),
            Href::LocalFile(path) => write!(f, "{path}"),
            Href::Unparsed(raw) => write!(f, "{raw}"),
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum HrefError {
    Unsupported(String),
}

impl Display for HrefError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HrefError::")?;
        match self {
            HrefError::Unsupported(href) => write!(f, "Unsupported({href})"),
        }
    }
}

impl std::error::Error for HrefError {}

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
pub struct HrefDefinition {
    label: String,
    href_: Href,
}

impl HrefDefinition {
    pub fn create<H: Into<Href>>(label: String, href: H) -> Self {
        Self {
            label,
            href_: href.into(),
        }
    }

    pub fn href(&self) -> &Href {
        &self.href_
    }
}

#[derive(Debug)]
pub struct HrefReference {
    content: Group,
    label: String,
}

impl HrefReference {
    pub fn create(label: String, content: Group) -> Self {
        Self { content, label }
    }

    pub fn children(&self) -> &Group {
        &self.content
    }
}

#[derive(Debug)]
pub struct ImageReference {
    href_label: String,
    #[allow(unused)]
    alt: String,
}

impl ImageReference {
    pub fn create(href: String, alt: String) -> ImageReference {
        Self {
            href_label: href,
            alt,
        }
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

impl From<String> for FootnoteReference {
    fn from(value: String) -> Self {
        FootnoteReference(value)
    }
}

impl From<FootnoteReference> for Element {
    fn from(value: FootnoteReference) -> Self {
        Element::FootnoteReference(value)
    }
}

#[derive(Debug)]
pub struct FootnoteDefinition {
    label: String,
    content: Group,
}

impl FootnoteDefinition {
    pub fn create<G: Into<Group>>(id: String, content: G) -> Self {
        Self {
            label: id,
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
        self.label.clone()
    }
}
impl Definition for HrefDefinition {
    fn label(&self) -> String {
        self.label.clone()
    }
}

pub trait Reference {
    fn label(&self) -> &String;
}

impl Reference for &HrefReference {
    fn label(&self) -> &String {
        &self.label
    }
}

impl Reference for &ImageReference {
    fn label(&self) -> &String {
        &self.href_label
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
