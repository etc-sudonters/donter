use std::collections::HashMap;

use markdown::mdast;
use yaml_rust2::{self as yaml, yaml::Hash, Yaml, YamlLoader};

use crate::content::{self, Metadata, PageBuilder, PageStatus};

pub fn frontmatter_to_page_meta(y: &mdast::Yaml, b: &mut PageBuilder) -> crate::Result<()> {
    let docs = YamlLoader::load_from_str(&y.value)?;
    let metadoc = &docs[0];

    let meta = match &docs[0] {
        Yaml::Hash(map) => convert_yaml_map(map)?,
        _ => Err(super::Error::Unexpected(
            "frontmatter must have object at top level".to_owned(),
        ))?,
    };

    b.with_title(
        meta.get("title")
            .map(|t| match t {
                Metadata::Str(s) => Ok(s.clone()),
                _ => Err(super::Error::Unexpected(
                    "title must be a string".to_owned(),
                )),
            })
            .expect("title key must be provided")?,
    );

    b.written(
        meta.get("date")
            .map(|d| match d {
                Metadata::Str(s) => Ok(content::Date::new(s.clone())),
                _ => Err(super::Error::Unexpected("date must be a string".to_owned())),
            })
            .expect("date key must be provided")?,
    );

    b.status(
        meta.get("status")
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

    b.meta = meta;

    Ok(())
}

impl TryFrom<&Yaml> for Metadata {
    type Error = super::Error;
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
            )),
        }
    }
}

fn convert_yaml_map(m: &Hash) -> Result<HashMap<String, Metadata>, super::Error> {
    let mut map = HashMap::new();

    for (k, v) in m.iter() {
        match k {
            Yaml::String(s) => {
                map.insert(s.to_lowercase(), v.try_into()?);
            }
            _ => Err(super::Error::Unexpected(
                "Frontmatter objects must have string keys".to_owned(),
            ))?,
        }
    }

    Ok(map)
}
