#![allow(dead_code, unused_variables, unused_mut)]
mod config;
mod content;
mod doctree;
mod files;
mod jinja;
mod md;
mod processors;
mod site;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    use std::env;

    let mut conf = config::load(env::args())?;
    let mut app = site::SiteBuilder::new()
        .with(jinja::Jinja::new(conf.site.templates.clone()))
        .with(md::Md)
        .with(processors::Linker::new())
        .create()?;

    let mut corpus = content::default();

    app.load(&conf.content.base(), &mut corpus)?;
    app.process(&mut corpus)?;
    let mut site = app.render_corpus(corpus)?;

    /*
    let mut writer: Box<dyn site::Writer> = todo!();
    writer.write(site)?;
    */

    println!("all done!");
    Ok(())
}
