#![feature(box_syntax)]
#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]

extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate failure;

use failure::{Error, ResultExt};

pub mod cli;
pub mod error;
pub mod output;

pub use cli::Cli;
pub use error::CatError;
pub use output::Output;

pub type Result<T> = std::result::Result<T, Error>;

use std::fs::File;
use std::path::Path;

pub fn open<P: AsRef<Path>>(path: P) -> Result<File> {
  Ok(File::open(path.as_ref())
    .with_context(|e| format!("{}: {}", path.as_ref().to_string_lossy(), e))?)
}
