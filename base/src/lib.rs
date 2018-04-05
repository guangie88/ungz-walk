#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![deny(missing_debug_implementations, missing_docs, warnings)]

//! # unwalk-base
//!
//! Provide basic trait for action on matching files.

#[macro_use]
extern crate failure;

pub mod error;

use error::Error;
use std::path::Path;

/// Alias to Result type of performing Action.
pub type ActionResult = Result<(), Error>;

/// Trait for action on matching files.
pub trait Action {
    /// Perform action on given matching file.
    fn execute<P>(path: P) -> ActionResult
    where
        P: AsRef<Path>;
}
