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
