use std::io::Write;

use clap::Parser;
use figment::Figment;
use figment::providers::Serialized;

use crate::cli::CliArgs;
use crate::log::{print_logo, setup_tracing};
use crate::save::SaveMetadata;

pub mod cli;
mod input;
mod log;
pub mod save;

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;
  print_logo();
  setup_tracing()?;

  let cli_args = CliArgs::parse();
  println!("{:?}", &cli_args);

  let metadata: SaveMetadata = Figment::from(Serialized::defaults(SaveMetadata::default()))
    .merge(&cli_args.overrides)
    .extract()?;

  println!("{:?}", &metadata);

  Ok(())
}
