use std::{
    collections::HashSet,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::Trivial;

/// To avoid conflicting impl error implementing [Trivial] for [HashSet]
#[derive(Debug)]
pub struct TrivialSet<T: Trivial>(pub HashSet<T>);

impl<T: Trivial + Eq + Hash> Deref for TrivialSet<T> {
    type Target = HashSet<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Trivial + Eq + Hash> DerefMut for TrivialSet<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Trivial + Eq + Hash> From<HashSet<T>> for TrivialSet<T> {
    fn from(value: HashSet<T>) -> Self {
        Self(value)
    }
}

impl<T: Trivial + Eq + Hash + Clone> Clone for TrivialSet<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Trivial + Eq + Hash> Trivial for TrivialSet<T> {
    fn dup(&self) -> Self {
        Self(self.0.iter().map(Trivial::dup).collect())
    }
}
