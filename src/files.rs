use std::{fmt::Display, ops::Deref, path};

pub enum RecursionBehavior {
    Recurse,
    Dont,
}

pub struct Walker<'a> {
    base: &'a DirPath,
    recurse: RecursionBehavior,
}

impl<'a> Walker<'a> {
    pub fn walk(base: &'a DirPath, recurse: RecursionBehavior) -> impl Iterator<Item = FilePath> {
        Self { base, recurse }.into_iter()
    }
}

impl<'a> IntoIterator for Walker<'a> {
    type Item = FilePath;
    type IntoIter = BreadthFirstWalker;
    fn into_iter(self) -> Self::IntoIter {
        BreadthFirstWalker {
            recurse: self.recurse,
            dirs: vec![self.base.clone()],
            files: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Path {
    File(FilePath),
    Dir(DirPath),
}

impl Display for Path {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(f) => Display::fmt(f, fmt),
            Self::Dir(d) => Display::fmt(d, fmt),
        }
    }
}

impl AsRef<path::Path> for Path {
    fn as_ref(&self) -> &path::Path {
        use Path::*;
        match self {
            File(p) => p.as_ref(),
            Dir(p) => p.as_ref(),
        }
    }
}

impl AsRef<Path> for Path {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl Path {
    pub fn parse<P: Into<path::PathBuf>>(p: P) -> Result<Path, path::PathBuf> {
        let path = p.into();
        if path.is_file() {
            Ok(Path::File(FilePath(path)))
        } else if path.is_dir() {
            Ok(Path::Dir(DirPath(path)))
        } else {
            Err(path)
        }
    }

    pub fn as_file(self) -> Option<FilePath> {
        match self {
            Self::File(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_dir(self) -> Option<DirPath> {
        match self {
            Self::Dir(d) => Some(d),
            _ => None,
        }
    }

    pub fn ext(&self) -> Option<&str> {
        let path = AsRef::<path::Path>::as_ref(self);
        path.extension().map(|ext| ext.to_str()).flatten()
    }

    pub fn parent(&self) -> Option<&str> {
        let path = AsRef::<path::Path>::as_ref(self);
        path.parent().map(|s| s.to_str()).flatten()
    }
}

impl From<FilePath> for Path {
    fn from(value: FilePath) -> Self {
        Self::File(value)
    }
}

impl From<DirPath> for Path {
    fn from(value: DirPath) -> Self {
        Self::Dir(value)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FilePath(path::PathBuf);

impl Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

impl Deref for FilePath {
    type Target = path::PathBuf;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FilePath {
    pub unsafe fn new<P: Into<path::PathBuf>>(p: P) -> FilePath {
        FilePath(p.into())
    }

    pub fn as_path(&self) -> &path::Path {
        self.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DirPath(path::PathBuf);

impl DirPath {
    pub unsafe fn new<P: Into<path::PathBuf>>(p: P) -> DirPath {
        DirPath(p.into())
    }

    pub fn join<P: AsRef<path::Path>>(&self, path: P) -> path::PathBuf {
        self.0.join(path)
    }
}

#[derive(Debug)]
pub enum PathError {
    Unsupported(path::PathBuf),
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PathError::*;
        write!(f, "PathError::")?;
        match self {
            Unsupported(path) => write!(f, "Unsupported({})", path.to_string_lossy()),
        }
    }
}

impl std::error::Error for PathError {}

impl From<path::PathBuf> for Path {
    fn from(value: path::PathBuf) -> Self {
        Self::parse(value).unwrap()
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

impl Display for DirPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
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
                match Path::parse(entry.path()) {
                    Ok(p) => match p {
                        Path::File(f) => self.files.push(f),
                        Path::Dir(d) => self.dirs.push(d),
                    },
                    _ => {}
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
