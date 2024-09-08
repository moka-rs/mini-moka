# Mini Moka

[![GitHub Actions][gh-actions-badge]][gh-actions]
[![crates.io release][release-badge]][crate]
[![docs][docs-badge]][docs]
[![dependency status][deps-rs-badge]][deps-rs]
<!-- [![coverage status][coveralls-badge]][coveralls] -->
[![license][license-badge]](#license)
<!-- [![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fmoka-rs%2Fmini-moka.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fmoka-rs%2Fmini-moka?ref=badge_shield) -->

Mini Moka is a fast, concurrent cache library for Rust. Mini Moka is a light edition
of [Moka][moka-git].

Mini Moka provides cache implementations on top of hash maps. They support full
concurrency of retrievals and a high expected concurrency for updates. Mini Moka also
provides a non-thread-safe cache implementation for single thread applications.

All caches perform a best-effort bounding of a hash map using an entry replacement
algorithm to determine which entries to evict when the capacity is exceeded.

[gh-actions-badge]: https://github.com/moka-rs/mini-moka/workflows/CI/badge.svg
[release-badge]: https://img.shields.io/crates/v/mini-moka.svg
[docs-badge]: https://docs.rs/mini-moka/badge.svg
[deps-rs-badge]: https://deps.rs/repo/github/moka-rs/mini-moka/status.svg
<!-- [coveralls-badge]: https://coveralls.io/repos/github/mini-moka-rs/moka/badge.svg?branch=main -->
[license-badge]: https://img.shields.io/crates/l/mini-moka.svg
<!-- [fossa-badge]: https://app.fossa.com/api/projects/git%2Bgithub.com%2Fmoka-rs%2Fmini-moka.svg?type=shield -->

[gh-actions]: https://github.com/moka-rs/mini-moka/actions?query=workflow%3ACI
[crate]: https://crates.io/crates/mini-moka
[docs]: https://docs.rs/mini-moka
[deps-rs]: https://deps.rs/repo/github/moka-rs/mini-moka
<!-- [coveralls]: https://coveralls.io/github/moka-rs/mini-moka?branch=main -->
<!-- [fossa]: https://app.fossa.com/projects/git%2Bgithub.com%2Fmoka-rs%2Fmini-moka?ref=badge_shield -->

[moka-git]: https://github.com/moka-rs/moka
[caffeine-git]: https://github.com/ben-manes/caffeine


## Features

- Thread-safe, highly concurrent in-memory cache implementation.
- A cache can be bounded by one of the followings:
    - The maximum number of entries.
    - The total weighted size of entries. (Size aware eviction)
- Maintains near optimal hit ratio by using an entry replacement algorithms inspired
  by Caffeine:
    - Admission to a cache is controlled by the Least Frequently Used (LFU) policy.
    - Eviction from a cache is controlled by the Least Recently Used (LRU) policy.
    - [More details and some benchmark results are available here][tiny-lfu].
- Supports expiration policies:
    - Time to live
    - Time to idle

<!--
Mini Moka provides a rich and flexible feature set while maintaining high hit ratio
and a high level of concurrency for concurrent access. However, it may not be as fast
as other caches, especially those that focus on much smaller feature sets.

If you do not need features like: time to live, and size aware eviction, you may want
to take a look at the [Quick Cache][quick-cache] crate.
-->

[tiny-lfu]: https://github.com/moka-rs/moka/wiki#admission-and-eviction-policies
<!-- [quick-cache]: https://crates.io/crates/quick_cache -->


## Change Log

- [CHANGELOG.md](https://github.com/moka-rs/mini-moka/blob/main/CHANGELOG.md)


## Table of Contents

- [Features](#features)
- [Change Log](#change-log)
- [Usage](#usage)
- [Example: Synchronous Cache](#example-synchronous-cache)
- [Avoiding to clone the value at `get`](#avoiding-to-clone-the-value-at-get)
- Examples (Part 2)
    - [Size Aware Eviction](#example-size-aware-eviction)
    - [Expiration Policies](#example-expiration-policies)
- [Minimum Supported Rust Versions](#minimum-supported-rust-versions)
- [Developing Mini Moka](#developing-mini-moka)
- [Credits](#credits)
- [License](#license)


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mini_moka = "0.10"
```


## Example: Synchronous Cache

The thread-safe, synchronous caches are defined in the `sync` module.

Cache entries are manually added using `insert` method, and are stored in the cache
until either evicted or manually invalidated.

Here's an example of reading and updating a cache by using multiple threads:

```rust
// Use the synchronous cache.
use mini_moka::sync::Cache;

use std::thread;

fn value(n: usize) -> String {
    format!("value {}", n)
}

fn main() {
    const NUM_THREADS: usize = 16;
    const NUM_KEYS_PER_THREAD: usize = 64;

    // Create a cache that can store up to 10,000 entries.
    let cache = Cache::new(10_000);

    // Spawn threads and read and update the cache simultaneously.
    let threads: Vec<_> = (0..NUM_THREADS)
        .map(|i| {
            // To share the same cache across the threads, clone it.
            // This is a cheap operation.
            let my_cache = cache.clone();
            let start = i * NUM_KEYS_PER_THREAD;
            let end = (i + 1) * NUM_KEYS_PER_THREAD;

            thread::spawn(move || {
                // Insert 64 entries. (NUM_KEYS_PER_THREAD = 64)
                for key in start..end {
                    my_cache.insert(key, value(key));
                    // get() returns Option<String>, a clone of the stored value.
                    assert_eq!(my_cache.get(&key), Some(value(key)));
                }

                // Invalidate every 4 element of the inserted entries.
                for key in (start..end).step_by(4) {
                    my_cache.invalidate(&key);
                }
            })
        })
        .collect();

    // Wait for all threads to complete.
    threads.into_iter().for_each(|t| t.join().expect("Failed"));

    // Verify the result.
    for key in 0..(NUM_THREADS * NUM_KEYS_PER_THREAD) {
        if key % 4 == 0 {
            assert_eq!(cache.get(&key), None);
        } else {
            assert_eq!(cache.get(&key), Some(value(key)));
        }
    }
}
```


## Avoiding to clone the value at `get`

For the concurrent cache (`sync` cache), the return type of `get` method is
`Option<V>` instead of `Option<&V>`, where `V` is the value type. Every time `get` is
called for an existing key, it creates a clone of the stored value `V` and returns
it. This is because the `Cache` allows concurrent updates from threads so a value
stored in the cache can be dropped or replaced at any time by any other thread. `get`
cannot return a reference `&V` as it is impossible to guarantee the value outlives
the reference.

If you want to store values that will be expensive to clone, wrap them by
`std::sync::Arc` before storing in a cache. [`Arc`][rustdoc-std-arc] is a thread-safe
reference-counted pointer and its `clone()` method is cheap.

[rustdoc-std-arc]: https://doc.rust-lang.org/stable/std/sync/struct.Arc.html

```rust,ignore
use std::sync::Arc;

let key = ...
let large_value = vec![0u8; 2 * 1024 * 1024]; // 2 MiB

// When insert, wrap the large_value by Arc.
cache.insert(key.clone(), Arc::new(large_value));

// get() will call Arc::clone() on the stored value, which is cheap.
cache.get(&key);
```


## Example: Size Aware Eviction

If different cache entries have different "weights" &mdash; e.g. each entry has
different memory footprints &mdash; you can specify a `weigher` closure at the cache
creation time. The closure should return a weighted size (relative size) of an entry
in `u32`, and the cache will evict entries when the total weighted size exceeds its
`max_capacity`.

```rust
use std::convert::TryInto;
use mini_moka::sync::Cache;

fn main() {
    let cache = Cache::builder()
        // A weigher closure takes &K and &V and returns a u32 representing the
        // relative size of the entry. Here, we use the byte length of the value
        // String as the size.
        .weigher(|_key, value: &String| -> u32 {
            value.len().try_into().unwrap_or(u32::MAX)
        })
        // This cache will hold up to 32MiB of values.
        .max_capacity(32 * 1024 * 1024)
        .build();
    cache.insert(0, "zero".to_string());
}
```

Note that weighted sizes are not used when making eviction selections.


## Example: Expiration Policies

Mini Moka supports the following expiration policies:

- **Time to live**: A cached entry will be expired after the specified duration past
  from `insert`.
- **Time to idle**: A cached entry will be expired after the specified duration past
  from `get` or `insert`.

To set them, use the `CacheBuilder`.

```rust
use mini_moka::sync::Cache;
use std::time::Duration;

fn main() {
    let cache = Cache::builder()
        // Time to live (TTL): 30 minutes
        .time_to_live(Duration::from_secs(30 * 60))
        // Time to idle (TTI):  5 minutes
        .time_to_idle(Duration::from_secs( 5 * 60))
        // Create the cache.
        .build();

    // This entry will expire after 5 minutes (TTI) if there is no get().
    cache.insert(0, "zero");

    // This get() will extend the entry life for another 5 minutes.
    cache.get(&0);

    // Even though we keep calling get(), the entry will expire
    // after 30 minutes (TTL) from the insert().
}
```

### A note on expiration policies

The cache builders will panic if configured with either `time_to_live` or `time to
idle` longer than 1000 years. This is done to protect against overflow when computing
key expiration.


## Minimum Supported Rust Versions

Mini Moka's minimum supported Rust versions (MSRV) are the followings:

| Feature          | MSRV                       |
|:-----------------|:--------------------------:|
| default features | Rust 1.76.0 (Feb 8, 2024)  |

It will keep a rolling MSRV policy of at least 6 months. If only the default features
are enabled, MSRV will be updated conservatively. When using other features, MSRV
might be updated more frequently, up to the latest stable. In both cases, increasing
MSRV is _not_ considered a semver-breaking change.


## Developing Mini Moka

**Running All Tests**

To run all tests including doc tests on the README, use the following command:

```console
$ RUSTFLAGS='--cfg trybuild' cargo test --all-features
```


**Generating the Doc**

```console
$ cargo +nightly -Z unstable-options --config 'build.rustdocflags="--cfg docsrs"' \
    doc --no-deps
```


## Credits

### Caffeine

Mini Moka's architecture is heavily inspired by the [Caffeine][caffeine-git] library
for Java. Thanks go to Ben Manes and all contributors of Caffeine.


## License

Mini Moka is distributed under either of

- The MIT license
- The Apache License (Version 2.0)

at your option.

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.

<!-- [![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fmoka-rs%2Fmini-moka.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fmoka-rs%2Fmini-moka?ref=badge_large) -->
