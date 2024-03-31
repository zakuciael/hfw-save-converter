use std::fmt::{Debug, Formatter};

use byteorder::{LittleEndian, ReadBytesExt};
use color_eyre::eyre::bail;
use figment::value::Value;

const SPECIAL_MODE_FORMAT: u16 = 0x0004;
const UTF8_FORMAT: u16 = 0x0204;
const INTEGER_FORMAT: u16 = 0x0404;

#[derive(Clone)]
pub enum SFOParamData {
  SpecialMode(Vec<u8>),
  String(String),
  Int(u32),
}

impl SFOParamData {
  pub fn parse(mut data: &[u8], format_type: u16) -> color_eyre::Result<Self> {
    Ok(match format_type {
      SPECIAL_MODE_FORMAT => SFOParamData::SpecialMode(Vec::from(data)),
      UTF8_FORMAT => {
        let end = data.iter().position(|&c| c == b'\0').unwrap_or(data.len()); // default to length if no `\0` present

        SFOParamData::String(String::from_utf8(Vec::from(&data[0..end]))?)
      }
      INTEGER_FORMAT => SFOParamData::Int(data.read_u32::<LittleEndian>()?),
      _ => bail!("unknown format type"),
    })
  }
}

impl Debug for SFOParamData {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      SFOParamData::SpecialMode(val) => {
        let limit = 10;

        if val.len() <= limit {
          fmt.debug_list().entries(val.iter().take(limit)).finish()
        } else {
          write!(
            fmt,
            "[{}, ...]",
            val
              .iter()
              .take(limit)
              .map(|v| v.to_string())
              .collect::<Vec<_>>()
              .join(", ")
          )
        }
      }
      SFOParamData::String(val) => write!(fmt, "{:?}", val),
      SFOParamData::Int(val) => write!(fmt, "{:?}", val),
    }
  }
}

impl Into<Value> for SFOParamData {
  fn into(self) -> Value {
    match self {
      SFOParamData::SpecialMode(val) => Value::from(val),
      SFOParamData::String(val) => Value::from(val),
      SFOParamData::Int(val) => Value::from(val),
    }
  }
}
