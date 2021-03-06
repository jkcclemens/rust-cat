#[derive(Debug, StructOpt)]
pub struct Cli {
  #[structopt(short = "b")]
  pub number_non_blank_lines: bool,
  #[structopt(short = "e")]
  pub np_dollar: bool,
  #[structopt(short = "n")]
  pub number_all_lines: bool,
  #[structopt(short = "s")]
  pub squeeze_empty: bool,
  #[structopt(short = "t")]
  pub np_tab: bool,
  #[structopt(short = "u")]
  pub disable_buffering: bool,
  #[structopt(short = "v")]
  pub np: bool,
  #[structopt]
  pub files: Vec<String>
}

impl Cli {
  pub fn simple(&self) -> bool {
    !self.number_all_lines && !self.np_dollar && !self.number_non_blank_lines && !self.squeeze_empty && !self.np_tab && !self.np
  }

  pub fn non_printing(&self) -> bool {
    self.np || self.np_dollar || self.np_tab
  }

  pub fn number_lines(&self) -> bool {
    self.number_all_lines || self.number_non_blank_lines
  }
}
