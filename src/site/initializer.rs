use crate::jinja;
use crate::Result;

use super::Loader;

pub struct Initializer<'builder, 'env> {
    pub(crate) loaders: &'builder mut Vec<Box<dyn Loader>>,
    pub(crate) renderer: jinja::Builder<'builder, 'env>,
}

impl<'builder, 'env> Initializer<'builder, 'env> {
    pub fn add_loader(&mut self, loader: Box<dyn Loader>) {
        self.loaders.push(loader)
    }

    pub fn configure_renderer<'a, F>(&'a mut self, configure: F) -> crate::Result<()>
    where
        'builder: 'a,
        F: FnOnce(&'a mut jinja::Builder<'builder, 'env>) -> crate::Result<()>,
    {
        configure(&mut self.renderer)
    }
}
