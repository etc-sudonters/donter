#![allow(unused)]
mod cli;
mod config;
mod content;
mod files;
mod jinja;
mod md;
mod processors;
mod render;
mod site;
mod writers;

use std::error::Error;

use clap::Parser;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let conf = cli::Args::parse().make_config();

    let mut app = site::Builder::new()
        .with_when(conf.output.clean, || {
            processors::Cleaner::new(conf.output.output.clone())
        })
        .with(jinja::Jinja::new(conf.site.templates.clone()))
        .with(md::Md)
        .with(processors::Linker::new(processors::LinkerOptions {
            content_base: conf.content.base.clone(),
            site_base: conf.site.base_url.clone(),
            slug_style: conf.output.slug_style,
            article_prefix: conf.output.article_prefix.clone(),
        }))
        .create()?;

    let mut corpus = content::Corpus::default();
    app.load(&conf.content.base(), &mut corpus)?;
    app.process(&mut corpus)?;
    let mut writer = conf.output.writer()?;
    writer.write(app.render_corpus(corpus)?)?;
    writer.flush()?;
    println!("all done!");
    Ok(())
}
