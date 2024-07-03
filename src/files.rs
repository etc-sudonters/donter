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

impl Path {
    pub fn impose(p: path::PathBuf) -> Option<Path> {
        if p.is_file() {
            Some(Path::File(FilePath(p)))
        } else if p.is_dir() {
            Some(Path::Dir(DirPath(p)))
        } else {
            None
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

impl From<FilePath> for String {
    fn from(value: FilePath) -> Self {
        value
            .0
            .into_os_string()
            .into_string()
            .expect("aaaaahh! weird file name")
    }
}

impl From<&FilePath> for String {
    fn from(value: &FilePath) -> Self {
        String::from(value.clone())
    }
}

impl Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        self.0.as_os_str().to_str().unwrap()
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

impl From<DirPath> for String {
    fn from(value: DirPath) -> Self {
        value
            .0
            .into_os_string()
            .into_string()
            .expect("aaaaahh! weird file name")
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

                match Path::impose(entry.path()) {
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
