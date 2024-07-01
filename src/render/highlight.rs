use crate::doctree;

pub trait CodeHighlighter {
    fn highlight(&self, code: &doctree::Code) -> String;
}

pub struct NullHighligher;

impl CodeHighlighter for NullHighligher {
    fn highlight(&self, code: &doctree::Code) -> String {
        let mut buffer = String::new();
        for line in code.content().lines() {
            buffer.push_str(format!("<span>{}\n</span>", line).as_str());
        }
        buffer
    }
}
