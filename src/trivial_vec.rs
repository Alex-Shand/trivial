use std::ops::Deref;

use crate::Trivial;

/// To avoid conflicting impl error implementing [Trivial] for [Vec]
#[derive(Debug)]
pub struct TrivialVec<T: Trivial>(pub Vec<T>);

impl<T: Trivial> Deref for TrivialVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Trivial> From<Vec<T>> for TrivialVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T: Trivial + Clone> Clone for TrivialVec<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Trivial> Trivial for TrivialVec<T> {
    fn dup(&self) -> Self {
        Self(self.0.iter().map(Trivial::dup).collect())
    }
}
