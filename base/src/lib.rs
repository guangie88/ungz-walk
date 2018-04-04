#![cfg_attr(feature = "cargo-clippy", deny(clippy_pedantic, warnings))]

use std::path::Path;

pub trait Action {
    fn execute<P>(path: P)
    where
        P: AsRef<Path>;
}
