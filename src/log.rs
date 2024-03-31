use std::str::FromStr;

use lazy_static::lazy_static;
use tiny_gradient::{GradientStr, RGB};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;

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

pub fn setup_tracing() -> color_eyre::Result<()> {
  // For this app, I define three types of log levels:
  // 1. INFO – Used to inform the user about the relevant actions that the program is performing.
  //           For example it will log that the save was generated to a location X
  // 2. DEBUG – Used to inform the user about in-between steps that the program is performing.
  //           For example it will log that its parsing CLI args, resolving some files etc.
  // 3. TRACE – Used to debug the application in case of a failure.
  //           For example it will log outputs of the functions that it performs or steps it took to parse the file.

  let level_filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .with_env_var("LOG_LEVEL")
    .from_env_lossy();

  tracing_subscriber::fmt()
    .with_env_filter(level_filter)
    .compact()
    .init();

  Ok(())
}

pub fn print_logo() {
  match std::env::var("NO_COLOR") {
    Ok(_) => println!("{}", LOGO),
    Err(_) => println!("{}", LOGO.gradient(*COLORS)),
  }
}
