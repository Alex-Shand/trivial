use std::{ops::Deref, rc::Rc};

use crate::Claim;

/// To avoid conflicting impl error implementing [Claim] for [Rc]
#[derive(Debug, Default)]
pub struct ClaimRc<T: ?Sized>(pub Rc<T>);

impl<T: ?Sized> Deref for ClaimRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> From<Rc<T>> for ClaimRc<T> {
    fn from(value: Rc<T>) -> Self {
        Self(value)
    }
}

impl<T: ?Sized> From<ClaimRc<T>> for Rc<T> {
    fn from(ClaimRc(rc): ClaimRc<T>) -> Self {
        rc
    }
}

impl<T: ?Sized> Clone for ClaimRc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> Claim for ClaimRc<T> {
    fn claim(&self) -> Self {
        self.clone()
    }
}
