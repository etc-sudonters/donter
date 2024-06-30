use std::{fmt::Display, io::Read, mem};

use minijinja;

use crate::{files, site};

pub struct Jinja {
    template_path: files::DirPath,
}

impl Jinja {
    pub fn new(template_path: files::DirPath) -> Self {
        Self { template_path }
    }
}

impl site::Processor for Jinja {
    fn initialize<'call, 'init>(
        &'call mut self,
        site: &'call mut site::Initializer<'init, '_>,
    ) -> crate::Result<()>
    where
        'init: 'call,
    {
        site.configure_renderer(|renderer| renderer.add_template_dir(&self.template_path))
    }
}

pub struct Builder<'builder, 'env>(&'builder mut minijinja::Environment<'env>)
where
    'env: 'builder;

impl<'builder, 'env> Builder<'builder, 'env> {
    pub fn new(renderer: &'builder mut minijinja::Environment<'env>) -> Self {
        Self(renderer)
    }

    pub fn add_template_file(&mut self, path: files::FilePath) -> crate::Result<()> {
        let tplname = {
            // um, maybe we should help
            let name = path.as_path().file_name().unwrap();
            name.to_str().unwrap().to_owned()
        };

        let mut fh = std::fs::File::open(path)?;
        let mut buf = String::new();
        fh.read_to_string(&mut buf)?;
        self.0.add_template_owned(tplname, buf)?;
        Ok(())
    }

    pub fn add_template_dir(&mut self, dir: &files::DirPath) -> crate::Result<()> {
        for path in files::Walker::walk(&dir, files::RecursionBehavior::Dont) {
            if matches!(path.as_path().extension(), Some(ext) if ext == "html") {
                self.add_template_file(path)?;
            }
        }

        Ok(())
    }

    pub fn configure<F>(&mut self, f: F) -> crate::Result<()>
    where
        F: FnOnce(&mut minijinja::Environment<'env>) -> crate::Result<()>,
    {
        f(self.0)
    }
}

pub struct RenderContext {
    ctx: minijinja::Value,
}

impl RenderContext {
    pub fn add<S: serde::ser::Serialize>(&mut self, key: String, value: S) -> &Self {
        self.ctx = minijinja::context!( key => value, ..self.take());
        self
    }

    pub fn merge(&mut self, merge: minijinja::Value) -> &Self {
        self.ctx = minijinja::context!(..merge, ..self.take());
        self
    }

    fn take(&mut self) -> minijinja::Value {
        let mut ctx = Default::default();
        mem::swap(&mut self.ctx, &mut ctx);
        ctx
    }
}

impl RenderContext {
    pub fn new(ctx: minijinja::Value) -> Self {
        Self { ctx }
    }
}

impl serde::ser::Serialize for RenderContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.ctx.serialize(serializer)
    }
}

#[derive(Debug)]
pub enum Error {
    Render(minijinja::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "jinja::Error::")?;
        match self {
            Self::Render(j) => write!(f, "Render({})", j),
        }
    }
}

impl std::error::Error for Error {}
