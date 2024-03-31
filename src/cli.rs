use std::io::Read;
use std::path::PathBuf;

use clap::{Args, Parser};
use figment::{Error, Metadata, Profile, Provider};
use figment::value::{Dict, Map, Value};

use crate::input::Input;
use crate::save::SaveType;

#[derive(Parser, Debug)]
pub struct CliArgs {
  #[arg(help = "Path to the checkpoint.dat file")]
  pub save_file: Input,

  #[arg(long, help = "Path to the param.sfo file")]
  pub sfo: Option<Input>,

  #[arg(short, long, help = "Path to generated save file")]
  pub output: Option<PathBuf>,

  #[command(flatten)]
  pub overrides: OverrideArgs,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = true)]
pub struct OverrideArgs {
  #[arg(
    long = "override-type",
    value_name = "type",
    help = "Override the type of the save file"
  )]
  r#type: Option<SaveType>,

  #[arg(
    long = "override-name",
    help = "Override the name of the generated save file"
  )]
  name: Option<String>,

  #[arg(
    long = "override-title",
    help = "Override the in-game title of the save file"
  )]
  title: Option<String>,

  #[arg(
    long = "override-image",
    help = "Override the in-game image of the save file"
  )]
  image: Option<Input>,
}

impl Provider for OverrideArgs {
  fn metadata(&self) -> Metadata {
    Metadata::named("CLI Args")
  }

  fn data(&self) -> Result<Map<Profile, Dict>, Error> {
    let mut dict = Dict::new();

    if let Some(value) = &self.r#type {
      dict.insert("type".to_owned(), Value::from(value.to_string()));
    }

    if let Some(value) = &self.name {
      dict.insert("name".to_owned(), Value::from(value.clone()));
    }

    if let Some(value) = &self.title {
      dict.insert("title".to_owned(), Value::from(value.clone()));
    }

    if let Some(mut value) = self.image.clone() {
      let image = {
        let mut buf = vec![];
        value.read_to_end(&mut buf).map_err(|err| err.to_string())?;

        buf
      };

      dict.insert("image".to_owned(), Value::from(image));
    }

    Ok(Profile::default().collect(dict))
  }
}
