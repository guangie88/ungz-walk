#![cfg_attr(feature = "clippy", deny(warnings))]

extern crate failure;
extern crate file;
extern crate filebuffer;
extern crate flate2;
extern crate log4rs;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate simple_logger;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate toml;
extern crate walkdir;

use failure::Error;
use filebuffer::FileBuffer;
use flate2::read::GzDecoder;
use std::ffi::OsStr;
use std::fs::remove_file;
use std::io::Read;
use std::process;
use structopt::StructOpt;
use toml::to_string_pretty;
use walkdir::WalkDir;

/// Argument configuration structure
#[derive(Serialize, StructOpt, Debug)]
#[structopt(name = "Un-gzip Walker",
            about = "To perform un-gzip of multiple files contained in directory")]
struct ArgConfig {
    /// Log configuration file path
    #[structopt(short = "l", long = "log", help = "Log configuration file path")]
    log_config_path: Option<String>,

    /// From root directory to start the un-gzip recursively
    #[structopt(short = "f", long = "from",
                help = "From root directory to start the un-gzip recursively")]
    from_dir: String,

    /// Delete .gz file after un-gzipping
    #[structopt(short = "x", help = "Do not delete .gz file after un-gzipping")]
    no_delete: bool,
}

type Result<T> = std::result::Result<T, Error>;

fn run() -> Result<()> {
    let args = ArgConfig::from_args();

    if let Some(ref log_config_path) = args.log_config_path {
        log4rs::init_file(log_config_path, Default::default())?;
    } else {
        simple_logger::init()?;
    }

    debug!("```\n{}```", to_string_pretty(&args)?);

    let entries = WalkDir::new(&args.from_dir)
        .into_iter()
        .inspect(|e| {
            if let Err(ref e) = *e {
                error!("DirEntry ERROR: {}", e);
            }
        })
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir());

    for entry in entries {
        let input_path = entry.path();

        if input_path.extension() == Some(OsStr::new("gz")) {
            let buf = FileBuffer::open(input_path)?;
            let mut decoder = GzDecoder::new(buf.as_ref());

            let mut s = String::new();
            decoder.read_to_string(&mut s)?;

            let output_path = input_path.with_extension("");
            file::put(&output_path, s.as_bytes())?;

            debug!("PROCESSED {:?}", input_path);

            if !args.no_delete {
                remove_file(input_path)?;
                debug!("> REMOVED {:?}", input_path);
            }
        } else {
            debug!("IGNORE {:?} because its extension is not '.gz'", input_path);
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
