use crate::{backoff::Backoff as _, ExponentialBackoff, ExponentialBackoffBuilder};
use std::time::Duration;

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
