use std::{collections::HashMap, fmt::Display};

use markdown::mdast;
use yaml_rust2::{self as yaml, yaml::Hash, Yaml, YamlLoader};

use crate::{
    content::{self, Metadata, PageBuilder, PageStatus},
    files,
};

#[derive(Debug)]
pub struct GenericError(String);

impl GenericError {
    pub fn with_reason<E: std::fmt::Display, S: AsRef<str>>(err: E, reason: S) -> GenericError {
        Self(format!("{}: {}", reason.as_ref(), err))
    }
}

impl std::error::Error for GenericError {}

impl Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn frontmatter_to_page_meta(y: &mdast::Yaml, b: &mut PageBuilder) -> crate::Result<()> {
    let docs = YamlLoader::load_from_str(&y.value)
        .map_err(|e| GenericError::with_reason(e, format!("on page {}", b.filepath)))?;
    let metadoc = &docs[0];

    b.meta = match &docs[0] {
        Yaml::Hash(map) => convert_yaml_map(map)?
            .into_iter()
            .map(|(k, v)| (k.to_lowercase(), v))
            .collect(),
        _ => Err(super::Error::Unexpected(
            "frontmatter must have object at top level".to_owned(),
        ))?,
    };

    b.with_title(
        b.meta
            .get("title")
            .map(|t| match t {
                Metadata::Str(s) => Ok(s.clone()),
                _ => Err(super::Error::Unexpected(
                    "title must be a string".to_owned(),
                )),
            })
            .expect("title key must be provided")?,
    );

    b.written(
        b.meta
            .get("date")
            .map(|d| match d {
                Metadata::Str(s) => Ok(s.clone()),
                _ => Err(super::Error::Unexpected("date must be a string".to_owned())),
            })
            .expect("date key must be provided")?,
    );

    b.status(
        b.meta
            .get("status")
            .map(|s| match s {
                Metadata::Str(s) => Ok(match s.to_lowercase().as_str() {
                    "draft" => PageStatus::Draft,
                    _ => PageStatus::Published,
                }),
                _ => Err(super::Error::Unexpected(
                    "status must be a string".to_owned(),
                )),
            })
            .unwrap_or(Ok(PageStatus::Published))?,
    );

    Ok(())
}

impl TryFrom<&Yaml> for Metadata {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
        match value {
            r @ Yaml::Real(_) => Ok(Metadata::Number(r.as_f64().unwrap())),
            Yaml::Hash(m) => Ok(Metadata::Map(convert_yaml_map(m)?)),
            Yaml::Array(a) => Ok(Metadata::List(
                a.iter()
                    .filter_map(|y| TryInto::<Metadata>::try_into(y).ok())
                    .collect(),
            )),
            Yaml::String(s) => Ok(Metadata::Str(s.clone())),
            Yaml::Integer(i) => Ok(Metadata::Number(*i as f64)),
            _ => Err(super::Error::Unexpected(
                "Unsupported yaml in frontmatter".to_owned(),
            ))?,
        }
    }
}

fn convert_yaml_map(m: &Hash) -> crate::Result<HashMap<String, Metadata>> {
    let mut map = HashMap::new();

    for (k, v) in m.iter() {
        match k {
            Yaml::String(s) => {
                map.insert(
                    s.to_lowercase(),
                    v.try_into().map_err(|e| {
                        GenericError::with_reason(e, format!("while processing key: {s}"))
                    })?,
                );
            }
            _ => Err(super::Error::Unexpected(
                "Frontmatter objects must have string keys".to_owned(),
            ))?,
        }
    }

    Ok(map)
}
