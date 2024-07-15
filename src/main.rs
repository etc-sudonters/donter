#![allow(unused)]
mod cli;
mod config;
mod content;
mod files;
mod ids;
mod jinja;
mod md;
mod processors;
mod render;
mod site;
mod writers;

use std::borrow::Cow;
use std::error::Error;

use clap::Parser;
use content::Origin;
use files::FilePath;
use processors::{Archive, DateArchivist, TagArchivist, TagSorting};
use site::PageTemplate;
use site::RenderedPageMetadata;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut conf = cli::Args::parse().make_config();
    let mut app = site::Builder::new()
        .linker(site::LinkerOptions {
            site_base: Cow::Borrowed(&conf.site.base_url),
            slug_style: conf.rendering.slug_style,
            page_root: conf.rendering.page_root.take(),
            slug_source: site::ArticleSlugSource::Filename,
        })
        .with(processors::Tags)
        .with_when(conf.output.clean, || {
            processors::Cleaner(conf.output.output.clone())
        })
        .with(jinja::JinjaConfiguration(&conf.site.templates))
        .with(md::Md)
        .with(Archive::new(
            TagArchivist(TagSorting::Alphabetical),
            PageTemplate {
                title: "Tag Archive",
                url: unsafe { files::FilePath::new("tags.html") },
                template: "tags.html",
            },
        ))
        .create()?;

    let mut corpus = content::Corpus::create(1312);
    app.load(&conf.content.base(), &mut corpus)?;
    app.process(&mut corpus)?;
    let mut writer = conf.output.writer()?;
    writer.write(app.render(&corpus)?)?;
    app.finalize()?;
    writer.flush()?;
    println!("all done!");
    Ok(())
}
