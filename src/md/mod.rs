use crate::content::Loader;
pub mod loader;
mod translate;

pub fn default() -> Vec<Box<dyn Loader>> {
    vec![Box::new(loader::Loader::default())]
}
