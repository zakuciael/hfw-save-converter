use binrw::{binread, NullString};
use std::io::SeekFrom;

use crate::sfo::data::format::SFODataFormat;
use crate::sfo::data::SFOParamData;

#[binread]
#[derive(Debug)]
#[br(little)]
#[br(import { key_table_offset: u32, data_table_offset: u32 })]
pub struct SFOParam {
  #[br(temp)]
  key_offset: u16,
  #[br(temp)]
  data_format: SFODataFormat,
  #[br(temp)]
  data_length: u32,
  #[br(temp)]
  data_capacity: u32,
  #[br(temp)]
  data_offset: u32,
  #[br(seek_before = SeekFrom::Start(key_table_offset as u64 + key_offset as u64), restore_position)]
  pub key: NullString,
  #[br(seek_before = SeekFrom::Start((data_table_offset + data_offset) as u64), restore_position)]
  #[br(args { format: data_format, length: data_length, capacity: data_capacity })]
  pub data: SFOParamData,
}
