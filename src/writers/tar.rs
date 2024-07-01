use super::url_to_relative_path;
use std::io::Write;
use url::Url;

use crate::{files, site};

pub struct Tar<W: Write> {
    archive: tar::Builder<W>,
}

impl<W: Write> site::Writer for Tar<W> {
    fn write_rendered_page(&mut self, url: Url, page: site::RenderedPage) -> crate::Result<()> {
        let mut header = tar::Header::new_gnu();
        header.set_size(page.size());
        header.set_mode(420); // 644
        self.archive
            .append_data(&mut header, url_to_relative_path(url), page.read())?;
        Ok(())
    }

    fn write_static_asset(&mut self, url: Url, asset: site::IncludedAsset) -> crate::Result<()> {
        match asset.path() {
            // append_path_with_name is (src, dest)
            files::Path::File(f) => self
                .archive
                .append_path_with_name(f, url_to_relative_path(url))?,
            // append_dir_all is (dest, src)
            files::Path::Dir(d) => self.archive.append_dir_all(url_to_relative_path(url), d)?,
        }
        Ok(())
    }

    fn flush(self: Box<Self>) -> crate::Result<()> {
        let mut file = self.archive.into_inner()?;
        file.flush()?;
        Ok(())
    }
}

impl<W: Write> Tar<W> {
    pub fn new(w: W) -> Self {
        Self {
            archive: tar::Builder::new(w),
        }
    }
}
