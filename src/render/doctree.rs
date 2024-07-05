use super::{CodeHighlighter, DisplayableOption, NullHighligher};
use crate::{
    content,
    content::doctree::{self, Definition, DefinitionLookup},
};

struct PageBuffer {
    buffer: String,
    indent_level: usize,
}

impl PageBuffer {
    pub fn new() -> PageBuffer {
        PageBuffer {
            buffer: String::new(),
            indent_level: 0,
        }
    }

    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }

    pub fn push<A: AsRef<str>>(&mut self, piece: A) {
        self.buffer.push_str(piece.as_ref());
    }

    pub fn push_line<A: AsRef<str>>(&mut self, piece: A) {
        self.push(format!(
            "{}{}\n",
            "  ".repeat(self.indent_level).as_str(),
            piece.as_ref()
        ));
    }

    pub fn newline(&mut self) {
        if !self.buffer.ends_with('\n') {
            self.buffer.push('\n');
        }
    }

    pub fn flush(self) -> String {
        self.buffer
    }
}

pub fn render_page(page: &content::PageContents) -> String {
    let mut buffer = PageBuffer::new();
    let helper = DoctreeRenderer {
        page: &page,
        highlighter: Box::new(NullHighligher),
    };
    buffer.push_line("<article>");
    buffer.indent();
    helper.render(&mut buffer);
    buffer.push_line("</article>");
    buffer.flush()
}

struct DoctreeRenderer<'a> {
    page: &'a content::PageContents,
    highlighter: Box<dyn CodeHighlighter>,
}

impl<'a> DoctreeRenderer<'a> {
    pub fn render(&self, buffer: &mut PageBuffer) {
        self.render_elms(&self.page.content, buffer);
        self.include_footnotes(buffer);
    }

    fn include_footnotes(&self, buffer: &mut PageBuffer) {
        buffer.push_line("<div class=\"footnotes\">");
        buffer.indent();
        buffer.push_line("<ol>");

        for ftn in self.page.footnotes.definitions() {
            self.wrap_children_block(
                format!("<li id=\"{}\">", ftn.label()),
                ftn.children(),
                "</li>",
                buffer,
            );
        }

        buffer.push_line("</ol>");
        buffer.dedent();
        buffer.push_line("</div>");
    }

    fn render_elms(&self, elms: &Vec<doctree::Element>, buffer: &mut PageBuffer) {
        for elm in elms.iter() {
            self.render_elm(elm, buffer);
        }
    }

    fn render_elm(&self, elm: &doctree::Element, buffer: &mut PageBuffer) {
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

    fn wrap_children_inline<A, B>(
        &self,
        open: A,
        children: &doctree::Group,
        close: B,
        buffer: &mut PageBuffer,
    ) where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        buffer.push(open);
        self.render_elms(children.children(), buffer);
        buffer.push(close);
    }

    fn wrap_children_block<A, B>(
        &self,
        open: A,
        children: &doctree::Group,
        close: B,
        buffer: &mut PageBuffer,
    ) where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        buffer.push_line(open);
        buffer.indent();
        self.render_elms(children.children(), buffer);
        buffer.dedent();
        buffer.push_line(close);
    }

    fn blockquote(&self, q: &doctree::Group, buffer: &mut PageBuffer) {
        self.wrap_children_block("<blockquote>", q, "</blockquote>", buffer);
    }

    fn codeblock(&self, c: &doctree::Code, buffer: &mut PageBuffer) {
        buffer.push_line(format!(
            "<div class=\"codeblock {}\"><pre><code>",
            DisplayableOption {
                value: c.lang(),
                or: ""
            }
        ));
        buffer.push_line(self.highlighter.highlight(c).as_str());
        buffer.push_line("</code></pre></div>");
    }

    fn delete(&self, d: &doctree::Group, buffer: &mut PageBuffer) {
        self.wrap_children_inline("<s>", d, "</s>", buffer);
    }

    fn emphasis(&self, d: &doctree::Group, buffer: &mut PageBuffer) {
        self.wrap_children_inline("<em>", d, "</em>", buffer);
    }

    fn footnote_reference(&self, f: &doctree::FootnoteReference, buffer: &mut PageBuffer) {
        buffer.push(format!(
            "<span class=\"footnote reference\"><a href=\"#{0}\">{0}</a></span>",
            f
        ));
    }

    fn heading(&self, d: &doctree::Header, buffer: &mut PageBuffer) {
        buffer.push_line(format!(
            "<h{0} id=\"{1}\"><span>{2}</span></h{0}>",
            d.depth(),
            d.label(),
            d.text()
        ));
    }

    fn href_reference(&self, d: &doctree::HrefReference, buffer: &mut PageBuffer) {
        let def = self.page.hrefs.lookup(d).unwrap();
        self.wrap_children_inline(
            format!("<a href=\"{}\">", def.href()),
            d.children(),
            "</a>",
            buffer,
        );
    }
    fn image_reference(&self, d: &doctree::ImageReference, buffer: &mut PageBuffer) {
        let def = self.page.hrefs.lookup(d).unwrap();
        buffer.push(format!("<img href=\"{}\" />", def.href()));
    }

    fn inline_code(&self, d: &doctree::Code, buffer: &mut PageBuffer) {
        buffer.push(format!("<span class=\"code inline\"><code>"));
        buffer.push(d.content());
        buffer.push("</code></span>");
    }

    fn list(&self, d: &doctree::List, buffer: &mut PageBuffer) {
        buffer.push_line("<ul>");
        for item in d.items() {
            self.wrap_children_block("<li>", item.children(), "</li>", buffer);
        }
        buffer.push_line("</ul>");
    }

    fn paragraph(&self, d: &doctree::Group, buffer: &mut PageBuffer) {
        self.wrap_children_block("<p>", d, "</p>", buffer)
    }

    fn strong(&self, d: &doctree::Group, buffer: &mut PageBuffer) {
        self.wrap_children_inline("<strong>", d, "</strong>", buffer)
    }

    fn table(&self, d: &doctree::Table, buffer: &mut PageBuffer) {
        buffer.push_line("<table>");

        for row in d.rows() {
            buffer.push_line("<tr>");

            for cell in row.cells() {
                self.wrap_children_block("<td>", cell.children(), "</td>", buffer);
            }

            buffer.push_line("</tr>");
        }

        buffer.push_line("</table>");
    }

    fn text(&self, d: &doctree::Text, buffer: &mut PageBuffer) {
        buffer.push(d);
    }
}
