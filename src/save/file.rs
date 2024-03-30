use std::fs;
use std::io::Write;
use std::mem::size_of;
use std::path::{Path, PathBuf};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::save::metadata::SaveMetadata;

static SAVE_FILE_HEADER: &[u8; 12] = &[
  0x47, 0x47, 0x44, 0x53, 0x01, 0x00, 0x00, 0x00, 0xA9, 0xF4, 0x44, 0x06,
];
static SAVE_TITLE_SIZE: usize = 0x80; // 128
static SAVE_TYPE_SIZE: usize = 0x100; // 256

pub struct SaveFile {
  metadata: SaveMetadata,
  data: Vec<u8>,
}

impl SaveFile {
  pub fn new(metadata: SaveMetadata, data: Vec<u8>) -> Self {
    Self { metadata, data }
  }

  pub fn generate<T: AsRef<Path>>(&self, path: &T) -> std::io::Result<()> {
    let path = {
      let tmp = PathBuf::from(path.as_ref());
      tmp.join(if self.metadata.name.ends_with(".dat") {
        self.metadata.name.clone()
      } else {
        format!("{}.dat", self.metadata.name)
      })
    };

    let mut buf = Vec::with_capacity(
      SAVE_FILE_HEADER.len()
        + (size_of::<u32>() * 2)
        + SAVE_TITLE_SIZE
        + SAVE_TYPE_SIZE
        + self.data.len()
        + self.metadata.image.len(),
    );

    // Write save file header that is always the same
    buf.write_all(SAVE_FILE_HEADER)?;

    // Write the size of the actual save data
    buf.write_u32::<LittleEndian>(self.data.len() as u32)?;

    // Write the size of the save image
    buf.write_u32::<LittleEndian>(self.metadata.image.len() as u32)?;

    // Write the save title padded with 0 bytes at the end to reach the SAVE_TITLE_SIZE
    {
      let tmp = self.metadata.title.as_bytes();
      buf.write_all(tmp)?;
      buf.write_all(&vec![0u8; SAVE_TITLE_SIZE - tmp.len()])?;
    }

    // Write the save type padded with 0 bytes at the end to reach the SAVE_TYPE_SIZE
    {
      let tmp = self.metadata.r#type.to_string();
      let tmp = tmp.as_bytes();
      buf.write_all(tmp)?;
      buf.write_all(&vec![0u8; SAVE_TYPE_SIZE - tmp.len()])?;
    }

    // Write the actual save data
    buf.write_all(&self.data)?;

    // Write the save image
    buf.write_all(&self.metadata.image)?;

    // Write to the actual file
    fs::write(path, &buf)?;

    Ok(())
  }
}
