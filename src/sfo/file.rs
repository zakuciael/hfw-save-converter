use std::collections::HashMap;
use std::path::PathBuf;

use figment::{Error, Metadata, Profile, Provider};
use figment::value::{Dict, Map, Value};
use tracing::{instrument, Level, span, trace};

use crate::save::SaveType;
use crate::sfo::{param_data::SFOParamData, SFOHeader, SFOParam};
use crate::sfo::table_entry::SFOIndexTableEntry;

#[derive(Debug)]
pub struct SFOFile {
  pub params: HashMap<String, SFOParam>,
  pub path: PathBuf,
}

impl SFOFile {
  #[instrument(skip_all, name = "SFOFile", level = "trace")]
  pub fn parse(data: &[u8], path: PathBuf) -> color_eyre::Result<Self> {
    trace!("Parsing SFO header..");
    let header = SFOHeader::parse(&data[0x00..0x14])?;
    trace!(header = ?&header);

    let mut params = HashMap::<String, SFOParam>::new();

    for i in 0..header.index_table_size {
      let offset = 0x14 + (i as usize * 0x10);
      let span = span!(
        Level::DEBUG,
        "index_table",
        table_index = i,
        entry_offset = offset
      );
      let _enter = span.enter();

      trace!("Parsing index table entry..");
      let entry = SFOIndexTableEntry::parse(&data[offset..offset + 0x10])?;
      trace!(entry = ?&entry);

      trace!("Parsing SFO param..");
      let key = {
        let buf = &data[header.key_table_offset as usize + entry.key_offset as usize..];
        let end = buf
          .iter()
          .position(|&c| c == b'\0')
          .expect("key should a null-terminated string");

        String::from_utf8(Vec::from(&buf[0..end]))?
      };

      let data = {
        let offset = (header.data_table_offset + entry.data_offset) as usize;
        let buf = &data[offset..offset + entry.data_max_length as usize];

        SFOParamData::parse(buf, entry.data_format)?
      };

      let param = SFOParam::new(
        key,
        entry.data_format,
        entry.data_length,
        entry.data_max_length,
        data,
      );
      trace!(param = ?&param);
      params.insert(param.key.clone(), param);
    }

    Ok(SFOFile { params, path })
  }
}

impl Provider for SFOFile {
  fn metadata(&self) -> Metadata {
    Metadata::named("SFO File").source(self.path.to_string_lossy().to_string())
  }

  fn data(&self) -> Result<Map<Profile, Dict>, Error> {
    let mut dict = Dict::new();

    if let Some(value) = self.params.get("MAINTITLE") {
      dict.insert("title".to_owned(), value.data.clone().into());
    }

    if let Some(value) = self.params.get("SAVEDATA_DIRECTORY") {
      let name = match &value.data {
        SFOParamData::String(val) => val.to_lowercase(),
        _ => {
          return Err(Error::from(
            "SAVEDATA_DIRECTORY has invalid type".to_owned(),
          ))
        }
      };
      let r#type = if name.starts_with("autosave") {
        SaveType::Autosave
      } else {
        SaveType::Manual
      };

      dict.insert("name".to_owned(), Value::from(name));
      dict.insert("type".to_owned(), Value::from(r#type.to_string()));
    }

    Ok(Profile::default().collect(dict))
  }
}
