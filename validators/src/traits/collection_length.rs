#[cfg(feature = "serde_json")]
use alloc::string::String;
use alloc::{
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    vec::Vec,
};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "serde_json")]
use serde_json::{Map, Value};

/// For types which should have the a `len` method.
pub trait CollectionLength {
    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> CollectionLength for Vec<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> CollectionLength for BinaryHeap<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> CollectionLength for BTreeSet<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<K, T> CollectionLength for BTreeMap<K, T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<T> CollectionLength for HashSet<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<K, T> CollectionLength for HashMap<K, T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "serde_json")]
impl CollectionLength for Map<String, Value> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}
