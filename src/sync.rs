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

/// Allows users to implement their own EvictionHandler
pub trait EvictionHandler<K, V>
where
    K: Send + Sync,
    V: Send + Sync,
{
    fn on_remove(&self, _: Arc<K>, _: &V) {}
}
