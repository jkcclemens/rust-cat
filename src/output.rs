use cli::Cli;
use Result;

use std::io::Write;

const LINE_COUNT_BUF_LEN: usize = 20;
const LINE_COUNT_BUF: [u8; LINE_COUNT_BUF_LEN] = [
  b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
  b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ', b'0',
  b'\t', b'\0'
];

pub struct Output<'a, 'b> {
  cli: &'a Cli,
  data: &'b [u8],
  line_count_buf: [u8; LINE_COUNT_BUF_LEN],
  line_num_print: usize,
  line_num_start: usize,
  line_num_end: usize,
  newlines: (bool, bool)
}

impl<'a, 'b> Output<'a, 'b> {
  pub fn new(cli: &'a Cli, data: &'b [u8]) -> Output<'a, 'b> {
    Output {
      cli,
      data,
      line_count_buf: LINE_COUNT_BUF,
      line_num_print: LINE_COUNT_BUF_LEN - 8,
      line_num_start: LINE_COUNT_BUF_LEN - 3,
      line_num_end: LINE_COUNT_BUF_LEN - 3,
      newlines: (false, true)
    }
  }
}

impl<'a, 'b> Output<'a, 'b> {
  pub fn write<W: Write>(mut self, mut writer: W) -> Result<()> {
    if self.cli.simple() {
      writer.write_all(self.data)?;
      return Ok(());
    }
    let non_printing = self.cli.non_printing();
    let mut next = Vec::with_capacity(self.data.len() * 2);
    for &byte in self.data {
      let line_break = byte == b'\n';
      if self.cli.squeeze_empty && line_break && self.empty() {
        continue;
      }
      self.newlines = (self.newlines.1, line_break);

      if self.newlines.0 && (self.cli.number_all_lines || self.cli.number_non_blank_lines && !self.empty()) {
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
    let mut endp = self.line_num_end;
    loop {
      if self.line_count_buf[endp] < b'9' {
        self.line_count_buf[endp] += 1;
        break;
      }
      self.line_count_buf[endp] = b'0';
      endp -= 1;
      if endp < self.line_num_start {
        break;
      }
    }
    if self.line_num_start > endp {
      self.line_num_start -= 1;
      self.line_count_buf[self.line_num_start] = b'1';
    }
    if self.line_num_start < self.line_num_print {
      self.line_num_print -= 1;
    }
    for &b in &self.line_count_buf[self.line_num_print..self.line_num_end + 1] {
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
