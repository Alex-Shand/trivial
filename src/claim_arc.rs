use std::{ops::Deref, sync::Arc};

use crate::Claim;

/// To avoid conflicting impl error implementing [Claim] for [Arc]
#[derive(Debug, Default)]
pub struct ClaimArc<T: ?Sized>(pub Arc<T>);

impl<T> ClaimArc<T> {
    /// Constructor
    pub fn new(t: T) -> Self {
        ClaimArc(Arc::new(t))
    }
}

impl<T: ?Sized> Deref for ClaimArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> From<Arc<T>> for ClaimArc<T> {
    fn from(value: Arc<T>) -> Self {
        Self(value)
    }
}

impl<T: ?Sized> From<ClaimArc<T>> for Arc<T> {
    fn from(ClaimArc(arc): ClaimArc<T>) -> Self {
        arc
    }
}

impl<T: ?Sized> Clone for ClaimArc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> Claim for ClaimArc<T> {
    fn claim(&self) -> Self {
        self.clone()
    }
}
