use crate::doctree::{self, HrefReference, Text};
use std::fmt::{Debug, Display};

impl TryFrom<&markdown::mdast::Node> for doctree::DocumentPart {
    type Error = ParseError;

    fn try_from(value: &markdown::mdast::Node) -> Result<Self, Self::Error> {
        use markdown::mdast::Node;
        match value {
            Node::FootnoteDefinition(ftn) => Ok(doctree::DocumentPart::Footnote(ftn.try_into()?)),
            Node::Definition(def) => Ok(doctree::DocumentPart::Href(def.into())),
            node @ _ => Ok(doctree::DocumentPart::Element(node.try_into()?)),
        }
    }
}

impl TryFrom<&markdown::mdast::Node> for doctree::Element {
    type Error = ParseError;

    fn try_from(value: &markdown::mdast::Node) -> Result<Self, Self::Error> {
        use doctree::Element;
        use markdown::mdast::Node;
        match value {
            Node::Root(r) => Ok((doctree::Group::try_from(&r.children)?).into()),
            Node::Code(code) => Ok(Element::CodeBlock(code.try_into()?)),
            Node::BlockQuote(r) => Ok(Element::BlockQuote(doctree::Group::try_from(&r.children)?)),
            Node::Break(_) => Ok(Element::Break),
            Node::Emphasis(e) => Ok(Element::Emphasis(doctree::Group::try_from(&e.children)?)),
            Node::FootnoteReference(r) => Ok(Element::FootnoteReference(r.into())),
            Node::Heading(h) => Ok(Element::Heading(doctree::Header::try_from(h)?)),
            Node::Image(i) => Ok(Element::Image(doctree::Image::from(i))),
            Node::ImageReference(r) => Ok(Element::ImageReference(doctree::HrefReference::from(r))),
            Node::InlineCode(code) => Ok(Element::InlineCode(code.into())),
            Node::Link(l) => Ok(Element::Link(doctree::Link::try_from(l)?)),
            Node::LinkReference(r) => {
                Ok(Element::HrefReference(doctree::HrefReference::try_from(r)?))
            }
            Node::Paragraph(p) => Ok(Element::Paragraph(doctree::Group::try_from(&p.children)?)),
            Node::Strong(s) => Ok(Element::Strong(doctree::Group::try_from(&s.children)?)),
            Node::Table(tbl) => Ok(Element::Table(doctree::Table::try_from(tbl)?)),
            Node::Text(t) => Ok(Element::Text(t.into())),
            Node::ThematicBreak(_) => Ok(Element::ThematicBreak),
            Node::List(list) => Ok(Element::List(list.try_into()?)),
            // just don't nest them :shrug:
            Node::Definition(_) | Node::FootnoteDefinition(_) => {
                Err(ParseError::Unsupported("Definition".to_owned()))
            }
            any @ _ => Err(ParseError::Unknown(format!("{:?}", any))),
        }
    }
}

pub enum ParseError {
    Unsupported(String),
    Unknown(String),
}
pub type ParseResult = Result<doctree::DocumentPart, ParseError>;

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError::")?;
        match self {
            Self::Unknown(msg) => write!(f, "Unknown({})", &msg),
            Self::Unsupported(what) => write!(f, "Unsupported({})", &what),
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl std::error::Error for ParseError {}

impl TryFrom<Vec<markdown::mdast::Node>> for doctree::Group {
    type Error = ParseError;

    fn try_from(value: Vec<markdown::mdast::Node>) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Vec<markdown::mdast::Node>> for doctree::Group {
    type Error = ParseError;

    fn try_from(value: &Vec<markdown::mdast::Node>) -> Result<Self, Self::Error> {
        let nodes = value
            .iter()
            .filter_map(|md| doctree::Element::try_from(md).ok())
            .collect();
        return Ok(nodes);
    }
}

impl TryFrom<&markdown::mdast::Code> for doctree::Code {
    type Error = ParseError;

    fn try_from(value: &markdown::mdast::Code) -> Result<Self, Self::Error> {
        // TODO get rid of clones -- mem::replace would be nice if possible
        let content = doctree::CodeLiteral::from(value.value.clone());
        let lang = value.lang.clone().map(|l| doctree::CodeLanguage::from(l));
        let meta = value.meta.clone();
        Ok(doctree::Code::new(content, lang, meta))
    }
}

impl From<&markdown::mdast::InlineCode> for doctree::Code {
    fn from(value: &markdown::mdast::InlineCode) -> Self {
        doctree::Code::new(doctree::CodeLiteral::from(value.value.clone()), None, None)
    }
}

impl TryFrom<&markdown::mdast::Heading> for doctree::Header {
    type Error = ParseError;

    fn try_from(value: &markdown::mdast::Heading) -> Result<Self, Self::Error> {
        Ok(doctree::Header::create(
            value.depth,
            (doctree::Group::try_from(&value.children)?).into(),
        ))
    }
}

impl From<&markdown::mdast::Text> for doctree::Text {
    fn from(value: &markdown::mdast::Text) -> Self {
        doctree::Text::create(value.value.clone())
    }
}

impl TryFrom<&markdown::mdast::Link> for doctree::Link {
    type Error = ParseError;

    fn try_from(value: &markdown::mdast::Link) -> Result<Self, Self::Error> {
        Ok(doctree::Link::create(
            value.url.clone(),
            (doctree::Group::try_from(&value.children)?).into(),
            value.title.clone(),
        ))
    }
}

impl From<&markdown::mdast::Definition> for doctree::HrefDefinition {
    fn from(value: &markdown::mdast::Definition) -> Self {
        Self::create(value.identifier.clone(), value.url.clone())
    }
}

impl TryFrom<&markdown::mdast::LinkReference> for doctree::HrefReference {
    type Error = ParseError;

    fn try_from(value: &markdown::mdast::LinkReference) -> Result<Self, Self::Error> {
        Ok(Self::create(
            value.identifier.clone(),
            (doctree::Group::try_from(&value.children)?).into(),
            value.label.clone(),
        ))
    }
}

impl From<&markdown::mdast::ImageReference> for HrefReference {
    fn from(value: &markdown::mdast::ImageReference) -> Self {
        Self::create(
            value.identifier.clone(),
            doctree::Element::Text(Text::create(value.alt.clone())),
            value.label.clone(),
        )
    }
}

impl From<&markdown::mdast::Image> for doctree::Image {
    fn from(value: &markdown::mdast::Image) -> Self {
        Self::create(value.url.clone(), value.alt.clone())
    }
}

impl TryFrom<&markdown::mdast::Table> for doctree::Table {
    type Error = ParseError;

    fn try_from(value: &markdown::mdast::Table) -> Result<Self, Self::Error> {
        let mut rows = Vec::with_capacity(value.children.len());

        for node in value.children.iter() {
            match node {
                markdown::mdast::Node::TableRow(row) => rows.push(row.try_into()?),
                _ => return Err(ParseError::Unsupported("Not table row".to_owned())),
            }
        }

        Ok(doctree::Table::from_iter(rows))
    }
}

impl TryFrom<&markdown::mdast::TableRow> for doctree::TableRow {
    type Error = ParseError;

    fn try_from(value: &markdown::mdast::TableRow) -> Result<Self, Self::Error> {
        let mut cells = Vec::with_capacity(value.children.len());

        for node in value.children.iter() {
            match node {
                markdown::mdast::Node::TableCell(cell) => {
                    cells.push(cell.try_into()?);
                }
                _ => return Err(ParseError::Unsupported("Not Table Cell".to_owned())),
            }
        }

        Ok(doctree::TableRow::from_iter(cells))
    }
}

impl TryFrom<&markdown::mdast::TableCell> for doctree::TableCell {
    type Error = ParseError;

    fn try_from(value: &markdown::mdast::TableCell) -> Result<Self, Self::Error> {
        Ok(doctree::TableCell::create(
            (doctree::Group::try_from(&value.children)?).into(),
        ))
    }
}

impl From<&markdown::mdast::FootnoteReference> for doctree::FootnoteReference {
    fn from(value: &markdown::mdast::FootnoteReference) -> Self {
        Self::create(value.identifier.clone())
    }
}

impl TryFrom<&markdown::mdast::FootnoteDefinition> for doctree::FootnoteDefinition {
    type Error = ParseError;
    fn try_from(value: &markdown::mdast::FootnoteDefinition) -> Result<Self, Self::Error> {
        Ok(Self::create(
            value.identifier.clone(),
            (doctree::Group::try_from(&value.children)?).into(),
        ))
    }
}

impl TryFrom<&markdown::mdast::List> for doctree::List {
    type Error = ParseError;
    fn try_from(value: &markdown::mdast::List) -> Result<Self, Self::Error> {
        let mut items = doctree::List::default();
        for node in value.children.iter() {
            match node {
                markdown::mdast::Node::ListItem(item) => items.push(item.try_into()?),
                any @ _ => {
                    return Err(ParseError::Unsupported(format!(
                        "Not a list item: {:?}",
                        any
                    )))
                }
            }
        }
        Ok(items)
    }
}

impl TryFrom<&markdown::mdast::ListItem> for doctree::ListItem {
    type Error = ParseError;
    fn try_from(value: &markdown::mdast::ListItem) -> Result<Self, Self::Error> {
        doctree::Group::try_from(&value.children).map(|g| g.into())
    }
}
