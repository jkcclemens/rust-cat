use cli::Cli;
use Result;

use std::io::Write;
use std::borrow::Cow;
use tabwriter::TabWriter;
use itertools::Itertools;

pub struct Output<'a, 'b> {
  cli: &'a Cli,
  data: &'b [u8],
  pos: usize,
  line_count: usize,
  empty: (bool, bool),
}

impl<'a, 'b> Output<'a, 'b> {
  pub fn new(cli: &'a Cli, data: &'b [u8]) -> Output<'a, 'b> {
    Output {
      cli,
      data,
      pos: 0,
      line_count: 0,
      empty: (false, false)
    }
  }
}

impl<'a, 'b> Output<'a, 'b> {
  pub fn write<W: Write>(self, writer: W) -> Result<()> {
    let mut tw = TabWriter::new(writer);
    let inter: Box<Iterator<Item = Cow<'b, [u8]>>> = if self.cli.np_dollar {
      box self.intersperse(Cow::Owned(vec![b'$']))
    } else {
      box self
    };
    for data in inter {
      tw.write_all(&data)?;
    }
    Ok(())
  }
}

impl<'a, 'b> Iterator for Output<'a, 'b> {
  type Item = Cow<'b, [u8]>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cli.simple() && self.pos == 0 {
      self.pos = self.data.len();
      return Some(Cow::Borrowed(self.data));
    }

    if self.pos == self.data.len() {
      return None;
    }

    let last_pos = self.pos;
    self.pos = match self.data[last_pos + 1..].iter().position(|x| x == &b'\n') {
      Some(p) => self.pos + 1 + p,
      None => self.data.len()
    };
    let line = &self.data[last_pos..self.pos];

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
    Some(Cow::Owned(next_str))
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
