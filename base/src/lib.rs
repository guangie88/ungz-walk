#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
#![deny(missing_debug_implementations, missing_docs, warnings)]

//! # unwalk-base
//!
//! Provide basic trait for action on matching files.

pub mod error;

use crate::error::Error;
use std::path::Path;

/// Alias to Result type of performing Action.
pub type ActionResult = Result<(), Error>;

/// Trait for action on matching files.
pub trait Action {
    /// Perform action on given matching file.
    fn execute<P>(path: P) -> ActionResult
    where
        P: AsRef<Path>;

    /// Default file name extension(s) to match to perform the action.
    fn default_extensions() -> &'static [&'static str];
}
