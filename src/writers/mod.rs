mod files;
mod tar;

use std::path;

pub use files::Files;
pub use tar::Tar;
use url::Url;

fn url_to_relative_path(url: Url) -> path::PathBuf {
    let path = url.path().strip_prefix('/').unwrap();
    path::Path::new(".").join(path)
}
