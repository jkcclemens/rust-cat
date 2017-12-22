#![feature(box_syntax)]

extern crate cat;
extern crate structopt;

use structopt::StructOpt;
use cat::{Cli, Output};
use std::io::{Read, BufReader, BufWriter, stdin, stdout};
use std::mem::uninitialized;

// https://github.com/coreutils/coreutils/blob/master/src/ioblksize.h
const BUFSIZE: usize = 128 * 1024;

fn main() {
  let code = match inner() {
    Ok(_) => 0,
    Err(e) => {
      println!("cat: {}", e);
      1
    }
  };
  std::process::exit(code);
}

fn inner() -> cat::Result<()> {
  let mut cli = Cli::from_args();

  if cli.files.is_empty() {
    cli.files.push("-".into());
  }

  let (stdin, stdout) = (stdin(), stdout());
  let mut stdin_lock = stdin.lock();
  let mut lock = BufWriter::with_capacity(BUFSIZE, stdout.lock());

  let mut buf: [u8; BUFSIZE] = unsafe { uninitialized() };
  for file in &cli.files {
    let mut f: BufReader<Box<Read>> = BufReader::with_capacity(BUFSIZE, if file == "-" {
      box &mut stdin_lock
    } else {
      box cat::open(file)?
    });
    loop {
      match f.read(&mut buf) {
        Ok(0) => break,
        Ok(i) => {
          Output::new(&cli, &buf[..i]).write(&mut lock)?;
        },
        Err(e) => return Err(e.into())
      }
    }
  }

  Ok(())
}
