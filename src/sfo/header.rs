use binrw::binread;

#[binread]
#[derive(Debug)]
#[br(little, magic = b"\0PSF")]
pub struct SFOHeader {
  pub version: u32,
  pub key_table_offset: u32,
  pub data_table_offset: u32,
  pub entries_count: u32,
}
