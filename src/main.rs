#![cfg_attr(feature = "clippy", deny(clippy_pedantic))]

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
mod error;
#[macro_use]
mod verbose;

use arg::Config;
use error::Result;
use std::ffi::OsStr;
use std::fs::remove_file;
use std::process;
use structopt::StructOpt;
use walkdir::WalkDir;
use unwalk_base::Action;
use unwalk_gz::GzAction;

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
        let entry_path = entry.path();

        if entry_path.extension() == Some(OsStr::new("gz")) {
            v2!(config.verbose, "Processing {:?}", entry_path);
            GzAction::execute(entry_path);
            v3!(config.verbose, "Processed {:?}", entry_path);

            if config.delete {
                remove_file(entry_path)?;
                v1!(config.verbose, "Removed {:?}", entry_path);
            }
        } else {
            v3!(
                config.verbose,
                "Ignored {:?} because its extension is not '.gz'",
                entry_path
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
