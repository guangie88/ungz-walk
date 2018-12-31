//! # unwalk
//!
//! Perform action on matching files/dirs during recursive walking of given
//! directory. The default action is to perform un-gzip, useful for unarchiving
//! all `.gz` files when copying files over from AWS S3 / HDFS services, which
//! also happens to be the original use case of this CLI application.

#![cfg_attr(feature = "cargo-clippy", deny(clippy::all))]
#![deny(missing_debug_implementations, missing_docs, warnings)]

mod arg;

use crate::arg::Config;
use failure::Fail;
use std::ffi::OsStr;
use std::fs::remove_file;
use std::process;
use structopt::StructOpt;
use unwalk_base::error::{ErrorKind, Result};
use unwalk_base::Action;
use unwalk_gz::GzAction;
use vlog::{set_verbosity_level, v1, v2, v3, ve1, verbose_elog, verbose_log};
use walkdir::WalkDir;

fn run(config: &Config) -> Result<()> {
    if !config.path.exists() {
        Err(ErrorKind::DirectoryNotFound)?;
    }

    let entries = WalkDir::new(&config.path)
        .into_iter()
        .inspect(|e| {
            if let Err(ref e) = *e {
                ve1!("{}", e);
            }
        })
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir());

    for entry in entries {
        let loop_fn = || -> Result<()> {
            let entry_path = entry.path();

            if entry_path.extension() == Some(OsStr::new("gz")) {
                v2!("Processing {:?}", entry_path);
                GzAction::execute(entry_path)?;
                v3!("Processed {:?}", entry_path);

                if config.delete {
                    remove_file(entry_path)?;
                    v1!("Removed {:?}", entry_path);
                }
            } else {
                v2!("Ignored {:?}, extension is not '.gz'", entry_path);
            }

            Ok(())
        };

        let loop_res = loop_fn();

        if let Err(e) = loop_res {
            eprintln!("{}", e);
        }
    }

    Ok(())
}

fn main() {
    let config = Config::from_args();
    set_verbosity_level(config.verbose as usize);

    match run(&config) {
        Ok(_) => v1!("Unwalk complete!"),
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
