use binrw::binread;

#[binread]
#[derive(Debug, PartialEq, Clone)]
#[br(repr(u16))]
pub enum SFODataFormat {
  SpecialMode = 0x0004,
  Utf8 = 0x0204,
  Int = 0x0404,
}
