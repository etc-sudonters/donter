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

    fn create_path(&self, path: files::Path) -> crate::Result<path::PathBuf> {
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
    fn write_static_asset(
        &mut self,
        path: files::Path,
        asset: site::IncludedAsset,
    ) -> crate::Result<()> {
        use files::Path::*;
        let dest = self.create_path(path)?;
        match asset.path() {
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

    fn write_rendered_page(
        &mut self,
        path: files::FilePath,
        page: site::RenderedPage,
    ) -> crate::Result<()> {
        let mut fh = File::create(self.create_path(path.into())?)?;
        io::copy(&mut page.read(), &mut fh)?;
        Ok(())
    }
}
