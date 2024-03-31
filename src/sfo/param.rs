use crate::sfo::param_data::SFOParamData;

#[derive(Debug)]
pub struct SFOParam {
  pub key: String,
  pub format_type: u16,
  pub length: u32,
  pub max_length: u32,
  pub data: SFOParamData,
}

impl SFOParam {
  pub fn new(
    key: String,
    format_type: u16,
    length: u32,
    max_length: u32,
    data: SFOParamData,
  ) -> Self {
    Self {
      key,
      format_type,
      length,
      max_length,
      data,
    }
  }
}
