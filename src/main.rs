#![allow(dead_code, unused_variables, unused_mut)]
mod config;
mod content;
mod doctree;
mod files;
mod jinja;
mod linker;
mod md;
mod site;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    use std::env;

    let mut conf = config::load(env::args())?;
    let mut corpus = content::default();
    let mut loaders = md::default();

    for path in files::Walker::from(conf.content().base()).into_iter() {
        for l in loaders.iter_mut() {
            if l.accept(&path)? {
                let mut builder = content::PageBuilder::new();
                builder.path(&path);
                let page = l
                    .load(
                        files::NamedReader::create(
                            path.clone(),
                            Box::new(std::fs::File::open(&path)?),
                        ),
                        builder,
                    )
                    .map_err(|e| content::Error::PageLoad(path.clone(), e))?;
                corpus.add_page(page);
                break;
            }
        }
    }

    println!("{:#?}", corpus);

    Ok(())
}
