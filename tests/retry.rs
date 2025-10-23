use maybe_backoff::Error;
use maybe_backoff::ExponentialBackoff;

use std::io;

#[test]
fn retry() {
    let mut i = 0;
    let success_on = 3;

    {
        let f = || -> Result<(), Error<io::Error>> {
            i += 1;
            if i == success_on {
                return Ok(());
            }

            Err(Error::Transient {
                err: io::Error::new(io::ErrorKind::Other, "err"),
                retry_after: None,
            })
        };

        let backoff = ExponentialBackoff::default();
        maybe_backoff::retry(backoff, f).ok().unwrap();
    }

    assert_eq!(i, success_on);
}

#[test]
fn permanent_error_immediately_returned() {
    let f = || -> Result<(), Error<io::Error>> {
        Err(Error::Permanent(io::Error::new(
            io::ErrorKind::Other,
            "err",
        )))
    };

    let backoff = ExponentialBackoff::default();
    match maybe_backoff::retry(backoff, f).err().unwrap() {
        Error::Permanent(_) => (),
        other => panic!("{}", other),
    }
}
