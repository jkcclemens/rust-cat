use cli::Cli;
use Result;

use std::io::Write;
use tabwriter::TabWriter;

pub struct Output<'a, 'b> {
  cli: &'a Cli,
  data: &'b [u8],
  line_count: usize,
  newlines: (bool, bool)
}

impl<'a, 'b> Output<'a, 'b> {
  pub fn new(cli: &'a Cli, data: &'b [u8]) -> Output<'a, 'b> {
    Output {
      cli,
      data,
      line_count: 0,
      newlines: (false, false)
    }
  }
}

impl<'a, 'b> Output<'a, 'b> {
  pub fn write<W: Write>(mut self, mut writer: W) -> Result<()> {
    if self.cli.simple() {
      writer.write_all(self.data)?;
      return Ok(());
    }
    // TODO: Don't use TabWriter (it's very, very, very slow)
    let mut writer: Box<Write> = if self.cli.number_lines() {
      box TabWriter::new(writer)
    } else {
      box writer
    };
    let non_printing = self.cli.non_printing();
    let mut next = Vec::with_capacity(self.data.len() * 2);
    for &byte in self.data {
      let line_break = byte == b'\n';
      if self.cli.squeeze_empty && line_break && self.empty() {
        continue;
      }
      self.newlines = (self.newlines.1, line_break);

      // FIXME: numbers aren't right-aligned
      if self.cli.number_all_lines || self.cli.number_non_blank_lines && !self.empty() {
        self.number_line(&mut next);
      }
      if self.cli.np_dollar && line_break {
        next.push(b'$');
      }
      if non_printing {
        self.get_control(self.cli.np_tab, byte, &mut next);
      } else {
        next.push(byte);
      }
    }
    writer.write_all(&next)?;
    Ok(writer.flush()?)
  }

  fn number_line(&mut self, next: &mut Vec<u8>) {
    self.line_count += 1;
    for b in self.line_count.to_string().into_bytes() {
      next.push(b);
    }
    next.push(b'\t');
  }

#[inline(always)]
fn get_control(&self, tabs: bool, byte: u8, next: &mut Vec<u8>) {
  if byte < 32 {
    if byte == b'\n' || byte == b'\t' && !tabs {
      next.push(byte);
    } else {
      next.push(b'^');
      next.push(byte + 64);
    }
    return;
  }
  if byte < 127 {
    next.push(byte);
  } else if byte == 127 {
    next.push(b'^');
    next.push(b'?');
  } else {
    next.push(b'M');
    next.push(b'-');
    if byte >= 128 + 32 {
      if byte < 128 + 127 {
        next.push(byte - 128);
      } else {
        next.push(b'^');
        next.push(b'?');
      }
    } else {
      next.push(b'^');
      next.push(byte - 128 + 64);
    }
  }
}

  fn empty(&self) -> bool {
    self.newlines.0 && self.newlines.1
  }
}
