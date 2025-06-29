use std::ops::Deref;

use crate::Trivial;

/// To avoid conflicting impl error implementing [Trivial] for [Box]
#[derive(Debug)]
pub struct TrivialBox<T: Trivial>(pub Box<T>);

impl<T: Trivial> TrivialBox<T> {
    /// Constructor
    pub fn new(t: T) -> Self {
        Self(Box::new(t))
    }

    /// Extract the inner box
    #[must_use]
    pub fn into_box(self) -> Box<T> {
        self.0
    }

    /// Extract the inner value
    #[must_use]
    pub fn take(self) -> T {
        *self.0
    }
}

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

impl<T: Trivial + PartialEq> PartialEq for TrivialBox<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Trivial + Eq> Eq for TrivialBox<T> {}

impl<T: Trivial> Trivial for TrivialBox<T> {
    fn dup(&self) -> Self {
        Self(Box::new(self.0.dup()))
    }
}
