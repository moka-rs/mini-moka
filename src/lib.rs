#![warn(clippy::all)]
#![warn(rust_2018_idioms)]
#![deny(rustdoc::broken_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Mini Moka is a fast, concurrent cache library for Rust. Mini Moka is a light
//! edition of [Moka][moka-git].
//!
//! Mini Moka provides an in-memory concurrent cache implementation on top of hash
//! map. It supports high expected concurrency of retrievals and updates.
//!
//! Mini Moka also provides an in-memory, non-thread-safe cache implementation for
//! single thread applications.
//!
//! All cache implementations perform a best-effort bounding of the map using an
//! entry replacement algorithm to determine which entries to evict when the capacity
//! is exceeded.
//!
//! [moka-git]: https://github.com/moka-rs/moka
//! [caffeine-git]: https://github.com/ben-manes/caffeine
//!
//! # Features
//!
//! - A thread-safe, highly concurrent in-memory cache implementation.
//! - A cache can be bounded by one of the followings:
//!     - The maximum number of entries.
//!     - The total weighted size of entries. (Size aware eviction)
//! - Maintains good hit rate by using entry replacement algorithms inspired by
//!   [Caffeine][caffeine-git]:
//!     - Admission to a cache is controlled by the Least Frequently Used (LFU) policy.
//!     - Eviction from a cache is controlled by the Least Recently Used (LRU) policy.
//! - Supports expiration policies:
//!     - Time to live
//!     - Time to idle
//!
//! # Examples
//!
//! See the following document:
//!
//! - A thread-safe, synchronous cache:
//!     - [`sync::Cache`][sync-cache-struct]
//! - A not thread-safe, blocking cache for single threaded applications:
//!     - [`unsync::Cache`][unsync-cache-struct]
//!
//! [sync-cache-struct]: ./sync/struct.Cache.html
//! [unsync-cache-struct]: ./unsync/struct.Cache.html
//!
//! # Minimum Supported Rust Versions
//!
//! This crate's minimum supported Rust versions (MSRV) are the followings:
//!
//! | Feature          | MSRV                       |
//! |:-----------------|:--------------------------:|
//! | default features | Rust 1.61.0 (May 19, 2022) |
//!
//! If only the default features are enabled, MSRV will be updated conservatively.
//! When using other features, MSRV might be updated more frequently, up to the
//! latest stable. In both cases, increasing MSRV is _not_ considered a
//! semver-breaking change.

pub mod common;
pub(crate) mod policy;
pub mod unsync;

#[cfg(feature = "sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub mod sync;

pub use policy::Policy;

#[cfg(test)]
mod tests {
    #[cfg(all(trybuild, feature = "sync"))]
    #[test]
    fn trybuild_sync() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/compile_tests/sync/clone/*.rs");
    }
}
