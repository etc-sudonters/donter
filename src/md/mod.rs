use crate::site::Loader;
pub mod loader;
mod walker;

pub fn default() -> Vec<Box<dyn Loader>> {
    vec![Box::new(loader::Loader::default())]
}

#[derive(Debug)]
pub enum Error {
    Unexpected(String),
    ParseError(markdown::message::Message),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WalkerError::")?;
        match self {
            Self::Unexpected(msg) => write!(f, "Unexpected({})", msg),
            Self::ParseError(msg) => write!(f, "MarkdownParseError({})", msg),
        }
    }
}

impl<T> Into<crate::Result<T>> for Error {
    fn into(self) -> crate::Result<T> {
        Err(Box::new(self))
    }
}

impl std::error::Error for Error {}
