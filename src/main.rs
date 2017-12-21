extern crate cat;
extern crate structopt;

use structopt::StructOpt;
use cat::{Cli, Output};
use std::io::{BufRead, stdin};

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
  let stdin = stdin();
  let mut lock = stdin.lock();
  loop {
    let mut line = Vec::new();
    let read = lock.read_until(b'\n', &mut line)?;
    if read == 0 {
      break;
    }
    Output::new(cli, vec![line].into_iter()).write_to_stdout()?;
  }
  Ok(())
}

fn inner() -> cat::Result<()> {
  let cli = Cli::from_args();

  if cli.files.is_empty() {
    return handle_stdin(&cli);
  }

  // FIXME(perf): process at same time as read, don't store contents and then process

  let files: cat::Result<Vec<Vec<u8>>> = cli.files.iter().map(cat::read_bytes).collect();
  let files = files?;

  for file in files {
    Output::new(&cli, file.split(|&x| x == b'\n')).write_to_stdout()?;
  }

  Ok(())
}
