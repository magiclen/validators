#[cfg(feature = "serde_json")]
use alloc::string::String;
use alloc::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
    vec::Vec,
};
#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "serde_json")]
use serde_json::{Map, Value};

/// For types which should have a `len` method.
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

impl<T> CollectionLength for VecDeque<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> CollectionLength for LinkedList<T> {
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

// Any hasher is allowed, so types such as `FxHashSet` also work.
#[cfg(feature = "std")]
impl<T, S> CollectionLength for HashSet<T, S> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<K, T, S> CollectionLength for HashMap<K, T, S> {
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
