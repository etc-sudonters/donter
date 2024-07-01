mod doctree;
mod highlight;
use std::fmt::Display;

use crate::site;
use doctree::render_page;
pub use highlight::{CodeHighlighter, NullHighligher};

pub struct DefaultFunctions;
impl site::Processor for DefaultFunctions {
    fn initialize<'call, 'init>(
        &'call mut self,
        site: &'call mut site::Initializer<'init, '_>,
    ) -> crate::Result<()>
    where
        'init: 'call,
    {
        site.configure_renderer(|r| {
            r.configure(|env| {
                env.add_function("render_page", render_page);
                Ok(())
            })
        })
    }
}

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
