use std::{io::Write, path};
use url::Url;

use crate::{files, site};

pub struct Tar<W: Write> {
    archive: tar::Builder<W>,
}

impl<W: Write> site::Writer for Tar<W> {
    fn write(&mut self, site: site::RenderedSite) -> crate::Result<()> {
        use site::Writable::*;

        for (url, writable) in site.entries() {
            println!("Writing {}", url);
            match writable {
                Page(page) => self.add_rendered_page(url, page)?,
                Asset(asset) => self.add_static_asset(url, asset)?,
            }
        }

        Ok(())
    }
}

impl<W: Write> Tar<W> {
    pub fn new(w: W) -> Self {
        Self {
            archive: tar::Builder::new(w),
        }
    }

    pub fn finish(self) -> crate::Result<W> {
        Ok(self.archive.into_inner()?)
    }

    fn add_rendered_page(&mut self, url: Url, page: site::RenderedPage) -> crate::Result<()> {
        let mut header = tar::Header::new_gnu();
        header.set_size(page.size());
        header.set_mode(292);
        self.archive
            .append_data(&mut header, Self::to_path(url), page.read())?;
        Ok(())
    }

    fn add_static_asset(&mut self, url: Url, asset: site::IncludedAsset) -> crate::Result<()> {
        match asset.path() {
            // append_path_with_name is (src, dest)
            files::Path::File(f) => self.archive.append_path_with_name(f, Self::to_path(url))?,
            // append_dir_all is (dest, src)
            files::Path::Dir(d) => self.archive.append_dir_all(Self::to_path(url), d)?,
        }
        Ok(())
    }

    fn to_path(url: Url) -> path::PathBuf {
        let path = url.path().strip_prefix('/').unwrap();
        println!("The file path might be: {:?}", path);
        let path = path::Path::new(".").join(path);
        println!("Creating archive path: {}", path.to_string_lossy());
        path
    }
}
