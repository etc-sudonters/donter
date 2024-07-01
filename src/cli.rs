use clap::Parser;
use url::Url;

use crate::{config, files};

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
}

impl Args {
    pub fn make_config(self) -> config::Configuration {
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
            },
        }
    }
}
