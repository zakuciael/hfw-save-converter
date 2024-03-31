use std::env;
use std::io::Read;

use clap::Parser;
use color_eyre::eyre::OptionExt;
use figment::Figment;
use figment::providers::Serialized;
use tracing::{debug, trace};

use crate::cli::CliArgs;
use crate::input::Input;
use crate::log::{print_logo, setup_tracing};
use crate::save::{SaveFile, SaveMetadata};
use crate::sfo::SFOFile;

pub mod cli;
mod input;
mod log;
pub mod save;
pub mod sfo;

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;
  print_logo();
  setup_tracing()?;

  debug!("Parsing CLI arguments..");
  let mut cli_args = CliArgs::parse();
  trace!(args = ?&cli_args);

  debug!("Resolving save file's base path..");
  let base_path = cli_args
    .save_file
    .path()
    .parent()
    .ok_or_eyre("failed to resolve save file's base path")?;
  trace!(base_path = ?&base_path);

  let mut figment = Figment::new().merge(Serialized::defaults(SaveMetadata::default()));

  // Either we get the path to the SFO file from the --sfo param,
  // or we search for it in both "base_path/sce_sys/param.sfo" and "base_path/param.sfo"
  // with the latter taking precedence over the first one.
  debug!("Resolving param.sfo file location..");
  let sfo_file_location = cli_args
    .sfo
    .or_else(|| Input::new(&(base_path.join("param.sfo"))).ok())
    .or_else(|| Input::new(&(base_path.join("sce_sys/param.sfo"))).ok());
  trace!(sfo_file_location = ?&sfo_file_location);

  if let Some(mut sfo_file_location) = sfo_file_location {
    debug!("Parsing param.sfo file..");
    let sfo_file = {
      let buf = {
        let mut buf = vec![];
        sfo_file_location.read_to_end(&mut buf)?;

        buf
      };

      SFOFile::parse(&buf, sfo_file_location.path().to_path_buf())?
    };
    trace!(sfo_file = ?&sfo_file);
    figment = figment.merge(&sfo_file);
  }

  debug!("Resolving save file image location..");
  let image_file_location = Input::new(&(base_path.join("icon0.png")))
    .ok()
    .or_else(|| Input::new(&(base_path.join("sce_sys/icon0.png"))).ok());
  trace!(image_file_location = ?&image_file_location);

  if let Some(mut image_file_location) = image_file_location {
    let buf = {
      let mut buf = vec![];
      image_file_location.read_to_end(&mut buf)?;

      buf
    };

    figment = figment.merge(("image", buf));
  }

  debug!("Resolving save metadata from combined sources..");
  let metadata = figment
    .merge(&cli_args.overrides)
    .extract::<SaveMetadata>()?;
  trace!(metadata = ?&metadata);

  trace!("Reading save file contents..");
  let save_data = {
    let mut buf = vec![];
    cli_args.save_file.read_to_end(&mut buf)?;

    buf
  };

  let save_file = SaveFile::new(metadata, save_data);
  save_file.generate(
    &cli_args
      .output
      .unwrap_or(env::current_dir().expect("failed to resolve working directory")),
  )?;

  Ok(())
}
