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

macro_rules! clap_error {
  ($msg:literal $(,)?) => (
    $crate::cli::macros::clap_error!(kind: clap::error::ErrorKind::ValueValidation, $msg)
  );
  (kind: $kind:expr, $msg:literal $(,)?) => ({
    let mut cmd = $crate::cli::CliArgs::command();
      cmd.error(
        $kind,
        core::format_args!($msg)
      ).exit();
  });
  ($fmt:expr, $($arg:tt)*) => (
    $crate::cli::macros::clap_error!(kind: clap::error::ErrorKind::ValueValidation, $fmt, $($arg)*)
  );
  (kind: $kind:expr, $fmt:expr, $($arg:tt)*) => ({
      let mut cmd = $crate::cli::CliArgs::command();
      cmd.error(
        $kind,
        format!($fmt, $($arg)*)
      ).exit();
  });
}

pub(crate) use clap_error;
