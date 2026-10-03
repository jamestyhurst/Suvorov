use std::fmt;

/// Errors that match the C++ slice's `std::invalid_argument` / `std::out_of_range` split.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidArgument(&'static str),
    OutOfRange(&'static str),
    FeatureDisabled(&'static str),
    Script(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidArgument(msg) | Error::OutOfRange(msg) | Error::FeatureDisabled(msg) => {
                f.write_str(msg)
            }
            Error::Script(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
