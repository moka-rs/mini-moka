use super::{base_cache::BaseCache, mapref::EntryRef};
use crate::common::concurrent::{arc::MiniArc, ValueEntry};

use std::{
    hash::{BuildHasher, Hash},
    sync::Arc,
};

pub(crate) type DashMapIter<'a, K, V, S> =
    dashmap::iter::Iter<'a, Arc<K>, MiniArc<ValueEntry<K, V>>, S>;

pub struct Iter<'a, K, V, S> {
    cache: &'a BaseCache<K, V, S>,
    map_iter: DashMapIter<'a, K, V, S>,
}

impl<'a, K, V, S> Iter<'a, K, V, S> {
    pub(crate) fn new(cache: &'a BaseCache<K, V, S>, map_iter: DashMapIter<'a, K, V, S>) -> Self {
        Self { cache, map_iter }
    }
}

impl<'a, K, V, S> Iterator for Iter<'a, K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher + Clone,
{
    type Item = EntryRef<'a, K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        for map_ref in &mut self.map_iter {
            if !self.cache.is_expired_entry(map_ref.value()) {
                return Some(EntryRef::new(map_ref));
            }
        }

        None
    }
}

// Clippy beta 0.1.83 (f41c7ed9889 2024-10-31) warns about unused lifetimes on 'a.
// This seems a false positive. The lifetimes are used in the trait bounds.
// https://rust-lang.github.io/rust-clippy/master/index.html#extra_unused_lifetimes
#[allow(clippy::extra_unused_lifetimes)]
unsafe impl<'a, K, V, S> Send for Iter<'_, K, V, S>
where
    K: 'a + Eq + Hash + Send,
    V: 'a + Send,
    S: 'a + BuildHasher + Clone,
{
}

// Clippy beta 0.1.83 (f41c7ed9889 2024-10-31) warns about unused lifetimes on 'a.
// This seems a false positive. The lifetimes are used in the trait bounds.
// https://rust-lang.github.io/rust-clippy/master/index.html#extra_unused_lifetimes
#[allow(clippy::extra_unused_lifetimes)]
unsafe impl<'a, K, V, S> Sync for Iter<'_, K, V, S>
where
    K: 'a + Eq + Hash + Sync,
    V: 'a + Sync,
    S: 'a + BuildHasher + Clone,
{
}
