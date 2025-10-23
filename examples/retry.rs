fn main() {
    match example::run("https://www.rust-lang.org") {
        Ok(_) => println!("Successfully fetched"),
        Err(err) => panic!("Failed to fetch: {}", err),
    }
}

#[cfg(target_arch = "wasm32")]
mod example {
    pub fn run(_url: &str) -> Result<String, maybe_backoff::Error<std::io::Error>> {
        unimplemented!("This example is not implemented for wasm32 target");
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod example {

    use maybe_backoff::{retry, Error, ExponentialBackoff};

    use std::io::Read;

    pub fn run(url: &str) -> Result<String, Error<reqwest::Error>> {
        let op = || {
            println!("Fetching {}", url);
            let mut resp = reqwest::blocking::get(url)?;

            let mut content = String::new();
            let _ = resp.read_to_string(&mut content);
            Ok(content)
        };

        let backoff = ExponentialBackoff::default();
        retry(backoff, op)
    }
}
