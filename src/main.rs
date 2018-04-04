#![cfg_attr(feature = "clippy", deny(clippy_pedantic))]

extern crate failure;
extern crate file;
extern crate filebuffer;
extern crate flate2;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate toml;
extern crate walkdir;

mod arg;
mod error;
#[macro_use]
mod verbose;

use arg::Config;
use error::Result;
use filebuffer::FileBuffer;
use flate2::read::GzDecoder;
use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::Read;
use std::process;
use structopt::StructOpt;
use walkdir::WalkDir;

fn run(config: &Config) -> Result<()> {
    let entries = WalkDir::new(&config.path)
        .into_iter()
        .inspect(|e| {
            if let Err(ref e) = *e {
                ve1!(config.verbose, "{}", e);
            }
        })
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir());

    for entry in entries {
        let input_path = entry.path();

        if input_path.extension() == Some(OsStr::new("gz")) {
            v2!(config.verbose, "Processing {:?}", input_path);

            let buf = FileBuffer::open(input_path)?;
            let mut decoder = GzDecoder::new(buf.as_ref());

            let mut s = String::new();
            decoder.read_to_string(&mut s)?;

            let output_path = input_path.with_extension("");
            file::put(&output_path, s.as_bytes())?;

            v3!(config.verbose, "Processed {:?}", input_path);

            if config.delete {
                remove_file(input_path)?;
                v1!(config.verbose, "Removed {:?}", input_path);
            }
        } else {
            v3!(
                config.verbose,
                "Ignored {:?} because its extension is not '.gz'",
                input_path
            );
        }
    }

    Ok(())
}

fn main() {
    let config = Config::from_args();

    match run(&config) {
        Ok(_) => v2!(config.verbose, "Program completed!"),
        Err(e) => {
            eprintln!("{}\n > BACKTRACE: {}", e.cause(), e.backtrace());
            process::exit(1);
        }
    }
}
