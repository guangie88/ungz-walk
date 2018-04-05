//! # unwalk-base error module
//!
//! Provide all error related implementations.

use std;

/// Common error type for `unwalk`.
#[derive(Fail, Debug)]
pub enum Error {
    /// Error kind variant.
    #[fail(display = "Kind({})", _0)]
    Kind(ErrorKind),

    /// I/O error variant.
    #[fail(display = "Io({})", _0)]
    Io(#[cause] std::io::Error),

    /// Custom error variant. Requires dynamic allocation.
    #[fail(display = "Custom({})", _0)]
    Custom(#[cause] Custom),
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Error {
        Error::Kind(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<Custom> for Error {
    fn from(e: Custom) -> Error {
        Error::Custom(e)
    }
}

/// Custom error to include error kind and dynamic allocated error.
#[derive(Fail, Debug)]
#[fail(display = "{{ kind: {}, error: {} }}", kind, error)]
pub struct Custom {
    kind: ErrorKind,
    error: Box<std::error::Error + Sync + Send>,
}

impl Custom {
    /// Creates a new custom error with given error kind and actual error value.
    ///
    /// # Arguments
    /// * `kind` - Error kind to describe the error.
    /// * `error` - Actual error value in boxed form.
    pub fn new<E>(kind: ErrorKind, error: E) -> Custom
    where
        E: Into<Box<std::error::Error + Sync + Send>>,
    {
        Custom {
            kind,
            error: error.into(),
        }
    }
}

/// Represent the context of failure.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    /// Invalid format error kind.
    #[fail(display = "Invalid format")]
    InvalidFormat,

    /// Open file error kind.
    #[fail(display = "Open file")]
    OpenFile,
}

/// Alias to result type used within `unwalk`.
pub type Result<T> = std::result::Result<T, Error>;
