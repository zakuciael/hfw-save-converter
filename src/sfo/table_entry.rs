use byteorder::{LittleEndian, ReadBytesExt};
use tracing::instrument;

#[derive(Debug)]
pub struct SFOIndexTableEntry {
  pub key_offset: u16,
  pub data_format: u16,
  pub data_length: u32,
  pub data_max_length: u32,
  pub data_offset: u32,
}

impl SFOIndexTableEntry {
  #[instrument(skip_all, name = "SFOIndexTableEntry", level = "trace")]
  pub fn parse(mut data: &[u8]) -> color_eyre::Result<Self> {
    Ok(Self {
      key_offset: data.read_u16::<LittleEndian>()?,
      data_format: data.read_u16::<LittleEndian>()?,
      data_length: data.read_u32::<LittleEndian>()?,
      data_max_length: data.read_u32::<LittleEndian>()?,
      data_offset: data.read_u32::<LittleEndian>()?,
    })
  }
}
