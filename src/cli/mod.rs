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

use std::path::PathBuf;

use clap::{Parser, ValueHint};

pub mod macros;

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
