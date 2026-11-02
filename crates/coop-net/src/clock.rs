//! Millisecond clocks. Everything in this crate takes time from a [`Clock`] so
//! tests can drive liveness (1 s keep-alive, 20 s timeout) without sleeping.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// A manually advanced clock handle; clones share the same time.
#[derive(Clone, Debug, Default)]
pub struct ManualClock(Arc<AtomicU64>);

impl ManualClock {
    /// A clock at t = 0.
    pub fn new() -> Self {
        Self::default()
    }
    /// Current time in ms.
    pub fn now(&self) -> u64 {
        self.0.load(Ordering::Relaxed)
    }
    /// Moves time forward.
    pub fn advance(&self, ms: u64) {
        self.0.fetch_add(ms, Ordering::Relaxed);
    }
}

/// Time source for an [`crate::Endpoint`].
#[derive(Clone, Debug)]
pub enum Clock {
    /// Wall clock, ms since this clock was created.
    System(Instant),
    /// Test clock.
    Manual(ManualClock),
}

impl Clock {
    /// A wall clock starting now.
    pub fn system() -> Self {
        Clock::System(Instant::now())
    }
    /// Current time in ms.
    pub fn now(&self) -> u64 {
        match self {
            Clock::System(t) => t.elapsed().as_millis() as u64,
            Clock::Manual(m) => m.now(),
        }
    }
}
