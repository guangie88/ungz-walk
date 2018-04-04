use std::path::PathBuf;

/// CLI arguments structure
#[derive(Serialize, StructOpt, Debug)]
#[structopt(name = "Unwalk",
            about = "Perform action (e.g. un-gzip) on matching paths recursively")]
pub struct Config {
    #[structopt(parse(from_os_str))]
    /// Root path to start performing action recursively
    pub path: PathBuf,

    /// Delete matching file/dir after performing action
    #[structopt(short = "d", long = "delete")]
    pub delete: bool,

    /// Verbose mode (-v, -vv, -vvv)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: u8,
}
