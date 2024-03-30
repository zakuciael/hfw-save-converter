use std::io::Read;

use clap::Parser;
use figment::Figment;
use figment::providers::Serialized;

use crate::cli::CliArgs;
use crate::log::{print_logo, setup_tracing};
use crate::save::{SaveFile, SaveMetadata};

pub mod cli;
mod input;
mod log;
pub mod save;

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;
  print_logo();
  setup_tracing()?;

  let mut cli_args = CliArgs::parse();
  println!("{:?}", &cli_args);

  let metadata: SaveMetadata = Figment::from(Serialized::defaults(SaveMetadata::default()))
    .merge(&cli_args.overrides)
    .extract()?;

  println!("{:?}", &metadata);

  let save_data = {
    let mut buf = vec![];
    cli_args.path.read_to_end(&mut buf)?;

    buf
  };

  let save_file = SaveFile::new(metadata, save_data);

  save_file.generate(&"./")?;

  Ok(())
}
