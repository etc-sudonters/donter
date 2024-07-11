use super::App;

use super::Processor;

pub struct Builder<'a>(Vec<Box<dyn Processor + 'a>>);

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn with_when<F, P>(mut self, cond: bool, factory: F) -> Self
    where
        P: Processor + 'a,
        F: FnOnce() -> P,
    {
        if cond {
            self.with(factory())
        } else {
            self
        }
    }

    pub fn with<P: Processor + 'a>(mut self, processor: P) -> Self {
        self.0.push(Box::new(processor));
        self
    }

    pub fn create(mut self) -> crate::Result<App<'a>> {
        App::create(self.0)
    }
}
