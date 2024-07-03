use markdown::mdast;
use yaml_rust2::{self as yaml, Yaml, YamlLoader};

use crate::content::{Metadata, PageBuilder};

pub fn frontmatter_to_page_meta(y: &mdast::Yaml, b: &mut PageBuilder) -> crate::Result<()> {
    let docs = YamlLoader::load_from_str(&y.value)?;
    let meta = &docs[0];

    Ok(())
}

impl TryFrom<&Yaml> for Metadata {
    type Error = super::Error;
    fn try_from(value: &Yaml) -> Result<Self, Self::Error> {
        match value {
            r @ Yaml::Real(_) => Ok(Metadata::Number(r.as_f64().unwrap())),
            Yaml::Hash(m) => Ok(todo!()),
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
