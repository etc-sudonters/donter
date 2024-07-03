use clap::Parser;
use url::Url;

use crate::{config, files, processors::ArticleSlugStyle};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'C', value_name = "CONTENT")]
    content_path: std::path::PathBuf,
    #[arg(short = 'O', value_name = "OUTPUT")]
    output: std::path::PathBuf,
    #[arg(short = 'U', value_name = "URL")]
    url_base: String,
    #[arg(short = 'T', value_name = "TEMPLATES")]
    template_path: std::path::PathBuf,
    #[arg(short = 'P', value_name = "PATH")]
    article_prefix: Option<String>,
    #[arg(short = 'D', default_value_t = false)]
    write_directories: bool,
}

impl Args {
    pub fn make_config(mut self) -> config::Configuration {
        config::Configuration {
            content: config::Content {
                base: unsafe { files::DirPath::new(self.content_path) },
            },
            site: config::Site {
                templates: unsafe { files::DirPath::new(self.template_path) },
                base_url: Url::parse(&self.url_base).expect("invalid base url"),
            },
            output: config::Output {
                output: match self.output.extension().map(|ext| ext.to_str().unwrap()) {
                    Some(ext) if ext == "gz" || ext == "tar" => {
                        files::Path::File(unsafe { files::FilePath::new(self.output) })
                    }
                    _ => files::Path::Dir(unsafe { files::DirPath::new(self.output) }),
                },
                slug_style: if self.write_directories {
                    ArticleSlugStyle::Directory
                } else {
                    ArticleSlugStyle::Page
                },
                article_prefix: self.article_prefix.take(),
            },
        }
    }
}
