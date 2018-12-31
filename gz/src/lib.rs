#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
#![deny(missing_debug_implementations, missing_docs, warnings)]

//! # unwalk-base
//!
//! Provide un-gzip implementation as the action on matching files.

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

    fn default_extensions() -> &'static [&'static str] {
        &["gz"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::io::{self, Write};
    use tempfile::NamedTempFile;

    fn write_compressed_bytes<B>(bytes: B) -> io::Result<Vec<u8>>
    where
        B: AsRef<[u8]>,
    {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(bytes.as_ref())?;
        encoder.finish()
    }

    fn write_tmpfile_bytes_wrapper<B>(bytes: B, compress: bool) -> ActionResult
    where
        B: AsRef<[u8]>,
    {
        let tmpfile = NamedTempFile::new();
        assert!(tmpfile.is_ok());
        let mut tmpfile = tmpfile.unwrap();

        let bytes = bytes.as_ref();

        let write_res = if compress {
            let compressed_bytes = write_compressed_bytes(bytes);
            assert!(compressed_bytes.is_ok());
            let compressed_bytes = compressed_bytes.unwrap();

            tmpfile.write_all(&compressed_bytes)
        } else {
            tmpfile.write_all(bytes)
        };

        assert!(write_res.is_ok());

        let path = tmpfile.into_temp_path();
        let exec_res = GzAction::execute(&path);

        let close_res = path.close();
        assert!(close_res.is_ok());

        exec_res
    }

    #[test]
    fn test_gz_action_execute_ok_1() {
        let exec_res = write_tmpfile_bytes_wrapper(b"", true);
        assert!(exec_res.is_ok());
    }

    #[test]
    fn test_gz_action_execute_ok_2() {
        let exec_res = write_tmpfile_bytes_wrapper(b"Hello World!", true);
        assert!(exec_res.is_ok());
    }

    #[test]
    fn test_gz_action_execute_fail_1() {
        let exec_res = write_tmpfile_bytes_wrapper(b"", false);
        assert!(exec_res.is_err());
    }

    #[test]
    fn test_gz_action_execute_fail_2() {
        let exec_res = write_tmpfile_bytes_wrapper(b"Hello World!", false);
        assert!(exec_res.is_err());
    }
}
