#![allow(unused_imports)]
#![deny(warnings, missing_debug_implementations, rust_2018_idioms)]

use color_eyre::eyre::{self, WrapErr};
use thiserror::Error;
use tracing::{event, info, instrument, span, warn, Level};
#[cfg(feature = "traced-error")]
use tracing_error::{prelude::*, TracedError};

#[derive(Debug, Error)]
pub struct Error {
    #[error("unknown error")]
    Unknown,
}

#[cfg(feature = "traced-error")]
type Result<T, E = TracedError<Error>> = color_eyre::Result<T, E>;
#[cfg(not(feature = "traced-error"))]
type Result<T, E = Error> = color_eyre::Result<T, E>;

macro_rules! err {
    ($e:expr) => {
        Err($e)
        #[cfg(feature = "traced-error")]
            .in_current_span()
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
