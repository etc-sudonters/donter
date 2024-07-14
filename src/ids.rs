use std::{fmt::Display, hash::Hash, marker::PhantomData};

#[derive(Debug, Copy)]
pub struct Id<T> {
    id: u64,
    parent: u64,
    _t: PhantomData<T>,
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id<{}>({}, {})",
            std::any::type_name::<T>(),
            self.parent,
            self.id
        )
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parent.hash(state);
        self.id.hash(state);
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.parent == other.parent
    }
}

impl<T> Eq for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            parent: self.parent.clone(),
            _t: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct IdPool<T> {
    nonce: u64,
    last: u64,
    _t: PhantomData<T>,
}

impl<T> IdPool<T> {
    pub fn new(nonce: u64) -> Self {
        Self {
            last: 0,
            _t: PhantomData,
            nonce,
        }
    }

    pub fn next(&mut self) -> Id<T> {
        let id = Id {
            id: self.last,
            parent: self.nonce,
            _t: PhantomData,
        };
        self.last += 1;
        id
    }
}
