#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", deny(warnings))]

extern crate failure;
extern crate file;
extern crate filebuffer;
extern crate flate2;
extern crate log4rs;
#[macro_use]
extern crate log;
extern crate simple_logger;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate walkdir;

use failure::Error;
use filebuffer::FileBuffer;
use flate2::read::GzDecoder;
use std::ffi::OsStr;
use std::io::Read;
use std::process;
use structopt::StructOpt;
use walkdir::WalkDir;

/// Main program configuration structure
#[derive(StructOpt, Debug)]
#[structopt(name = "Un-GZip Walker",
            about = "To perform un-gzip of multiple files contained in directory")]
struct ArgConfig {
    /// Log configuration file path
    #[structopt(short = "l", long = "log", help = "Log configuration file path")]
    log_config_path: Option<String>,

    /// Glob pattern to apply for matching of files for acceptance
    #[structopt(short = "f", long = "from",
                help = "From root directory to start the un-gzip recursively")]
    from_dir: String,
}

type Result<T> = std::result::Result<T, Error>;

fn run() -> Result<()> {
    let args = ArgConfig::from_args();

    if let Some(log_config_path) = args.log_config_path {
        log4rs::init_file(log_config_path, Default::default())?;
    } else {
        simple_logger::init()?;
    }

    for entry in WalkDir::new(&args.from_dir) {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.extension() == Some(OsStr::new("gz")) {
                    let buf = FileBuffer::open(path)?;
                    let mut decoder = GzDecoder::new(buf.as_ref());

                    let mut s = String::new();
                    decoder.read_to_string(&mut s)?;

                    let output_file_path = path.with_extension("");
                    file::put(&output_file_path, s.as_bytes())?;
                } else {
                    debug!("IGNORE {:?} because its extension is not '.gz'", path);
                }
            }

            Err(e) => error!("DirEntry ERROR: {}", e),
        }
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => info!("Program completed!"),
        Err(e) => {
            error!("ERROR: {}\n > BACKTRACE: {}", e.cause(), e.backtrace());
            process::exit(1);
        }
    }
}
