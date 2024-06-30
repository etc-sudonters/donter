#![allow(dead_code, unused_variables, unused_mut)]
mod config;
mod content;
mod doctree;
mod files;
mod jinja;
mod md;
mod processors;
mod site;
mod writers;

use std::{error::Error, fs::File, io::Write};

use site::Writer;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    use std::env;

    let mut conf = config::load(env::args())?;
    let mut app = site::Builder::new()
        .with(jinja::Jinja::new(conf.site.templates.clone()))
        .with(md::Md)
        .with(processors::Linker::new(processors::LinkerOptions {
            content_base: conf.content.base.clone(),
            site_base: conf.site.base_url.clone(),
        }))
        .create()?;

    let mut corpus = content::default();

    app.load(&conf.content.base(), &mut corpus)?;
    app.process(&mut corpus)?;
    let mut site = app.render_corpus(corpus)?;
    let mut writer = writers::Tar::new(File::create("site.tar")?);
    writer.write(site)?;
    let mut file = writer.finish()?;
    file.flush()?;
    println!("all done!");
    Ok(())
}
