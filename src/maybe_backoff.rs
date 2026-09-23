use crate::{backoff::Backoff as _, ExponentialBackoff, ExponentialBackoffBuilder};
use std::time::Duration;

/// `MaybeBackoff` provides a simplified way to manage an optional exponential backoff while giving control over when to wait.
///
/// # Example
/// ```rust,no_run,ignore
/// let mut backoff = MaybeBackoff::default();
///
/// // Loop that runs fallible operation that should be retried with backoff.
/// loop {
///     backoff.sleep().await; // Does nothing when not armed (disarmed by default).
///     backoff.arm();
///
///     while let Some(event) = event_source.next().await {
///         match event {
///             Ok(Event::Open) => debug!("Connected!"),
///             Ok(Event::Message(event)) => match parse(event) {
///                 Ok(data) => {
///                     backoff.disarm();
///                     forward_data(data).await;
///                     break;
///                 }
///                 Err(error) => {
///                     error!("Parsing failed: {error:?}");
///                     event_source.close();
///                     continue;
///                 }
///             },
///             Err(error) => {
///                 error!("Event source failed: {error:?}");
///                 event_source.close();
///                 continue;
///             }
///         }
///     }
/// }
/// ```
#[derive(Default)]
pub struct MaybeBackoff {
    backoff: Option<ExponentialBackoff>,
}

impl MaybeBackoff {
    /// Starts the backoff unless it is already armed. An armed `MaybeBackoff`
    /// always waits and never runs out: unlike the `ExponentialBackoff`
    /// default, it has no 15-minute limit, so loops that retry forever keep
    /// waiting between attempts.
    pub fn arm(&mut self) {
        if self.backoff.is_none() {
            self.backoff = Some(
                ExponentialBackoffBuilder::new()
                    .with_initial_interval(Duration::from_millis(50))
                    .with_max_interval(Duration::from_secs(3))
                    .with_multiplier(1.5)
                    .with_randomization_factor(0.2)
                    .with_max_elapsed_time(None)
                    .build(),
            )
        }
    }

    pub fn disarm(&mut self) {
        self.backoff = None;
    }

    pub async fn sleep(&mut self) {
        if let Some(duration) = self.backoff.as_mut().and_then(|b| b.next_backoff()) {
            #[cfg(all(not(target_arch = "wasm32"), not(feature = "tokio")))]
            std::thread::sleep(duration);
            #[cfg(all(not(target_arch = "wasm32"), feature = "tokio"))]
            tokio_1::time::sleep(duration).await;
            #[cfg(target_arch = "wasm32")]
            gloo::timers::future::TimeoutFuture::new(duration.as_millis().try_into().unwrap())
                .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armed_backoff_keeps_waiting_after_fifteen_minutes() {
        let mut maybe_backoff = MaybeBackoff::default();
        maybe_backoff.arm();
        let backoff = maybe_backoff.backoff.as_mut().unwrap();
        // `ExponentialBackoff` stops returning delays after 15 minutes by default.
        backoff.start_time -= Duration::from_secs(16 * 60);
        assert!(backoff.next_backoff().is_some());
    }
}
