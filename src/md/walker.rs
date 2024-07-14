use markdown::mdast;

use super::Error;

use crate::{
    content::{
        self,
        doctree::{self, Href},
        Metadata,
    },
    md::frontmatter::frontmatter_to_page_meta,
};

fn slug<S: AsRef<str>>(s: S) -> String {
    s.as_ref().replace(' ', "-").to_lowercase()
}

pub struct MarkdownPageBuilder<'p> {
    groups: Vec<doctree::Group>,
    builder: &'p mut content::PageBuilder,
    opts: &'p markdown::ParseOptions,
}

impl<'p> MarkdownPageBuilder<'p> {
    pub fn new(builder: &'p mut content::PageBuilder, opts: &'p markdown::ParseOptions) -> Self {
        Self {
            groups: Default::default(),
            builder,
            opts,
        }
    }

    pub fn build(mut self, node: &markdown::mdast::Node) -> crate::Result<()> {
        use markdown::mdast::Node;
        match node {
            Node::Root(r) => {
                let grp = self.collect_children(&r.children)?;
                if self.groups.len() != 0 {
                    panic!("Unpopped groups exist");
                }
                self.builder.content(grp.into());
                Ok(())
            }
            any @ _ => Error::Unexpected(format!("Expected Root, got {:?}", any)).into(),
        }
    }

    fn push_group(&mut self) {
        self.groups.push(Default::default())
    }

    fn pop_group(&mut self) -> doctree::Group {
        match self.groups.pop() {
            None => panic!("attempted to pop empty group!"),
            Some(grp) => grp,
        }
    }

    fn push_element(&mut self, elm: doctree::Element) {
        match self.groups.last_mut() {
            None => panic!("attempt to add element to empty group!"),
            Some(grp) => grp.push(elm),
        }
    }

    fn push_footnote(&mut self, id: String, content: doctree::Group) {
        let ftn = doctree::FootnoteDefinition::create(id.clone(), content);
        self.builder.footnotes(move |notes| notes.define(&id, ftn));
    }

    fn push_href(&mut self, id: String, href: String) {
        let href = doctree::HrefDefinition::create(id.clone(), doctree::Href::Unparsed(href));
        self.builder.hrefs(move |notes| notes.define(&id, href));
    }

    fn collect_children(
        &mut self,
        children: &Vec<markdown::mdast::Node>,
    ) -> crate::Result<doctree::Group> {
        self.push_group();
        for node in children.iter() {
            self.walk(node)?;
        }
        Ok(self.pop_group())
    }

    fn walk(&mut self, node: &markdown::mdast::Node) -> crate::Result<()> {
        use markdown::mdast::Node;
        match node {
            Node::Yaml(y) => self.handle_meta(y),
            Node::Root(_) => Error::Unexpected("Unexpected nested root element".to_owned()).into(),
            Node::BlockQuote(quote) => self.blockquote(quote),
            Node::Code(block) => self.codeblock(block),
            Node::Definition(def) => self.href_definition(def),
            Node::Delete(del) => self.delete(del),
            Node::Emphasis(em) => self.emphasis(em),
            Node::FootnoteDefinition(def) => self.footnote_definition(def),
            Node::FootnoteReference(ftn) => self.footnote_reference(ftn),
            Node::Heading(heading) => self.header(heading),
            Node::Image(img) => self.image(img),
            Node::ImageReference(img) => self.image_reference(img),
            Node::InlineCode(code) => self.inline_code(code),
            Node::Link(link) => self.link(link),
            Node::LinkReference(link) => self.link_reference(link),
            Node::List(list) => self.list(list),
            Node::Paragraph(para) => self.paragraph(para),
            Node::Strong(str_) => self.strong(str_),
            Node::Table(tbl) => self.table(tbl),
            Node::Text(txt) => self.text(txt),
            Node::Break(_) | Node::ThematicBreak(_) => Ok(()),
            any @ _ => Error::Unexpected(format!("Unexpected element: {:?}", any)).into(),
        }
    }

    fn handle_meta(&mut self, meta: &mdast::Yaml) -> crate::Result<()> {
        frontmatter_to_page_meta(meta, self.builder)?;

        if let Some(Metadata::Str(s)) = self.builder.meta.remove("summary") {
            let node = markdown::to_mdast(&s, self.opts).map_err(|e| Error::ParseError(e))?;
            self.builder.summary = Some(match node {
                mdast::Node::Root(r) => self.collect_children(&r.children)?,
                node @ _ => {
                    self.push_group();
                    self.walk(&node)?;
                    self.pop_group()
                }
            });
        }
        Ok(())
    }

    fn codeblock(&mut self, code: &markdown::mdast::Code) -> crate::Result<()> {
        // TODO get rid of clones -- mem::replace would be nice if possible
        let content = doctree::CodeLiteral::from(code.value.clone());
        let lang = code.lang.clone().map(|l| doctree::CodeLanguage::from(l));
        self.push_element(doctree::Code::new(content, lang).block());
        Ok(())
    }

    fn blockquote(&mut self, quote: &markdown::mdast::BlockQuote) -> crate::Result<()> {
        let quote = self.collect_children(&quote.children)?;
        self.push_element(doctree::Element::BlockQuote(quote));
        Ok(())
    }

    fn href_definition(&mut self, definition: &markdown::mdast::Definition) -> crate::Result<()> {
        if definition.label.is_none() {
            panic!("Unlabeled href definition: {:#?}", definition);
        }

        self.push_href(definition.label.clone().unwrap(), definition.url.clone());
        Ok(())
    }

    fn emphasis(&mut self, emph: &markdown::mdast::Emphasis) -> crate::Result<()> {
        let emph = self.collect_children(&emph.children)?;
        self.push_element(doctree::Element::Emphasis(emph));
        Ok(())
    }

    fn footnote_definition(
        &mut self,
        def: &markdown::mdast::FootnoteDefinition,
    ) -> crate::Result<()> {
        let content = self.collect_children(&def.children)?;
        self.push_footnote(def.identifier.clone(), content);
        Ok(())
    }

    fn footnote_reference(
        &mut self,
        ftn: &markdown::mdast::FootnoteReference,
    ) -> crate::Result<()> {
        self.builder
            .footnotes(|notes| notes.add_label(&ftn.identifier));
        self.push_element(doctree::Element::FootnoteReference(
            doctree::FootnoteReference::from(ftn.identifier.clone()),
        ));
        Ok(())
    }

    fn header(&mut self, header: &markdown::mdast::Heading) -> crate::Result<()> {
        match doctree::Element::from(self.collect_children(&header.children)?) {
            doctree::Element::Text(txt) => {
                let display = txt.inner();
                let id = slug(&display);
                let header = doctree::Header::create(header.depth, display, id);
                self.push_element(doctree::Element::Heading(header));
                Ok(())
            }
            _ => Err(todo!()),
        }
    }

    fn image(&mut self, img: &markdown::mdast::Image) -> crate::Result<()> {
        let label = get_hex_hash_string(&img.url);
        self.push_href(label.clone(), img.url.clone());
        self.push_element(doctree::Element::ImageReference(
            doctree::ImageReference::create(label, img.alt.clone()),
        ));
        Ok(())
    }

    fn image_reference(&mut self, img: &markdown::mdast::ImageReference) -> crate::Result<()> {
        self.builder.hrefs(|hrefs| hrefs.add_label(&img.identifier));
        self.push_element(doctree::Element::ImageReference(
            doctree::ImageReference::create(img.identifier.clone(), img.alt.clone()),
        ));
        Ok(())
    }

    fn inline_code(&mut self, code: &markdown::mdast::InlineCode) -> crate::Result<()> {
        self.push_element(doctree::Element::InlineCode(doctree::Code::new(
            doctree::CodeLiteral::from(code.value.clone()),
            None,
        )));
        Ok(())
    }

    fn link(&mut self, link: &markdown::mdast::Link) -> crate::Result<()> {
        let label = get_hex_hash_string(&link.url);
        let content = self.collect_children(&link.children)?;
        self.push_element(doctree::Element::HrefReference(
            doctree::HrefReference::create(label.clone(), content.into(), link.title.clone()),
        ));
        self.push_href(label.clone(), link.url.clone());
        Ok(())
    }

    fn link_reference(&mut self, link: &markdown::mdast::LinkReference) -> crate::Result<()> {
        self.builder
            .hrefs(|hrefs| hrefs.add_label(&link.identifier));
        let content = self.collect_children(&link.children)?;
        self.push_element(doctree::Element::HrefReference(
            doctree::HrefReference::create(link.identifier.clone(), content.into(), None),
        ));
        Ok(())
    }

    fn list(&mut self, list: &markdown::mdast::List) -> crate::Result<()> {
        use markdown::mdast::Node;
        let mut lst = doctree::List::default();
        for item in list.children.iter() {
            match item {
                Node::ListItem(it) => {
                    let content = self.collect_children(&it.children)?;
                    lst.push(doctree::ListItem::from(content));
                }
                any @ _ => {
                    return Error::Unexpected(format!("Unexpected element: {:?}", any)).into()
                }
            }
        }
        self.push_element(doctree::Element::List(lst));
        Ok(())
    }

    fn paragraph(&mut self, p: &markdown::mdast::Paragraph) -> crate::Result<()> {
        let content = self.collect_children(&p.children)?;
        self.push_element(doctree::Element::Paragraph(content));
        Ok(())
    }

    fn strong(&mut self, s: &markdown::mdast::Strong) -> crate::Result<()> {
        let content = self.collect_children(&s.children)?;
        self.push_element(doctree::Element::Strong(content));
        Ok(())
    }

    fn table(&mut self, md_table: &markdown::mdast::Table) -> crate::Result<()> {
        use markdown::mdast::Node;
        let mut table = doctree::Table::new();
        for node in md_table.children.iter() {
            match node {
                Node::TableRow(row) => {
                    self.table_row(&mut table, row)?;
                }
                any @ _ => {
                    return Error::Unexpected(format!("Unexpected element: {:?}", any)).into()
                }
            }
        }
        self.push_element(doctree::Element::Table(table));
        Ok(())
    }

    fn table_row(
        &mut self,
        tbl: &mut doctree::Table,
        md_row: &markdown::mdast::TableRow,
    ) -> crate::Result<()> {
        use markdown::mdast::Node;
        let mut row = doctree::TableRow::new();
        for node in md_row.children.iter() {
            match node {
                Node::TableCell(cell) => {
                    self.table_cell(&mut row, cell)?;
                }
                any @ _ => {
                    return Error::Unexpected(format!("Unexpected element: {:?}", any)).into()
                }
            }
        }
        tbl.push(row);
        Ok(())
    }

    fn table_cell(
        &mut self,
        row: &mut doctree::TableRow,
        cell: &markdown::mdast::TableCell,
    ) -> crate::Result<()> {
        let children = self.collect_children(&cell.children)?;
        row.push(doctree::TableCell::create(children.into()));
        Ok(())
    }

    fn text(&mut self, txt: &markdown::mdast::Text) -> crate::Result<()> {
        self.push_element(doctree::Element::Text(doctree::Text::create(
            txt.value.clone(),
        )));
        Ok(())
    }

    fn delete(&mut self, del: &markdown::mdast::Delete) -> crate::Result<()> {
        let content = self.collect_children(&del.children)?;
        self.push_element(doctree::Element::Delete(content));
        Ok(())
    }
}

fn get_hex_hash_string<S: AsRef<str>>(s: S) -> String {
    use std::hash::{DefaultHasher, Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    s.as_ref().hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
