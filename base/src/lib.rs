#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![deny(missing_docs, warnings)]

//! # unwalk-base
//!
//! Provide basic trait for action on matching files.

use std::path::Path;

/// Trait for action on matching files.
pub trait Action {
    /// Perform action on given matching file.
    fn execute<P>(path: P)
    where
        P: AsRef<Path>;
}
