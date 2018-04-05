//! # unwalk
//!
//! Perform action on matching files/dirs during recursive walking of given
//! directory. The default action is to perform un-gzip, useful for unarchiving
//! all `.gz` files when copying files over from AWS S3 / HDFS services, which
//! also happens to be the original use case of this CLI application.

#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![deny(missing_debug_implementations, missing_docs, warnings)]

#[macro_use]
extern crate clap;
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate toml;
extern crate unwalk_base;
extern crate unwalk_gz;
extern crate walkdir;

mod arg;
#[macro_use]
mod verbose;

use arg::Config;
use failure::Fail;
use std::ffi::OsStr;
use std::fs::remove_file;
use std::process;
use structopt::StructOpt;
use unwalk_base::Action;
use unwalk_base::error::{ErrorKind, Result};
use unwalk_gz::GzAction;
use walkdir::WalkDir;

fn run(config: &Config) -> Result<()> {
    if !config.path.exists() {
        Err(ErrorKind::DirectoryNotFound)?;
    }

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
        let entry_path = entry.path();

        if entry_path.extension() == Some(OsStr::new("gz")) {
            v2!(config.verbose, "Processing {:?}", entry_path);
            GzAction::execute(entry_path)?;
            v3!(config.verbose, "Processed {:?}", entry_path);

            if config.delete {
                remove_file(entry_path)?;
                v1!(config.verbose, "Removed {:?}", entry_path);
            }
        } else {
            v2!(
                config.verbose,
                "Ignored {:?}, extension is not '.gz'",
                entry_path
            );
        }
    }

    Ok(())
}

fn main() {
    let config = Config::from_args();

    match run(&config) {
        Ok(_) => v1!(config.verbose, "Program completed!"),
        Err(e) => {
            eprintln!("{}", e);

            if let Some(cause) = e.cause() {
                eprintln!("Caused by: {}", cause);
            }

            if let Some(backtrace) = e.backtrace() {
                eprintln!("Backtrace: {}", backtrace);
            }

            process::exit(1);
        }
    }
}
