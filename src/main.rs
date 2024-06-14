#![allow(dead_code)]
mod config;
mod content;
mod doctree;
mod files;
mod jinja;
mod linker;
mod loaders;
mod output;
mod processors;
mod site;

use std::error::Error;

type Res<T> = Result<T, Box<dyn Error>>;

fn main() -> Res<()> {
    use std::env;

    let mut conf = config::load(env::args())?;
    let mut corpus = content::default();
    let mut loaders = loaders::default();

    let mut page_builder = content::PageBuilder::new();

    for path in files::Walker::from(conf.content().base()).into_iter() {
        for l in loaders.iter_mut() {
            if l.accept(&path)? {
                page_builder.clear();
                l.load(&path, &mut corpus, &mut page_builder)?;
            }
        }
    }

    println!("{:?}", corpus);

    Ok(())
}
