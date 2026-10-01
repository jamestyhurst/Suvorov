use std::fmt;

/// Errors that match the C++ slice's `std::invalid_argument` / `std::out_of_range` split.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidArgument(&'static str),
    OutOfRange(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidArgument(msg) | Error::OutOfRange(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
