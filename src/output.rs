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
  pub fn write<W: Write>(mut self, writer: W) -> Result<()> {
    let mut tw: Box<Write> = if self.cli.simple() || (!self.cli.number_all_lines && !self.cli.number_non_blank_lines) {
      box writer
    } else {
      box TabWriter::new(writer)
    };
    while self.write_next_line(&mut tw)? != 0 {
    }
    Ok(tw.flush()?)
  }

  fn write_next_line<W: Write>(&mut self, mut writer: W) -> Result<usize> {
    if self.cli.simple() && self.pos == 0 {
      self.pos = self.data.len();
      writer.write_all(self.data)?;
      return Ok(self.pos);
    }

    if self.pos == self.data.len() {
      return Ok(0);
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
        return self.write_next_line(writer);
      }
    } else {
      self.empty = (self.empty.1, false);
    }

    let mut written = 0;

    // FIXME: numbers aren't right-aligned
    if self.cli.number_all_lines || self.cli.number_non_blank_lines && !line.is_empty() {
      self.line_count += 1;
      let line_num = self.line_count.to_string().into_bytes();
      writer.write_all(&line_num)?;
      written += line_num.len();
      writer.write_all(b"\t")?;
      written += 1;
    }
    for &byte in line {
      if_chain! {
        if self.cli.np || self.cli.np_dollar || self.cli.np_tab;
        if let Some(ctl) = get_control(self.cli.np_tab, byte);
        then {
          writer.write_all(&[b'^', ctl])?;
          written += 2;
        } else {
          writer.write_all(&[byte])?;
          written += 1;
        }
      }
    }
    Ok(written)
  }
}

fn get_control(tabs: bool, byte: u8) -> Option<u8> {
  if !byte.is_ascii_control() || (byte == b'\t' && !tabs) {
    return None;
  }
  Some(byte + 64)
}

struct Sanitize;

impl Sanitize {
  fn sanitize(tabs: bool, bytes: &[u8]) -> Vec<u8> {
    bytes.iter()
      .flat_map(|&x| match get_control(tabs, x) {
        Some(ctl) => vec![b'^', ctl as u8],
        None => vec![x]
      })
      .collect()
  }
}
