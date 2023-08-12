use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    Mutex,
};

use super::{AccessTime, KeyDeqNodeAo, KeyDeqNodeWo};
use crate::common::{concurrent::atomic_time::AtomicInstant, time::Instant};

pub(crate) struct DeqNodes<K> {
    access_order_q_node: Option<KeyDeqNodeAo<K>>,
    write_order_q_node: Option<KeyDeqNodeWo<K>>,
}

// We need this `unsafe impl` as DeqNodes have NonNull pointers.
unsafe impl<K> Send for DeqNodes<K> {}

pub(crate) struct EntryInfo<K> {
    /// `is_admitted` indicates that the entry has been admitted to the
    /// cache. When `false`, it means the entry is _temporary_ admitted to
    /// the cache or evicted from the cache (so it should not have LRU nodes).
    is_admitted: AtomicBool,
    /// `is_dirty` indicates that the entry has been inserted (or updated)
    /// in the hash table, but the history of the insertion has not yet
    /// been applied to the LRU deques and LFU estimator.
    is_dirty: AtomicBool,
    last_accessed: AtomicInstant,
    last_modified: AtomicInstant,
    policy_weight: AtomicU32,
    nodes: Mutex<DeqNodes<K>>,
}

impl<K> EntryInfo<K> {
    #[inline]
    pub(crate) fn new(timestamp: Instant, policy_weight: u32) -> Self {
        Self {
            is_admitted: Default::default(),
            is_dirty: AtomicBool::new(true),
            last_accessed: AtomicInstant::new(timestamp),
            last_modified: AtomicInstant::new(timestamp),
            policy_weight: AtomicU32::new(policy_weight),
            nodes: Mutex::new(DeqNodes {
                access_order_q_node: None,
                write_order_q_node: None,
            }),
        }
    }

    #[inline]
    pub(crate) fn is_admitted(&self) -> bool {
        self.is_admitted.load(Ordering::Acquire)
    }

    #[inline]
    pub(crate) fn set_admitted(&self, value: bool) {
        self.is_admitted.store(value, Ordering::Release);
    }

    #[inline]
    pub(crate) fn is_dirty(&self) -> bool {
        self.is_dirty.load(Ordering::Acquire)
    }

    #[inline]
    pub(crate) fn set_dirty(&self, value: bool) {
        self.is_dirty.store(value, Ordering::Release);
    }

    #[inline]
    pub(crate) fn policy_weight(&self) -> u32 {
        self.policy_weight.load(Ordering::Acquire)
    }

    #[inline]
    pub(crate) fn set_policy_weight(&self, size: u32) {
        self.policy_weight.store(size, Ordering::Release);
    }

    #[inline]
    pub(crate) fn access_order_q_node(&self) -> Option<KeyDeqNodeAo<K>> {
        self.nodes
            .lock()
            .expect("lock poisoned")
            .access_order_q_node
    }

    #[inline]
    pub(crate) fn set_access_order_q_node(&self, node: Option<KeyDeqNodeAo<K>>) {
        self.nodes
            .lock()
            .expect("lock poisoned")
            .access_order_q_node = node;
    }

    #[inline]
    pub(crate) fn take_access_order_q_node(&self) -> Option<KeyDeqNodeAo<K>> {
        self.nodes
            .lock()
            .expect("lock poisoned")
            .access_order_q_node
            .take()
    }

    #[inline]
    pub(crate) fn write_order_q_node(&self) -> Option<KeyDeqNodeWo<K>> {
        self.nodes.lock().expect("lock poisoned").write_order_q_node
    }

    #[inline]
    pub(crate) fn set_write_order_q_node(&self, node: Option<KeyDeqNodeWo<K>>) {
        self.nodes.lock().expect("lock poisoned").write_order_q_node = node;
    }

    #[inline]
    pub(crate) fn take_write_order_q_node(&self) -> Option<KeyDeqNodeWo<K>> {
        self.nodes
            .lock()
            .expect("lock poisoned")
            .write_order_q_node
            .take()
    }

    #[inline]
    pub(crate) fn unset_q_nodes(&self) {
        let mut nodes = self.nodes.lock().expect("lock poisoned");
        nodes.access_order_q_node = None;
        nodes.write_order_q_node = None;
    }
}

impl<K> AccessTime for EntryInfo<K> {
    #[inline]
    fn last_accessed(&self) -> Option<Instant> {
        self.last_accessed.instant()
    }

    #[inline]
    fn set_last_accessed(&self, timestamp: Instant) {
        self.last_accessed.set_instant(timestamp);
    }

    #[inline]
    fn last_modified(&self) -> Option<Instant> {
        self.last_modified.instant()
    }

    #[inline]
    fn set_last_modified(&self, timestamp: Instant) {
        self.last_modified.set_instant(timestamp);
    }
}
