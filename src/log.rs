use std::str::FromStr;

use lazy_static::lazy_static;
use tiny_gradient::{GradientStr, RGB};
use tracing::Level;

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
  tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
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