fn main() {
    match example::run("https::///wrong URL") {
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
    use maybe_backoff::{Error, ExponentialBackoff};
    use reqwest::Url;

    use std::fmt::Display;
    use std::io::{self, Read};

    fn new_io_err<E: Display>(err: E) -> io::Error {
        io::Error::other(err.to_string())
    }

    pub fn run(url: &str) -> Result<String, Error<io::Error>> {
        let op = || {
            println!("Fetching {}", url);
            let url = Url::parse(url)
                .map_err(new_io_err)
                // Permanent errors need to be explicitly constructed.
                .map_err(Error::Permanent)?;

            let mut resp = reqwest::blocking::get(url)
                // Transient errors can be constructed with the ? operator
                // or with the try! macro. No explicit conversion needed
                // from E: Error to backoff::Error;
                .map_err(new_io_err)?;

            let mut content = String::new();
            let _ = resp.read_to_string(&mut content);
            Ok(content)
        };

        let backoff = ExponentialBackoff::default();
        maybe_backoff::retry(backoff, op)
    }
}
