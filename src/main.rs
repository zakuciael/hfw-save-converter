/*
 * hfw-save-converter
 * Copyright (c) 2024 Krzysztof Saczuk <zakku@zakku.eu>.
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of  MERCHANTABILITY or FITNESS FOR
 * A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::env;
use std::fs::File;
use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use color_eyre::eyre::{eyre, WrapErr};
use path_absolutize::Absolutize;
use tracing::{debug, info, trace, warn};

use crate::cli::macros::clap_error;
use crate::cli::CliArgs;
use crate::log::{print_logo, setup_tracing};
use crate::save::{SaveFile, SaveMetadata};
use crate::sfo::SFOFile;

mod cli;
mod log;
mod save;
mod sfo;
mod utils;

/// The CLI accepts two arguments the "save_file" and the "output_dir"
/// The first one is a path to the PS4 save file, and the second one
/// is a path to the location where the generated save file will be stored.
///
/// The "save_file" argument is parsed to extract the "base_path" and the actual "save_file" paths,
/// the first one is the parent directory of the save file,
/// and the latter is the actual save file path.
///
/// Then we proceed to look up the save metadata needed to generate the PC save file.
/// The easiest way to do this is to look up the `param.sfo` file
/// either in the `base_path` or in the `base_path/sce_sys` directory.
///
/// This file contains PS4 metadata about the save file,
/// like the name of the file (for example, AUTOSAVE0) or the in-game title.
/// We can also find the save file image in the `base_path/sce_sys` directory, so we can
/// assume to search for it under the `base_path` for the maintainability,
/// or we can abandon the `base_path` search all together.
///
/// The latter makes it easier to implement by looking up only one directory.
/// It also removes the need for the `base_path`, because we can replace it with the `sce_path`
fn main() -> color_eyre::Result<()> {
  print_logo();
  color_eyre::install()?;

  let cli = CliArgs::parse();
  setup_tracing(if cli.verbose {
    tracing::Level::DEBUG
  } else if cli.trace {
    tracing::Level::TRACE
  } else {
    tracing::Level::INFO
  })
  .wrap_err("failed to setup logging for the application")?;

  debug!("Resolving save file location..");
  let (sce_sys_path, save_file) = {
    let path = &cli
      .save_file
      .absolutize()
      .wrap_err("failed to resolve save file path")?;

    if !path.exists() {
      clap_error!("path {:?} doesn't exist", &path);
    }

    let verify_save_file = |base_path: PathBuf, save_file: PathBuf| -> (PathBuf, PathBuf) {
      if !save_file.exists() {
        clap_error!("no save file found in {:?}", &base_path);
      }

      if !save_file.is_file() {
        clap_error!("no save file found in {:?}", &base_path);
      }

      match File::open(&save_file) {
        Ok(_) => (base_path, save_file),
        Err(err) => clap_error!("cannot open file {:?}: {}", &save_file, err),
      }
    };

    let (base_path, save_file) = if path.is_dir() {
      let base_path = path.to_path_buf();
      let save_file = path.join("checkpoint.dat");

      verify_save_file(base_path, save_file)
    } else {
      let base_path = path
        .parent()
        .ok_or_else(|| eyre!("failed to resolve parent for {:?} path", &path))?
        .to_path_buf();
      let save_file = path.to_path_buf();

      verify_save_file(base_path, save_file)
    };

    (base_path.join("sce_sys"), save_file)
  };
  trace!(sce_sys_path = ?&sce_sys_path, save_file = ?&save_file);

  info!("Looking for PS4 metadata files in {:?}..", &sce_sys_path);
  let metadata = if sce_sys_path.exists() && sce_sys_path.is_dir() {
    let sfo_path = sce_sys_path.join("param.sfo");
    let icon_path = sce_sys_path.join("icon0.png");

    if sfo_path.exists() && sfo_path.is_file() {
      debug!("Parsing param.sfo file..");
      let sfo_file = SFOFile::open(sfo_path).wrap_err("failed to read param.sfo file")?;
      trace!(sfo_file = ?&sfo_file);

      let mut metadata: SaveMetadata = sfo_file
        .try_into()
        .wrap_err("failed to resolve save metadata from param.sfo file")?;

      if icon_path.exists() && icon_path.is_file() {
        metadata.image_path = Some(icon_path);
      }

      metadata
    } else {
      warn!("No param.sfo file found in PS4 metadata directory, dummy metadata will be used.");
      SaveMetadata::default()
    }
  } else {
    warn!("No PS4 metadata directory found, dummy metadata will be used.");
    SaveMetadata::default()
  };
  trace!(metadata = ?&metadata);

  let output_dir = if let Some(output_dir) = cli.output_dir {
    output_dir
  } else {
    env::current_dir().wrap_err("failed to resolve current working directory")?
  };

  SaveFile::generate(save_file, output_dir, metadata, cli.force)
}
