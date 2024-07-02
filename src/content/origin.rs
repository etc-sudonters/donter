use serde::de::Visitor;

use crate::files;

#[derive(Debug)]
pub struct Origin(files::FilePath);

impl Origin {
    pub fn new(p: files::FilePath) -> Origin {
        Self(p)
    }
}

impl std::ops::Deref for Origin {
    type Target = files::FilePath;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Origin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl serde::Serialize for Origin {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_path().to_str().unwrap())
    }
}

impl<'de> serde::Deserialize<'de> for Origin {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(OriginVisitor)
    }
}

struct OriginVisitor;

impl<'de> Visitor<'de> for OriginVisitor {
    type Value = Origin;
    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Origin(unsafe { files::FilePath::new(v) }))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a filepath")
    }
}
