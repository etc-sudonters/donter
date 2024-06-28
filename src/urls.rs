use std::collections::HashMap;

pub struct Url;

impl Url {
    pub fn parse<S: AsRef<str>>(s: S) -> crate::Result<Url> {
        Ok(Url)
    }
}

#[derive(Default, Debug)]
pub struct UrlBuilder {
    domain: Option<String>,
    fragment: Option<String>,
    path: Option<String>,
    port: Option<u16>,
    protocol: Option<String>,
    query_string: Option<HashMap<String, String>>,
}

impl UrlBuilder {
    pub fn build(self) -> Url {
        Url
    }
}
