use std::path::{self, PathBuf};

pub struct Walker {
    base: DirPath,
}

impl IntoIterator for Walker {
    type Item = FilePath;
    type IntoIter = BreadthFirstWalker;
    fn into_iter(self) -> Self::IntoIter {
        BreadthFirstWalker {
            dirs: vec![self.base],
            files: Vec::new(),
        }
    }
}

impl From<DirPath> for Walker {
    fn from(value: DirPath) -> Self {
        Walker { base: value }
    }
}

pub enum PathKind {
    File,
    Dir,
}

pub enum Path {
    File(FilePath),
    Dir(DirPath),
    Unsupported,
}

impl PathKind {
    pub fn impose(p: path::PathBuf) -> Path {
        if p.is_file() {
            Path::File(FilePath(p))
        } else if p.is_dir() {
            Path::Dir(DirPath(p))
        } else {
            Path::Unsupported
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilePath(path::PathBuf);

impl FilePath {
    pub unsafe fn new<P: Into<PathBuf>>(p: P) -> FilePath {
        FilePath(p.into())
    }
}

#[derive(Debug, Clone)]
pub struct DirPath(path::PathBuf);
impl DirPath {
    pub unsafe fn new<P: Into<PathBuf>>(p: P) -> DirPath {
        DirPath(p.into())
    }
}

impl AsRef<path::Path> for DirPath {
    fn as_ref(&self) -> &path::Path {
        AsRef::<path::Path>::as_ref(&self.0)
    }
}

impl AsRef<path::Path> for FilePath {
    fn as_ref(&self) -> &path::Path {
        AsRef::<path::Path>::as_ref(&self.0)
    }
}

pub struct BreadthFirstWalker {
    dirs: Vec<DirPath>,
    files: Vec<FilePath>,
}

impl BreadthFirstWalker {
    fn pop(&mut self) -> Option<<Self as Iterator>::Item> {
        self.files.pop()
    }

    fn reload(&mut self) {
        loop {
            let dir = match self.dirs.pop() {
                None => break,
                Some(d) => d,
            };

            let entries = match std::fs::read_dir(dir) {
                Err(_) => continue,
                Ok(e) => e,
            };

            for entry in entries {
                let entry = match entry {
                    Err(_) => continue,
                    Ok(e) => e,
                };

                match PathKind::impose(entry.path()) {
                    Path::File(f) => self.files.push(f),
                    Path::Dir(d) => self.dirs.push(d),
                    Path::Unsupported => {}
                }
            }
        }
    }

    fn reload_and_pop(&mut self) -> Option<<Self as Iterator>::Item> {
        self.reload();
        self.pop()
    }
}

impl Iterator for BreadthFirstWalker {
    type Item = FilePath;
    fn next(&mut self) -> Option<Self::Item> {
        match self.pop() {
            None => self.reload_and_pop(),
            f @ _ => f,
        }
    }
}
