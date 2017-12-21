extern crate structopt;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate failure;
extern crate tabwriter;
extern crate itertools;
#[macro_use]
extern crate if_chain;

use failure::{Error, ResultExt};

pub mod cli;
pub mod error;
pub mod output;

pub use cli::Cli;
pub use error::CatError;
pub use output::Output;

pub type Result<T> = std::result::Result<T, Error>;

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn open<P: AsRef<Path>>(path: P) -> Result<File> {
  Ok(File::open(path)?)
}

fn read_file(mut f: File) -> Result<Vec<u8>> {
  let mut contents = Vec::new();
  f.read_to_end(&mut contents)?;
  Ok(contents)
}

pub fn read_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
  let path = path.as_ref();
  let name = path.to_string_lossy().into_owned();
  Ok(open(path).and_then(|f| read_file(f)).with_context(|e| format!("{}: {}", name, e))?)
}
