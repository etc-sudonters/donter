use minijinja::value::ViaDeserialize;

use super::{CodeHighlighter, DisplayableOption, NullHighligher};
use crate::{
    content,
    doctree::{self, DefinitionLookup},
};

pub fn render_page(page: ViaDeserialize<content::PageContents>) -> String {
    let mut buffer = String::new();
    let helper = DoctreeRenderer {
        page: &page,
        highlighter: Box::new(NullHighligher),
    };
    helper.render(&mut buffer);
    buffer
}

struct DoctreeRenderer<'a> {
    page: &'a content::PageContents,
    highlighter: Box<dyn CodeHighlighter>,
}

impl<'a> DoctreeRenderer<'a> {
    pub fn render(&self, buffer: &mut String) {
        self.render_elms(&self.page.content, buffer)
    }

    fn render_elms(&self, elms: &Vec<doctree::Element>, buffer: &mut String) {
        for elm in elms.iter() {
            self.render_elm(elm, buffer);
        }
    }

    fn render_elm(&self, elm: &doctree::Element, buffer: &mut String) {
        use doctree::Element::*;
        match elm {
            BlockQuote(q) => self.blockquote(q, buffer),
            CodeBlock(c) => self.codeblock(c, buffer),
            Delete(d) => self.delete(d, buffer),
            Emphasis(e) => self.emphasis(e, buffer),
            Empty => {}
            FootnoteReference(f) => self.footnote_reference(f, buffer),
            Group(g) => self.render_elms(g.children(), buffer),
            Heading(h) => self.heading(h, buffer),
            HrefReference(h) => self.href_reference(h, buffer),
            ImageReference(i) => self.image_reference(i, buffer),
            InlineCode(i) => self.inline_code(i, buffer),
            List(l) => self.list(l, buffer),
            Paragraph(p) => self.paragraph(p, buffer),
            Strong(s) => self.strong(s, buffer),
            Table(t) => self.table(t, buffer),
            Text(t) => self.text(t, buffer),
        }
    }

    fn wrap_children(
        &self,
        open: &str,
        children: &doctree::Group,
        close: &str,
        buffer: &mut String,
    ) {
        buffer.push_str(open);
        self.render_elms(children.children(), buffer);
        buffer.push_str(close);
    }

    fn blockquote(&self, q: &doctree::Group, buffer: &mut String) {
        self.wrap_children("<blockquote>", q, "</blockquote>", buffer);
    }

    fn codeblock(&self, c: &doctree::Code, buffer: &mut String) {
        buffer.push_str(
            format!(
                "<div class=\"codeblock {}\"><pre><code>",
                DisplayableOption {
                    value: c.lang(),
                    or: ""
                }
            )
            .as_str(),
        );

        buffer.push_str(self.highlighter.highlight(c).as_str());
        buffer.push_str("</code></pre></div>");
    }

    fn delete(&self, d: &doctree::Group, buffer: &mut String) {
        self.wrap_children("<s>", d, "</s>", buffer);
    }
    fn emphasis(&self, d: &doctree::Group, buffer: &mut String) {
        self.wrap_children("<em>", d, "</em>", buffer);
    }
    fn footnote_reference(&self, f: &doctree::FootnoteReference, buffer: &mut String) {
        buffer.push_str(
            format!(
                "<span class=\"footnote reference\"><a href=\"#{0}\">{0}</a></span>",
                f
            )
            .as_str(),
        );
    }
    fn heading(&self, d: &doctree::Header, buffer: &mut String) {
        self.wrap_children(
            format!("<h{}>", d.depth()).as_str(),
            d.children(),
            format!("</h{}>", d.depth()).as_str(),
            buffer,
        );
    }
    fn href_reference(&self, d: &doctree::HrefReference, buffer: &mut String) {
        let def = self.page.hrefs.lookup(d).unwrap();
        self.wrap_children(
            format!("<a href=\"{}\">", def.href()).as_str(),
            d.children(),
            "</a>",
            buffer,
        );
    }
    fn image_reference(&self, d: &doctree::ImageReference, buffer: &mut String) {
        let def = self.page.hrefs.lookup(d).unwrap();
        buffer.push_str(format!("<img href=\"{}\" />", def.href()).as_str());
    }
    fn inline_code(&self, d: &doctree::Code, buffer: &mut String) {
        todo!()
    }
    fn list(&self, d: &doctree::List, buffer: &mut String) {
        todo!()
    }
    fn paragraph(&self, d: &doctree::Group, buffer: &mut String) {
        todo!()
    }
    fn strong(&self, d: &doctree::Group, buffer: &mut String) {
        todo!()
    }
    fn table(&self, d: &doctree::Table, buffer: &mut String) {
        todo!()
    }
    fn text(&self, d: &doctree::Text, buffer: &mut String) {
        todo!()
    }
}
