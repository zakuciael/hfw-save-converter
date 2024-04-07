use crate::save::SaveMetadata;
use crate::sfo::data::SFOParamData;
use crate::sfo::param::SFOParam;
use crate::utils::error::ToReport;
use binrw::io::BufReader;
use binrw::{binread, BinRead};
use color_eyre::eyre::bail;
pub use header::SFOHeader;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

mod data;
mod header;
mod param;

static SFO_SAVE_DATA_CATEGORY: &str = "sd";

#[binread]
#[derive(Debug)]
#[br(little)]
pub struct SFOFile {
  #[br(ignore)]
  path: PathBuf,

  pub header: SFOHeader,

  #[br(map = |params: Vec<SFOParam>| params.into_iter().map(|param| (param.key.to_string(), param.data)).collect())]
  #[br(args { count: header.entries_count as usize, inner: binrw::args!{ key_table_offset: header.key_table_offset, data_table_offset: header.data_table_offset } })]
  pub params: HashMap<String, SFOParamData>,
}

impl SFOFile {
  //noinspection DuplicatedCode
  pub fn open<P: AsRef<Path>>(path: P) -> color_eyre::Result<Self> {
    let mut val =
      Self::read(&mut BufReader::new(File::open(path.as_ref())?)).map_err(|err| err.to_report())?;
    val.path = path.as_ref().to_path_buf();

    Ok(val)
  }
}

impl TryInto<SaveMetadata> for SFOFile {
  type Error = color_eyre::Report;

  fn try_into(self) -> Result<SaveMetadata, Self::Error> {
    if let Some(SFOParamData::Utf8(category)) = self.params.get("CATEGORY") {
      if category.to_string() != SFO_SAVE_DATA_CATEGORY {
        bail!(
          "invalid SFO file, expected category to be {:?} but found {:?}",
          SFO_SAVE_DATA_CATEGORY,
          category.to_string()
        )
      }
    } else {
      bail!("invalid SFO file, missing or invalid CATEGORY param");
    }

    let title = if let Some(SFOParamData::Utf8(title)) = self.params.get("MAINTITLE") {
      title.to_string()
    } else {
      bail!("invalid SFO file, missing or invalid MAINTITLE param");
    };

    let sub_title = if let Some(SFOParamData::Utf8(sub_title)) = self.params.get("SUBTITLE") {
      sub_title
        .to_string()
        .replace("Horizon Forbidden West™ - ", "")
    } else {
      bail!("invalid SFO file, missing or invalid SUBTITLE param");
    };

    let file_name =
      if let Some(SFOParamData::Utf8(file_name)) = self.params.get("SAVEDATA_DIRECTORY") {
        file_name.to_string().to_lowercase()
      } else {
        bail!("invalid SFO file, missing or invalid SAVEDATA_DIRECTORY param");
      };

    let checksum = if let Some(SFOParamData::Int(checksum)) = self.params.get("SAVEDATA_LIST_PARAM")
    {
      *checksum
    } else {
      bail!("invalid SFO file, missing or invalid SAVEDATA_LIST_PARAM param");
    };

    Ok(SaveMetadata::new(
      file_name, title, sub_title, checksum, None,
    ))
  }
}
