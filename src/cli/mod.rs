pub mod macros;

use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
pub struct CliArgs {
  #[arg(help = "Path to the decrypted PS4 save file", value_hint = ValueHint::AnyPath)]
  pub save_file: PathBuf,

  #[arg(long = "output", short = 'o', help = "Path to a directory where the generated save file will be stored", value_hint = ValueHint::DirPath)]
  pub output_dir: Option<PathBuf>,

  #[arg(long, help = "Overwrite if output file already exists")]
  pub force: bool,

  #[arg(
    long,
    group = "log_level",
    help = "Enable verbose logging for the application"
  )]
  pub verbose: bool,

  #[arg(
    long,
    group = "log_level",
    help = "Enable trace logging for the application"
  )]
  pub trace: bool,
}
