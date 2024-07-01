mod cli;
mod config;
mod content;
mod doctree;
mod files;
mod jinja;
mod md;
mod processors;
mod site;
mod writers;

use std::error::Error;

use clap::Parser;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let conf = cli::Args::parse().make_config();

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
    let mut writer = conf.output.writer()?;
    writer.write(app.render_corpus(corpus)?)?;
    writer.flush()?;
    println!("all done!");
    Ok(())
}
