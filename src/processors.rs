use crate::site;

pub struct TimeToRead;
impl TimeToRead {
    pub fn create() -> TimeToRead {
        TimeToRead
    }
}
impl site::Preprocessor for TimeToRead {
    fn run(&mut self, corpus: &mut crate::content::Corpus) -> crate::Result<()> {
        todo!()
    }
}

pub struct ReferenceOrganization;
impl ReferenceOrganization {
    pub fn create() -> ReferenceOrganization {
        ReferenceOrganization
    }
}
impl site::Preprocessor for ReferenceOrganization {
    fn run(&mut self, corpus: &mut crate::content::Corpus) -> crate::Result<()> {
        todo!()
    }
}

pub mod index {
    use crate::{content, site, Result};

    #[derive(Default)]
    pub struct Tag {}
    #[derive(Default)]
    pub struct Date {}
    #[derive(Default)]
    pub struct Series {}

    impl Tag {
        pub fn create() -> Tag {
            Tag {}
        }
    }

    impl Date {
        pub fn create() -> Date {
            Date {}
        }
    }

    impl Series {
        pub fn create() -> Series {
            Series {}
        }
    }

    impl Indexer for Tag {
        fn index(&mut self, corpus: &mut content::Corpus) -> Result<()> {
            todo!()
        }
    }

    impl Indexer for Date {
        fn index(&mut self, corpus: &mut content::Corpus) -> Result<()> {
            todo!()
        }
    }
    impl Indexer for Series {
        fn index(&mut self, corpus: &mut content::Corpus) -> Result<()> {
            todo!()
        }
    }

    pub trait Indexer {
        fn index(&mut self, corpus: &mut content::Corpus) -> Result<()>;
    }

    impl<T> site::Postprocessor for T
    where
        T: Indexer,
    {
        fn run(&mut self, corpus: &mut content::Corpus) -> Result<()> {
            self.index(corpus)
        }
    }
}

pub mod feed {
    use crate::site::Processor;

    pub struct Atom;
    impl Atom {
        pub fn create() -> Atom {
            Atom
        }
    }

    impl Processor for Atom {
        fn run(&mut self, corpus: &mut crate::content::Corpus) -> crate::Result<()> {
            todo!()
        }
    }

    pub struct Rss;
    impl Rss {
        pub fn create() -> Rss {
            Rss
        }
    }

    impl Processor for Rss {
        fn run(&mut self, corpus: &mut crate::content::Corpus) -> crate::Result<()> {
            todo!()
        }
    }
}
