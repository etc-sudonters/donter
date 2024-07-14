use std::{cell::RefCell, io::Read, mem, rc::Rc};

use minijinja;

use crate::{files, site};

pub struct JinjaConfiguration<'a>(pub &'a files::DirPath);

impl<'a> site::Processor for JinjaConfiguration<'a> {
    fn initialize<'call, 'init>(
        &'call mut self,
        site: &'call mut site::Initializer<'init, '_>,
    ) -> crate::Result<()>
    where
        'init: 'call,
    {
        site.configure_renderer(|renderer| renderer.add_template_dir(self.0))
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

pub struct Renderer<'rendering, 'env>
where
    'env: 'rendering,
{
    r: &'rendering Rc<RefCell<minijinja::Environment<'env>>>,
    globals: RenderContext,
}

impl<'rendering, 'env> Renderer<'rendering, 'env>
where
    'env: 'rendering,
{
    pub fn new(
        r: &'rendering Rc<RefCell<minijinja::Environment<'env>>>,
        globals: RenderContext,
    ) -> Self {
        Self { r, globals }
    }

    pub fn render_template(
        &mut self,
        template: &str,
        mut values: RenderContext,
    ) -> crate::Result<String> {
        let eng = self.r.borrow_mut();
        let tpl = eng.get_template(template)?;
        values.merge(minijinja::context! { globals => &self.globals });
        Ok(tpl.render(values)?)
    }

    pub fn values<F>(&mut self, f: F) -> crate::Result<()>
    where
        F: FnOnce(&mut RenderContext) -> crate::Result<()>,
    {
        f(&mut self.globals)
    }
}

pub struct RenderContext {
    ctx: minijinja::Value,
}

impl RenderContext {
    pub fn new(ctx: minijinja::Value) -> Self {
        Self { ctx }
    }

    pub fn empty() -> Self {
        Self::new(minijinja::Value::default())
    }

    pub fn merge(&mut self, merge: minijinja::Value) {
        self.ctx = minijinja::context!(..merge, ..self.take());
    }

    fn take(&mut self) -> minijinja::Value {
        let mut ctx = Default::default();
        mem::swap(&mut self.ctx, &mut ctx);
        ctx
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
