use std::path::{self, PathBuf};

pub struct NamedReader {
    filepath: FilePath,
    reader: Box<dyn std::io::Read>,
}

impl std::io::Read for NamedReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl NamedReader {
    pub fn path(&self) -> FilePath {
        self.filepath.clone()
    }

    pub fn create(p: FilePath, r: Box<dyn std::io::Read>) -> Self {
        Self {
            filepath: p,
            reader: r,
        }
    }
}

pub enum RecursionBehavior {
    Recurse,
    Dont,
}

pub struct Walker {
    base: DirPath,
    recurse: RecursionBehavior,
}

impl Walker {
    pub fn new(base: DirPath, recurse: RecursionBehavior) -> Walker {
        Walker { base, recurse }
    }
}

impl IntoIterator for Walker {
    type Item = FilePath;
    type IntoIter = BreadthFirstWalker;
    fn into_iter(self) -> Self::IntoIter {
        BreadthFirstWalker {
            recurse: self.recurse,
            dirs: vec![self.base],
            files: Vec::new(),
        }
    }
}

impl From<DirPath> for Walker {
    fn from(value: DirPath) -> Self {
        Walker {
            base: value,
            recurse: RecursionBehavior::Recurse,
        }
    }
}

pub enum PathKind {
    File,
    Dir,
}

#[derive(Debug)]
pub enum Path {
    File(FilePath),
    Dir(DirPath),
}

impl PathKind {
    pub fn impose(p: path::PathBuf) -> Option<Path> {
        if p.is_file() {
            Some(Path::File(FilePath(p)))
        } else if p.is_dir() {
            Some(Path::Dir(DirPath(p)))
        } else {
            None
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
    recurse: RecursionBehavior,
    dirs: Vec<DirPath>,
    files: Vec<FilePath>,
}

impl BreadthFirstWalker {
    fn pop(&mut self) -> Option<<Self as Iterator>::Item> {
        self.files.pop()
    }

    fn reload(&mut self) {
        if matches!(self.recurse, RecursionBehavior::Dont if self.dirs.len() == 0) {
            return;
        }

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
                    Some(p) => match p {
                        Path::File(f) => self.files.push(f),
                        Path::Dir(d) => self.dirs.push(d),
                    },
                    None => {}
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
