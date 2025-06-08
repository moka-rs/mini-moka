use std::sync::{Arc, RwLock};

#[cfg(test)]
use std::time::Duration;

#[cfg(not(feature = "js"))]
pub(crate) type Instant = std::time::Instant;

#[cfg(feature = "js")]
pub(crate) type Instant = web_time::Instant;

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
            Instant::now()
        }
    }
}

pub(crate) struct Mock {
    now: RwLock<Instant>,
}

impl Default for Mock {
    fn default() -> Self {
        Self {
            now: RwLock::new(Instant::now()),
        }
    }
}

#[cfg(test)]
impl Mock {
    pub(crate) fn increment(&self, amount: Duration) {
        *self.now.write().expect("lock poisoned") += amount;
    }
}
