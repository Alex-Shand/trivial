use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::Trivial;

/// To avoid conflicting impl error implementing [Trivial] for [HashMap]
#[derive(Debug)]
pub struct TrivialMap<K: Trivial + Eq + Hash, V: Trivial>(pub HashMap<K, V>);

impl<K: Trivial + Eq + Hash, V: Trivial> Deref for TrivialMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Trivial + Eq + Hash, V: Trivial> DerefMut for TrivialMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: Trivial + Eq + Hash, V: Trivial> From<HashMap<K, V>>
    for TrivialMap<K, V>
{
    fn from(value: HashMap<K, V>) -> Self {
        Self(value)
    }
}

impl<K: Trivial + Eq + Hash + Clone, V: Trivial + Clone> Clone
    for TrivialMap<K, V>
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<K: Trivial + Eq + Hash, V: Trivial> Trivial for TrivialMap<K, V> {
    fn dup(&self) -> Self {
        Self(self.0.iter().map(|(k, v)| (k.dup(), v.dup())).collect())
    }
}
