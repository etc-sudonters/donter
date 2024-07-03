use std::io::Write;

use crate::{files, site};

pub struct Tar<W: Write> {
    archive: tar::Builder<W>,
}

impl<W: Write> site::Writer for Tar<W> {
    fn write_rendered_page(
        &mut self,
        path: files::FilePath,
        page: site::RenderedPage,
    ) -> crate::Result<()> {
        let mut header = tar::Header::new_gnu();
        header.set_size(page.size());
        header.set_mode(420); // 644
        self.archive.append_data(&mut header, path, page.read())?;
        Ok(())
    }

    fn write_static_asset(
        &mut self,
        path: files::Path,
        asset: site::IncludedAsset,
    ) -> crate::Result<()> {
        match asset.path() {
            // append_path_with_name is (src, dest)
            files::Path::File(f) => self
                .archive
                .append_path_with_name(f, path.as_file().unwrap())?,
            // append_dir_all is (dest, src)
            files::Path::Dir(d) => self.archive.append_dir_all(path.as_dir().unwrap(), d)?,
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
