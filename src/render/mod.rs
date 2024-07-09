mod doctree;
mod highlight;
use std::fmt::Display;

use crate::site;
pub use doctree::{render_page, render_summary};
pub use highlight::{CodeHighlighter, NullHighligher};

struct DisplayableOption<'a, T>
where
    T: Display,
{
    value: &'a Option<T>,
    or: &'a str,
}

impl<'a, T> Display for DisplayableOption<'a, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Some(t) => Display::fmt(t, f),
            None => write!(f, "{}", self.or),
        }
    }
}
