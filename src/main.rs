#![allow(dead_code, unused_variables, unused_mut)]
mod config;
mod content;
mod doctree;
mod files;
mod jinja;
mod linker;
mod md;
mod processors;
mod site;
mod urls;

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    use std::env;

    let mut conf = config::load(env::args())?;
    let mut corpus = content::default();
    let mut loaders = md::default();

    for path in files::Walker::from(&conf.content.base).into_iter() {
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

    let mut render_builder = jinja::Builder::new();
    render_builder.add_template_dir(conf.site.templates)?;
    let mut renderer: minijinja::Environment<'_> = render_builder.into();

    for entry in corpus.entries() {
        match entry {
            content::CorpusEntry::Page(p) => {
                println!("Rendering {}", p.meta.path);
                match renderer.get_template("page.html") {
                    Ok(tpl) => {
                        let ctx = minijinja::context! {
                          page => minijinja::context! {
                            filename => String::from(p.meta.path),
                            footnotes => p.content.footnotes,
                          }
                        };
                        match tpl.render(ctx) {
                            Ok(rendered) => {
                                println!("Rendered: {}", rendered);
                            }
                            Err(e) => {
                                println!("Failed to render template 'page.html': {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to get template 'page.html': {}", e);
                    }
                }
            }
            _ => {}
        };
    }

    //println!("{:#?}", corpus);

    println!("all done!");
    Ok(())
}
