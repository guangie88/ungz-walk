// TODO: this is required if action type is enabled
// once `clap` updates, this can be removed
// see <https://github.com/kbknapp/clap-rs/pull/1242>
// #![allow(deprecated)]

use std::path::PathBuf;
use structopt::StructOpt;

/// CLI arguments structure
#[derive(StructOpt, Debug)]
#[structopt(
    name = "Unwalk",
    about = "Perform action (e.g. un-gzip) on matching paths recursively"
)]
pub struct Config {
    #[structopt(parse(from_os_str))]
    /// Root path to start performing action recursively
    pub path: PathBuf,

    // #[structopt(short = "t", long = "type",
    //             raw(possible_values = "&Type::variants()",
    //                 case_insensitive = "true"),
    //             default_value = "Gz")]
    // /// Action type to perform
    // pub typ: Type,

    // TODO: not implemented at the moment
    // #[structopt(short = "m", long = "match")]
    /// File extension(s) match to provide
    // pub ext_matches: Vec<String>,

    #[structopt(short = "d", long = "delete")]
    /// Delete matching file/dir after performing action
    pub delete: bool,

    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    /// Verbose mode (-v, -vv, -vvv)
    pub verbose: u8,
}

// /// Action type to perform
// arg_enum! {
//     #[derive(Serialize, StructOpt, PartialEq, Debug)]
//     pub enum Type {
//         Gz,
//     }
// }
