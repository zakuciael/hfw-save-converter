use crate::sfo::data::format::SFODataFormat;
use binrw::{binread, NullString};
use derivative::Derivative;

pub mod format;

#[binread]
#[derive(Derivative)]
#[derivative(Debug)]
#[br(little)]
#[br(import { format: SFODataFormat, length: u32, capacity: u32 })]
pub enum SFOParamData {
  #[br(pre_assert(format == SFODataFormat::SpecialMode))]
  SpecialMode(
    #[br(count = length, pad_size_to = capacity)]
    #[derivative(Debug(format_with = "crate::utils::fmt::debug_vec"))]
    Vec<u8>,
  ),
  #[br(pre_assert(format == SFODataFormat::Utf8))]
  Utf8(#[br(pad_size_to = capacity)] NullString),
  #[br(pre_assert(format == SFODataFormat::Int))]
  Int(u32),
}
