#[cfg(not(target_family = "wasm"))]
use std::time::Instant;
#[cfg(target_family = "wasm")]
use web_time::Instant;

/// Clock returns the current time.
pub trait Clock {
    fn now(&self) -> Instant;
}

/// `SystemClock` uses the system's clock to get the current time.
/// This Clock should be used for real use-cases.
#[derive(Debug, Default, Clone)]
pub struct SystemClock {}

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}
