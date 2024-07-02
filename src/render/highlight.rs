use crate::content::doctree;

pub trait CodeHighlighter {
    fn highlight(&self, code: &doctree::Code) -> String;
}

pub struct NullHighligher;

impl CodeHighlighter for NullHighligher {
    fn highlight(&self, code: &doctree::Code) -> String {
        let mut buffer = String::new();
        for line in code.content().lines() {
            buffer.push_str(format!("<span>{}</span>\n", line).as_str());
        }
        buffer
    }
}
