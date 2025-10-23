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
    pub fn arm(&mut self) {
        if self.backoff.is_none() {
            self.backoff = Some(
                ExponentialBackoffBuilder::new()
                    .with_initial_interval(Duration::from_millis(50))
                    .with_max_interval(Duration::from_secs(3))
                    .with_multiplier(1.5)
                    .with_randomization_factor(0.2)
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
