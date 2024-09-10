use super::{
    atomic_time::AtomicInstant,
    constants::{
        MAX_SYNC_REPEATS, PERIODICAL_SYNC_INTERVAL_MILLIS, READ_LOG_FLUSH_POINT,
        WRITE_LOG_FLUSH_POINT,
    },
};

use crate::common::time::{CheckedTimeOps, Instant};

use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

pub(crate) trait InnerSync {
    fn sync(&self, max_sync_repeats: usize);
    fn now(&self) -> Instant;
}

#[cfg_attr(feature = "typesize", derive(typesize::derive::TypeSize))]
pub(crate) struct Housekeeper {
    is_sync_running: AtomicBool,
    sync_after: AtomicInstant,
}

impl Default for Housekeeper {
    fn default() -> Self {
        Self {
            is_sync_running: Default::default(),
            sync_after: AtomicInstant::new(Self::sync_after(Instant::now())),
        }
    }
}

impl Housekeeper {
    pub(crate) fn should_apply_reads(&self, ch_len: usize, now: Instant) -> bool {
        self.should_apply(ch_len, READ_LOG_FLUSH_POINT, now)
    }

    pub(crate) fn should_apply_writes(&self, ch_len: usize, now: Instant) -> bool {
        self.should_apply(ch_len, WRITE_LOG_FLUSH_POINT, now)
    }

    #[inline]
    pub(crate) fn should_apply(&self, ch_len: usize, ch_flush_point: usize, now: Instant) -> bool {
        ch_len >= ch_flush_point || self.sync_after.instant().unwrap() >= now
    }

    pub(crate) fn try_sync<T: InnerSync>(&self, cache: &T) -> bool {
        // Try to flip the value of sync_scheduled from false to true.
        match self.is_sync_running.compare_exchange(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                let now = cache.now();
                self.sync_after.set_instant(Self::sync_after(now));

                cache.sync(MAX_SYNC_REPEATS);

                self.is_sync_running.store(false, Ordering::Release);
                true
            }
            Err(_) => false,
        }
    }

    fn sync_after(now: Instant) -> Instant {
        let dur = Duration::from_millis(PERIODICAL_SYNC_INTERVAL_MILLIS);
        let ts = now.checked_add(dur);
        // Assuming that `now` is current wall clock time, this should never fail at
        // least next millions of years.
        ts.expect("Timestamp overflow")
    }
}
