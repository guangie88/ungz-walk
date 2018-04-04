#![cfg_attr(feature = "clippy", deny(clippy_pedantic))]

extern crate file;
extern crate filebuffer;
extern crate flate2;
extern crate unwalk_base;

use filebuffer::FileBuffer;
use flate2::read::GzDecoder;
use std::io::Read;
use std::path::Path;
use unwalk_base::Action;

pub struct GzAction;

impl Action for GzAction {
    fn execute<P>(path: P)
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let buf = FileBuffer::open(path).unwrap();
        let mut decoder = GzDecoder::new(buf.as_ref());

        let mut s = String::new();
        decoder.read_to_string(&mut s).unwrap();

        let output_path = path.with_extension("");
        file::put(&output_path, s.as_bytes()).unwrap();
    }
}
