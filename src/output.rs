use cli::Cli;
use Result;

use std::io::{Write, stdout};
use std::marker::PhantomData;
use tabwriter::TabWriter;
use itertools::Itertools;

pub struct Output<'a, L, S> {
  cli: &'a Cli,
  lines: L,
  line_count: usize,
  empty: (bool, bool),
  _phantom: PhantomData<S>
}

impl<'a, L, S> Output<'a, L, S> {
  pub fn new(cli: &'a Cli, iter: L) -> Output<'a, L, S> {
    Output {
      cli,
      lines: iter,
      line_count: 0,
      empty: (false, false),
      _phantom: PhantomData
    }
  }
}

impl<'a, L, S> Output<'a, L, S>
  where L: Iterator<Item = S>,
        S: AsRef<[u8]>
{
  pub fn write_to_stdout(self) -> Result<()> {
    let stdout = stdout();
    let mut tw = TabWriter::new(stdout.lock());
    let inter = if self.cli.np_dollar { vec![b'$', b'\n'] } else { vec![b'\n'] };
    for data in self.intersperse(inter) {
      tw.write_all(&data)?;
    }
    Ok(tw.flush()?)
  }
}

impl<'a, L, S> Iterator for Output<'a, L, S>
  where L: Iterator<Item = S>,
        S: AsRef<[u8]>
{
  type Item = Vec<u8>;

  fn next(&mut self) -> Option<Self::Item> {
    let line = match self.lines.next() {
      Some(l) => l,
      None => return None
    };
    let line = line.as_ref();

    if line.is_empty() {
      self.empty = (self.empty.1, true);
      if self.cli.squeeze_empty && self.empty.1 && self.empty.0 {
        return self.next();
      }
    } else {
      self.empty = (self.empty.1, false);
    }

    let mut next_str = Vec::new();
    // FIXME: numbers aren't right-aligned
    if self.cli.number_all_lines || self.cli.number_non_blank_lines && !line.is_empty() {
      self.line_count += 1;
      next_str.extend(self.line_count.to_string().into_bytes());
      next_str.extend(b"\t");
    }
    if self.cli.np || self.cli.np_dollar || self.cli.np_tab {
      next_str.extend(Sanitize::sanitize(self.cli.np_tab, line));
    } else {
      next_str.extend(line);
    }
    Some(next_str)
  }
}

const ESCAPE_CODES: &[(u8, char)] = &[
  (0, '@'),
  (1, 'A'),
  (2, 'B'),
  (3, 'C'),
  (4, 'D'),
  (5, 'E'),
  (6, 'F'),
  (7, 'G'),
  (8, 'H'),
  (11, 'K'),
  (12, 'L'),
  (13, 'M'),
  (14, 'N'),
  (15, 'O'),
  (16, 'P'),
  (17, 'Q'),
  (18, 'R'),
  (19, 'S'),
  (20, 'T'),
  (21, 'U'),
  (22, 'V'),
  (23, 'W'),
  (24, 'X'),
  (25, 'Y'),
  (26, 'Z'),
  (27, '['),
  (28, '\\'),
  (29, ']'),
  (30, '^'),
  (31, '_'),
  (127, '?'),
];

fn get_control(tabs: bool, byte: u8) -> Option<char> {
  if tabs && byte == 9 {
    return Some('I');
  }
  ESCAPE_CODES.iter().find(|&&(code, _)| code == byte).map(|&(_, x)| x)
}

struct Sanitize;

impl Sanitize {
  fn sanitize(tabs: bool, bytes: &[u8]) -> Vec<u8> {
    bytes.iter()
    .flat_map(|x| if_chain! {
        if x.is_ascii_control();
        if let Some(ctl) = get_control(tabs, *x);
        then {
          vec![b'^', ctl as u8]
        } else {
          vec![*x]
        }
      })
    .collect()
  }
}
