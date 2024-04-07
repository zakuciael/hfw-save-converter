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

use std::str::FromStr;

use lazy_static::lazy_static;
use tiny_gradient::{GradientStr, RGB};
use tracing_error::ErrorLayer;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static LOGO: &str = r#"  _  _ _____      __  ___   ___   _____    ___ ___  _  ___   _____ ___ _____ ___ ___
 | || | __\ \    / / / __| /_\ \ / / __|  / __/ _ \| \| \ \ / / __| _ \_   _| __| _ \
 | __ | _| \ \/\/ /  \__ \/ _ \ V /| _|  | (_| (_) | .` |\ V /| _||   / | | | _||   /
 |_||_|_|   \_/\_/   |___/_/ \_\_/ |___|  \___\___/|_|\_| \_/ |___|_|_\ |_| |___|_|_\
"#;

lazy_static! {
  static ref COLORS: [RGB; 6] = ["#add7ff", "#89ddff", "#5de4c7", "#fae4fc", "#d0679d", "#fffac2",]
    .map(RGB::from_str)
    .map(|v| v.unwrap());
}

pub fn setup_tracing(level: tracing::Level) -> color_eyre::Result<()> {
  // For this app, I define three types of log levels:
  // 1. INFO – Used to inform the user about the relevant actions that the program is performing.
  //           For example it will log that the save was generated to a location X
  // 2. DEBUG – Used to inform the user about in-between steps that the program is performing.
  //           For example it will log that its parsing CLI args, resolving some files etc.
  // 3. TRACE – Used to debug the application in case of a failure.
  //           For example it will log outputs of the functions that it performs or steps it took to parse the file.

  let fmt_layer = tracing_subscriber::fmt::layer()
    .with_target(false)
    .compact();

  let level_filter = LevelFilter::from_level(level);

  Ok(
    tracing_subscriber::registry()
      .with(level_filter)
      .with(fmt_layer)
      .with(ErrorLayer::default())
      .try_init()?,
  )
}

pub fn print_logo() {
  match std::env::var("NO_COLOR") {
    Ok(_) => println!("{}", LOGO),
    Err(_) => println!("{}", LOGO.gradient(*COLORS)),
  }
}
