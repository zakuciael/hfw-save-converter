use std::ffi::{OsStr, OsString};
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use anyhow::bail;
use clap::{Arg, Command};
use clap::builder::{TypedValueParser, ValueParserFactory};
use clap::error::{Error, ErrorKind};

pub struct Input {
  path: PathBuf,
  file: File,
}

impl Input {
  pub fn new<T: AsRef<Path>>(path: &T) -> anyhow::Result<Self> {
    let path = path.as_ref().to_path_buf();
    let file = File::open(&path)?;

    if file.metadata()?.is_dir() {
      bail!("path is a directory")
    }

    Ok(Self { path, file })
  }

  pub fn len(&self) -> Option<u64> {
    self.file.metadata().ok().map(|x| x.len())
  }

  pub fn path(&self) -> &Path {
    &self.path
  }
}

impl Read for Input {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    self.file.read(buf)
  }
}

impl Seek for Input {
  fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
    self.file.seek(pos)
  }
}

impl TryFrom<&OsStr> for Input {
  type Error = anyhow::Error;

  fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
    Input::new(&value)
  }
}

impl TryFrom<OsString> for Input {
  type Error = anyhow::Error;

  fn try_from(value: OsString) -> Result<Self, Self::Error> {
    Input::new(&value)
  }
}

impl TryFrom<PathBuf> for Input {
  type Error = anyhow::Error;

  fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
    Input::new(&value)
  }
}

impl TryFrom<&Path> for Input {
  type Error = anyhow::Error;

  fn try_from(value: &Path) -> Result<Self, Self::Error> {
    Input::new(&value)
  }
}

impl TryFrom<&String> for Input {
  type Error = anyhow::Error;

  fn try_from(value: &String) -> Result<Self, Self::Error> {
    Input::new(&value)
  }
}

impl TryFrom<&str> for Input {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    Input::new(&value)
  }
}

impl Debug for Input {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    write!(fmt, "{:?}", self.path.as_os_str())
  }
}

impl Display for Input {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    write!(fmt, "{:?}", self.path.as_os_str())
  }
}

impl Clone for Input {
  fn clone(&self) -> Self {
    Input::new(&self.path.clone()).unwrap()
  }
}

impl ValueParserFactory for Input {
  type Parser = FilePathParser;

  fn value_parser() -> Self::Parser {
    FilePathParser::default()
  }
}

trait Parseable: Clone + Sync + Send {}

impl Parseable for Input {}

#[derive(Default, Clone, Debug)]
pub struct FilePathParser {
  extension: Option<String>,
}

impl FilePathParser {
  pub fn extension<T: AsRef<str>>(mut self, ext: &T) -> Self {
    self.extension = Some(ext.as_ref().to_string());
    self
  }

  fn validate(&self, value: &OsStr) -> anyhow::Result<PathBuf> {
    let path = PathBuf::from(value);

    if !path.exists() {
      bail!("path doesn't exist");
    }

    if path.is_dir() {
      bail!("path is a directory");
    }

    if self.extension.is_some()
      && path.extension().map(|v| v.to_string_lossy().to_string()) != self.extension
    {
      bail!(
        "file extension should be {:?}",
        self.extension.clone().unwrap()
      );
    }

    Ok(path)
  }
}

impl TypedValueParser for FilePathParser {
  type Value = Input;

  fn parse_ref(
    &self,
    cmd: &Command,
    arg: Option<&Arg>,
    value: &OsStr,
  ) -> Result<Self::Value, Error> {
    self
      .validate(value)
      .and_then(Input::try_from)
      .map_err(|err| {
        cmd.clone().error(
          ErrorKind::InvalidValue,
          if let Some(arg) = arg {
            format!(
              "Invalid value for {}: Could not open {:?}: {}",
              arg, value, err
            )
          } else {
            format!("Could not open {:?}: {}", value, err)
          },
        )
      })
  }
}
