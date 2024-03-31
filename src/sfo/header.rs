use byteorder::{LittleEndian, ReadBytesExt};
use color_eyre::eyre::bail;

static SFO_MAGIC: u32 = 0x46535000;
static SFO_VERSION: u32 = 0x0101;

#[derive(Debug)]
pub struct SFOHeader {
  pub magic: u32,
  pub version: u32,
  pub key_table_offset: u32,
  pub data_table_offset: u32,
  pub index_table_size: u32,
}

impl SFOHeader {
  pub fn parse(mut data: &[u8]) -> color_eyre::Result<Self> {
    let magic = data.read_u32::<LittleEndian>()?;
    if magic != SFO_MAGIC {
      bail!("invalid sfo magic");
    }

    let version = data.read_u32::<LittleEndian>()?;
    if version != SFO_VERSION {
      bail!("invalid sfo version");
    }

    Ok(Self {
      magic,
      version,
      key_table_offset: data.read_u32::<LittleEndian>()?,
      data_table_offset: data.read_u32::<LittleEndian>()?,
      index_table_size: data.read_u32::<LittleEndian>()?,
    })
  }
}
