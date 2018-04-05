use failure::Error;
use std;

pub type Result<T> = std::result::Result<T, Error>;
