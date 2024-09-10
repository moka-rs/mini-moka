use super::Instant;

use std::sync::RwLock;

#[cfg_attr(feature = "typesize", derive(typesize::derive::TypeSize))]
pub(crate) struct AtomicInstant {
    instant: RwLock<Option<Instant>>,
}

impl Default for AtomicInstant {
    fn default() -> Self {
        Self {
            instant: RwLock::new(None),
        }
    }
}

impl AtomicInstant {
    pub(crate) fn new(timestamp: Instant) -> Self {
        let ai = Self::default();
        ai.set_instant(timestamp);
        ai
    }

    pub(crate) fn is_set(&self) -> bool {
        self.instant.read().expect("lock poisoned").is_some()
    }

    pub(crate) fn instant(&self) -> Option<Instant> {
        *self.instant.read().expect("lock poisoned")
    }

    pub(crate) fn set_instant(&self, instant: Instant) {
        *self.instant.write().expect("lock poisoned") = Some(instant);
    }
}
