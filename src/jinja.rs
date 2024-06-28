use std::{fmt::Display, io::Read, mem};

use minijinja;

use crate::files::{self, DirPath};

pub struct Builder<'a>(minijinja::Environment<'a>);

impl<'a> From<Builder<'a>> for minijinja::Environment<'a> {
    fn from(value: Builder<'a>) -> Self {
        value.0
    }
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self(minijinja::Environment::<'a>::new())
    }

    pub fn add_template_file(&mut self, path: files::FilePath) -> crate::Result<()> {
        let tplname = {
            // um, maybe we should help
            let name = path.as_ref().file_name().unwrap();
            name.to_str().unwrap().to_owned()
        };

        let mut fh = std::fs::File::open(path)?;
        let mut buf = String::new();
        fh.read_to_string(&mut buf)?;
        self.0.add_template_owned(tplname, buf)?;
        Ok(())
    }

    pub fn add_template_dir(&mut self, dir: DirPath) -> crate::Result<()> {
        for path in files::Walker::from(dir).into_iter() {
            if matches!(path.as_ref().extension(), Some(ext) if ext == "html") {
                self.add_template_file(path)?;
            }
        }

        Ok(())
    }

    pub fn configure<F>(&mut self, f: F) -> crate::Result<()>
    where
        F: FnOnce(&mut minijinja::Environment<'a>) -> crate::Result<()>,
    {
        f(&mut self.0)
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

pub trait Renderer {
    fn render(&mut self, tpl_name: &'_ str, ctx: RenderContext) -> crate::Result<String>;
}

pub fn load_templates<'a>(
    tpl_path: files::DirPath,
    env: &mut minijinja::Environment<'a>,
) -> crate::Result<()> {
    use std::io::Read;
    for path in files::Walker::new(tpl_path, files::RecursionBehavior::Dont).into_iter() {
        if matches!(path.as_ref().extension(), Some(ext) if ext == "html") {
            let mut buf = String::new();
            let mut fh = std::fs::File::open(&path)?;
            fh.read_to_string(&mut buf)?;

            let tplname = {
                // um, maybe we should help
                let name = path.as_ref().file_name().unwrap();
                name.to_str().unwrap().to_owned()
            };

            env.add_template_owned(tplname, buf)?;
        }
    }
    Ok(())
}

pub struct Jinja<'a> {
    jinja: minijinja::Environment<'a>,
}

impl<'a> Jinja<'a> {
    pub fn new(jinja: minijinja::Environment<'a>) -> Jinja<'a> {
        Self { jinja }
    }
}

impl<'a> From<minijinja::Environment<'a>> for Jinja<'a> {
    fn from(value: minijinja::Environment<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> Renderer for Jinja<'a> {
    fn render(&mut self, tpl_name: &'_ str, ctx: RenderContext) -> crate::Result<String> {
        let tpl = self.jinja.get_template(tpl_name)?;
        // what
        Ok(tpl
            .render(ctx)
            .map_err(|err| Box::new(Error::Render(err)))?)
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
