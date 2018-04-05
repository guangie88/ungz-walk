#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![deny(missing_debug_implementations, missing_docs, warnings)]

//! # unwalk-base
//!
//! Provide un-gzip implementation as the action on matching files.

extern crate file;
extern crate filebuffer;
extern crate flate2;
extern crate unwalk_base;

use filebuffer::FileBuffer;
use flate2::read::GzDecoder;
use std::io::Read;
use std::path::Path;
use unwalk_base::{Action, ActionResult};

/// Implementation for un-gzip action.
#[derive(Debug)]
pub struct GzAction;

impl Action for GzAction {
    fn execute<P>(path: P) -> ActionResult
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let buf = FileBuffer::open(path)?;
        let mut decoder = GzDecoder::new(buf.as_ref());

        let mut s = String::new();
        decoder.read_to_string(&mut s)?;

        let output_path = path.with_extension("");
        file::put(&output_path, s.as_bytes())?;

        Ok(())
    }
}
