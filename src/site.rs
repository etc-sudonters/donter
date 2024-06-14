use crate::{config, content::Corpus, jinja, linker, Res};

pub struct Site {
    pre: Vec<Box<dyn Preprocessor>>,
    pro: Vec<Box<dyn Processor>>,
    post: Vec<Box<dyn Postprocessor>>,
    rend: Box<dyn Renderer>,
    link: Box<dyn Linker>,
}

#[derive(Default)]
pub struct Builder {
    pre: Vec<Box<dyn Preprocessor>>,
    pro: Vec<Box<dyn Processor>>,
    post: Vec<Box<dyn Postprocessor>>,
}

impl Builder {
    pub fn construct(self) -> Site {
        Site {
            pre: self.pre,
            pro: self.pro,
            post: self.post,
            link: Box::new(linker::default()),
            rend: Box::new(jinja::Renderer::default()),
        }
    }

    pub fn preprocessor<P: Preprocessor>(mut self, p: P) -> Self {
        self
    }

    pub fn processor<P: Processor>(mut self, p: P) -> Self {
        self
    }

    pub fn postprocessor<P: Postprocessor>(mut self, p: P) -> Self {
        self
    }

    pub fn configure_renderer<R, F>(mut self, c: F) -> Self
    where
        R: Renderer,
        F: FnMut(R) -> (),
    {
        self
    }
}

impl From<config::Site> for Builder {
    fn from(value: config::Site) -> Builder {
        Builder::default()
    }
}

impl Site {
    pub fn preprocessors(&mut self) -> &mut Vec<Box<dyn Preprocessor>> {
        &mut self.pre
    }
    pub fn processors(&mut self) -> &mut Vec<Box<dyn Processor>> {
        &mut self.pro
    }
    pub fn postprocessors(&mut self) -> &mut Vec<Box<dyn Postprocessor>> {
        &mut self.post
    }
    pub fn linker(&mut self) -> &mut Box<dyn Linker> {
        &mut self.link
    }
    pub fn renderer(&mut self) -> &mut Box<dyn Renderer> {
        &mut self.rend
    }
}

pub trait Preprocessor {
    fn run(&mut self, corpus: &mut Corpus) -> Res<()>;
}
pub trait Processor {
    fn run(&mut self, corpus: &mut Corpus) -> Res<()>;
}
pub trait Postprocessor {
    fn run(&mut self, corpus: &mut Corpus) -> Res<()>;
}
pub trait Linker {
    fn link(&mut self, corpus: &mut Corpus) -> Res<()>;
}
pub trait Renderer {
    fn render(&mut self, corpus: Corpus) -> Res<RenderedSite>;
}

pub struct RenderedSite {}
