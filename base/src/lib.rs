#![cfg_attr(feature = "clippy", deny(clippy_pedantic))]

use std::path::Path;

pub trait Action {
    fn execute<P>(path: P)
    where
        P: AsRef<Path>;
}
