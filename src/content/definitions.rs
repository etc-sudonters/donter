use super::doctree;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Definitions<T: doctree::Definition> {
    labels: Vec<String>,
    defs: HashMap<String, T>,
}

impl<R: doctree::Reference, T: doctree::Definition> doctree::DefinitionLookup<R, T>
    for Definitions<T>
{
    fn lookup(&self, id: R) -> Option<&T> {
        self.defs.get(id.label())
    }
}

impl<T: doctree::Definition> Default for Definitions<T> {
    fn default() -> Self {
        Definitions {
            labels: Vec::new(),
            defs: HashMap::new(),
        }
    }
}

impl<T: doctree::Definition> Definitions<T> {
    pub fn add_label(&mut self, key: &String) {
        match self.labels.iter().find(|lbl| **lbl == *key) {
            None => self.labels.push(key.to_owned()),
            _ => {}
        }
    }

    pub fn define(&mut self, key: &String, value: T) {
        let entry = self.defs.entry(key.to_owned());
        entry.or_insert(value);
    }

    pub fn definitions(&self) -> impl Iterator<Item = &T> {
        self.labels.iter().map(|lbl| {
            self.defs
                .get(lbl)
                .expect(format!("reference {} was not defined", lbl).as_str())
        })
    }
}
