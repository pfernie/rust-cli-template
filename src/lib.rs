#![allow(unused_imports)]
#![deny(warnings, missing_debug_implementations, rust_2018_idioms)]

use thiserror::Error;
use tracing::{event, info, instrument, span, warn, Level};
#[cfg(feature = "traced-error")]
use tracing_error::{prelude::*, TracedError};

#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown error")]
    Unknown,
}

#[cfg(feature = "traced-error")]
type Result<T, E = TracedError<Error>> = color_eyre::Result<T, E>;
#[cfg(not(feature = "traced-error"))]
type Result<T, E = Error> = color_eyre::Result<T, E>;

#[cfg(feature = "traced-error")]
macro_rules! err {
    ($e:expr) => {
        Err($e).in_current_span()
    };
}
#[cfg(not(feature = "traced-error"))]
macro_rules! err {
    ($e:expr) => {
        Err($e)
    };
}

/// The string literal `"hello, world!"`
/// ```
/// use {{crate_name}}::hello;
/// assert_eq!(hello(), "hello, world!");
/// ```
pub fn hello() -> &'static str {
    "hello, world!"
}

/// A fn that will simply Err
pub fn will_err() -> Result<()> {
    err!(Error::Unknown)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn hello_test() {
        assert_eq!(hello(), "hello, world!");
    }
}
