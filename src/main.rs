#![feature(box_syntax)]

extern crate cat;
extern crate structopt;

use structopt::StructOpt;
use cat::{Cli, Output};
use std::io::{Read, BufRead, BufReader, BufWriter, stdin, stdout};
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

fn handle_stdin(cli: &Cli) -> cat::Result<()> {
  let (stdin, stdout) = (stdin(), stdout());
  let mut lock = BufReader::with_capacity(BUFSIZE, stdin.lock());
  let mut stdout_lock = BufWriter::with_capacity(BUFSIZE, stdout.lock());
  loop {
    let mut line = Vec::new();
    if lock.read_until(b'\n', &mut line)? == 0 {
      break;
    }
    Output::new(cli, &line).write(&mut stdout_lock)?;
  }
  Ok(())
}

fn inner() -> cat::Result<()> {
  let cli = Cli::from_args();

  if cli.files.is_empty() {
    return handle_stdin(&cli);
  }

  // FIXME(perf): process at same time as read, don't store contents and then process

  let (stdin, stdout) = (stdin(), stdout());
  let mut stdin_lock = stdin.lock();
  let mut lock = BufWriter::with_capacity(BUFSIZE, stdout.lock());

  for file in &cli.files {
    let mut buf: [u8; BUFSIZE] = unsafe { uninitialized() };
    let mut f: Box<Read> = if file == "-" {
      box BufReader::with_capacity(BUFSIZE, &mut stdin_lock)
    } else {
      box BufReader::with_capacity(BUFSIZE, cat::open(file)?)
    };
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
