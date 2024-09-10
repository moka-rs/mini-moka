use std::{
    sync::{Arc, RwLock},
    time::Instant as StdInstant,
};

#[cfg(test)]
use std::time::Duration;

use crate::common::typesize_helpers::MaybeOwnedArc;

pub(crate) type Instant = StdInstant;

#[cfg_attr(feature = "typesize", derive(typesize::derive::TypeSize))]
pub(crate) struct Clock {
    mock: Option<MaybeOwnedArc<Mock>>,
}

impl Clock {
    #[cfg(test)]
    pub(crate) fn mock() -> (Clock, MaybeOwnedArc<Mock>) {
        let mock = MaybeOwnedArc::new(Mock::default());
        let clock = Clock {
            mock: Some(MaybeOwnedArc::clone(&mock)),
        };
        (clock, mock)
    }

    pub(crate) fn now(&self) -> Instant {
        if let Some(mock) = &self.mock {
            *mock.now.read().expect("lock poisoned")
        } else {
            StdInstant::now()
        }
    }
}

#[cfg_attr(feature = "typesize", derive(typesize::derive::TypeSize))]
pub(crate) struct Mock {
    now: RwLock<Instant>,
}

impl Default for Mock {
    fn default() -> Self {
        Self {
            now: RwLock::new(StdInstant::now()),
        }
    }
}

#[cfg(test)]
impl Mock {
    pub(crate) fn increment(&self, amount: Duration) {
        *self.now.write().expect("lock poisoned") += amount;
    }
}
