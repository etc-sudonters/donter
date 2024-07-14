use std::{
    fs::{self, File},
    io, path,
};

use crate::{files, site};

pub struct Files(files::DirPath);

impl Files {
    pub fn create(dir: files::DirPath) -> crate::Result<Files> {
        fs::create_dir_all(&dir)?;
        Ok(Self(dir))
    }

    fn create_path<P: AsRef<path::Path>>(&self, path: P) -> crate::Result<path::PathBuf> {
        let mut path = self.0.join(path);
        match path.extension() {
            None => fs::create_dir_all(&path)?,
            Some(_) => {
                let filename = path.file_name().unwrap().to_os_string();
                path.pop();
                fs::create_dir_all(&path)?;
                path.push(filename);
            }
        }

        Ok(path)
    }
}

impl site::Writer for Files {
    fn write_static_asset(&mut self, asset: site::IncludedAsset) -> crate::Result<()> {
        use files::Path::*;
        let dest = self.create_path(asset.destination())?;
        match asset.source() {
            File(f) => {
                fs::copy(f, dest)?;
            }
            Dir(d) => {
                for path in files::Walker::walk(&d, files::RecursionBehavior::Recurse) {
                    let name = path.file_name().unwrap().to_os_string();
                    fs::copy(path, dest.join(name))?;
                }
            }
        }

        Ok(())
    }

    fn write_rendered_page(&mut self, page: site::RenderedPage) -> crate::Result<()> {
        let mut fh = File::create(self.create_path(page.metadata().url)?)?;
        io::copy(&mut page.read(), &mut fh)?;
        Ok(())
    }
}
