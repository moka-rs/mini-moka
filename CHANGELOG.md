# Mini Moka Cache &mdash; Change Log

## Version 0.10.3

### Fixed

- Fixed occasional panic in internal `FrequencySketch` in debug build.
  ([#21][gh-issue-0021])


## Version 0.10.2

### Fixed

- Fixed a memory corruption bug caused by the timing of concurrent `insert`,
  `get` and removal of the same cached entry. ([#15][gh-pull-0015]).


## Version 0.10.1

Bumped the minimum supported Rust version (MSRV) to 1.61 (May 19, 2022).
([#5][gh-pull-0005])

### Fixed

- Fixed the caches mutating a deque node through a `NonNull` pointer derived from a
  shared reference. ([#6][gh-pull-0006]).


## Version 0.10.0

In this version, we removed some dependencies from Mini Moka to make it more
lightweight.

### Removed

- Remove the background threads from the `sync::Cache` ([#1][gh-pull-0001]):
    - Also remove the following dependencies:
        - `scheduled-thread-pool`
        - `num_cpus`
        - `once_cell` (Moved to the dev-dependencies)
- Remove the following dependencies and crate features ([#2][gh-pull-0002]):
    - Removed dependencies:
        - `quanta`
        - `parking_lot`
        - `rustc_version` (from the build-dependencies)
    - Removed crate features:
        - `quanta` (was enabled by default)
        - `atomic64` (was enabled by default)

## Version 0.9.6

### Added

- Move the relevant source code from the GitHub moka-rs/moka repository (at
  [v0.9.6][moka-v0.9.6] tag) to this moka-rs/mini-moka repository.
    - Rename `moka::dash` module to `mini_moka::sync`.
    - Rename `moka::unsync` module to `mini_moka::unsync`.
    - Rename a crate feature `dash` to `sync` and make it a default.

<!-- Links -->
[moka-v0.9.6]: https://github.com/moka-rs/moka/tree/v0.9.6

[gh-issue-0021]: https://github.com/moka-rs/mini-moka/issues/21/

[gh-pull-0015]: https://github.com/moka-rs/mini-moka/pull/15/
[gh-pull-0006]: https://github.com/moka-rs/mini-moka/pull/6/
[gh-pull-0005]: https://github.com/moka-rs/mini-moka/pull/5/
[gh-pull-0002]: https://github.com/moka-rs/mini-moka/pull/2/
[gh-pull-0001]: https://github.com/moka-rs/mini-moka/pull/1/
