use crate::{content, site};

pub fn default() -> Linker {
    Default::default()
}

#[derive(Default)]
pub struct Linker {}

impl site::Linker for Linker {
    fn link(&mut self, corpus: &mut content::Corpus) -> crate::Res<()> {
        todo!()
    }
}
