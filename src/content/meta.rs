use std::collections::HashMap;

#[derive(Debug)]
pub enum Metadata {
    List(Vec<Metadata>),
    Map(HashMap<String, Metadata>),
    Number(f64),
    Str(String),
    Bool(bool),
}
