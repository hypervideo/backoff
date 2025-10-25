# maybe-backoff

Exponential backoff and retry. This is a fork of [backoff](https://github.com/ihrwein/backoff). The parent repo has been unmaintained for a while and this version provides up-to-date dependencies and a `MaybeBackoff` interface that streamlines conditional backoff usage.

---

Inspired by the retry mechanism in Google's [google-http-java-client](https://github.com/google/google-http-java-client) library and
its [Golang port](https://github.com/cenkalti/backoff).

[![CI](https://github.com/hypervideo/backoff/actions/workflows/ci.yml/badge.svg)](https://github.com/hypervideo/backoff/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/maybe-backoff)](https://crates.io/crates/maybe-backoff)
[![Documentation](https://docs.rs/maybe-backoff/badge.svg)](https://docs.rs/maybe-backoff)

Compile with feature `wasm-bindgen` or `stdweb` for use in WASM environments. `retry_notify` is not yet supported, as it uses `std::thread::sleep`.

:warning: **BREAKING CHANGES**: migration instructions under [Breaking changes](#breaking-changes).

## Overview

`backoff` is small crate which allows you to retry operations according to backoff policies. It provides:

- Error type to wrap errors as either transient of permanent,
- different backoff algorithms, including exponential,
- supporting both sync and async code.

## Sync example

Just wrap your fallible operation into a closure, and pass it into `retry`:

```rust
use backoff::{retry, ExponentialBackoff, Error};

let op = || {
    reqwest::blocking::get("http://example.com").map_err(Error::transient)
};

let _ = retry(&mut ExponentialBackoff::default(), op);
```

The retry policy will use jitters according to the `randomization_factor` field of `ExponentialBackoff`. Check the documentation for more parameters.

## Async example

Futures are supported by the `futures` module:

```rust
use backoff::ExponentialBackoff;
use backoff::future::retry;

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    retry(ExponentialBackoff::default(), || async {
        println!("Fetching {}", url);
        Ok(reqwest::get(url).await?.text().await?)
    })
    .await
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the Work by You, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
