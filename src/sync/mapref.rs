use crate::common::concurrent::ValueEntry;

use std::{hash::Hash, sync::Arc};
use triomphe::Arc as TrioArc;

type DashMapRef<'a, K, V> =
    dashmap::mapref::multiple::RefMulti<'a, Arc<K>, TrioArc<ValueEntry<K, V>>>;

pub struct EntryRef<'a, K, V>(DashMapRef<'a, K, V>);

unsafe impl<'a, K, V> Sync for EntryRef<'a, K, V>
where
    K: Eq + Hash + Send + Sync,
    V: Send + Sync,
{
}

impl<'a, K, V> EntryRef<'a, K, V>
where
    K: Eq + Hash,
{
    pub(crate) fn new(map_ref: DashMapRef<'a, K, V>) -> Self {
        Self(map_ref)
    }

    pub fn key(&self) -> &K {
        self.0.key()
    }

    pub fn value(&self) -> &V {
        &self.0.value().value
    }

    pub fn pair(&self) -> (&K, &V) {
        (self.key(), self.value())
    }
}

impl<'a, K, V> std::ops::Deref for EntryRef<'a, K, V>
where
    K: Eq + Hash,
{
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}
