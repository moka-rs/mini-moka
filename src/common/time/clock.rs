use std::{
    sync::{Arc, RwLock},
    time::Instant as StdInstant,
};

#[cfg(test)]
use std::time::Duration;

pub(crate) type Instant = StdInstant;

pub(crate) struct Clock {
    mock: Option<Arc<Mock>>,
}

impl Clock {
    #[cfg(test)]
    pub(crate) fn mock() -> (Clock, Arc<Mock>) {
        let mock = Arc::new(Mock::default());
        let clock = Clock {
            mock: Some(Arc::clone(&mock)),
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
