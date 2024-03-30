use std::fmt::Display;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SaveType {
  Autosave,
  Manual,
}

impl Display for SaveType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let str = match self {
      SaveType::Autosave => "Autosave",
      SaveType::Manual => "Manual",
    };
    write!(f, "{}", str)
  }
}
