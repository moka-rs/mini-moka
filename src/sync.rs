//! Provides a thread-safe, concurrent cache implementation built upon
//! [`dashmap::DashMap`][dashmap].
//!
//! [dashmap]: https://docs.rs/dashmap/*/dashmap/struct.DashMap.html

mod base_cache;
mod builder;
mod cache;
mod iter;
mod mapref;

pub use builder::CacheBuilder;
pub use cache::Cache;
pub use iter::Iter;
pub use mapref::EntryRef;

use std::sync::Arc;

/// Provides extra methods that will be useful for testing.
pub trait ConcurrentCacheExt<K, V> {
    /// Performs any pending maintenance operations needed by the cache.
    fn sync(&self);
}

pub(crate) type EvictionHandler<K, V> =
    Arc<dyn Fn(Arc<K>, &V, RemovalCause) + Send + Sync + 'static>;

pub(crate) fn default_eviction_handler<K, V>() -> EvictionHandler<K, V> {
    Arc::new(|_, _, _| {})
}

/// Indicates the reason why a cached entry was removed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RemovalCause {
    /// The entry's expiration timestamp has passed.
    Expired,
    /// The entry was manually removed by the user.
    Explicit,
    /// The entry itself was not actually removed, but its value was replaced by
    /// the user.
    Replaced,
    /// The entry was evicted due to size constraints.
    Size,
}
