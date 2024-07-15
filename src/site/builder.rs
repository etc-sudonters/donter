use super::App;

use super::Processor;

pub struct Builder<'a> {
    pub(crate) processors: Vec<Box<dyn Processor + 'a>>,
    pub(crate) linker_opts: super::LinkerOptions<'a>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self {
            processors: vec![],
            linker_opts: Default::default(),
        }
    }

    pub fn linker(mut self, opts: super::LinkerOptions<'a>) -> Self {
        self.linker_opts = opts;
        self
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
        self.processors.push(Box::new(processor));
        self
    }

    pub fn create(mut self) -> crate::Result<App<'a>> {
        App::create(self)
    }
}
