// once `clap` updates, this can be removed
// see <https://github.com/kbknapp/clap-rs/pull/1242>
#![allow(deprecated)]

use std::path::PathBuf;

/// CLI arguments structure
#[derive(Serialize, StructOpt, Debug)]
#[structopt(name = "Unwalk",
            about = "Perform action (e.g. un-gzip) on matching paths recursively")]
pub struct Config {
    #[structopt(parse(from_os_str))]
    /// Root path to start performing action recursively
    pub path: PathBuf,

    #[structopt(short = "m", long = "mode",
                raw(possible_values = "&Mode::variants()",
                    case_insensitive = "true"),
                default_value = "gz")]
    /// Action mode to perform
    pub mode: Mode,

    #[structopt(short = "d", long = "delete")]
    /// Delete matching file/dir after performing action
    pub delete: bool,

    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    /// Verbose mode (-v, -vv, -vvv)
    pub verbose: u8,
}

/// Action mode to perform
arg_enum! {
    #[derive(Serialize, StructOpt, PartialEq, Debug)]
    pub enum Mode {
        Gz,
    }
}
