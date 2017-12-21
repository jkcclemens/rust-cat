use std::io;

#[derive(Debug, Fail)]
pub enum CatError {
  #[fail(display = "An IO error occurred: {}", _0)]
  Io(io::Error)
}
