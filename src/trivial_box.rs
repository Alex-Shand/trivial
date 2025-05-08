use std::ops::Deref;

use crate::Trivial;

/// To avoid conflicting impl error implementing [Trivial] for [Box]
#[derive(Debug)]
pub struct TrivialBox<T: Trivial>(pub Box<T>);

impl<T: Trivial> Deref for TrivialBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Trivial> From<Box<T>> for TrivialBox<T> {
    fn from(value: Box<T>) -> Self {
        Self(value)
    }
}

impl<T: Trivial + Clone> Clone for TrivialBox<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Trivial> Trivial for TrivialBox<T> {
    fn dup(&self) -> Self {
        Self(Box::new(self.0.dup()))
    }
}
