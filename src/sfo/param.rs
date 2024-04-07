/*
 * hfw-save-converter
 * Copyright (c) 2024 Krzysztof Saczuk <zakku@zakku.eu>.
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, either version 3 of the License, or (at your option) any later
 * version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY
 * WARRANTY; without even the implied warranty of  MERCHANTABILITY or FITNESS FOR
 * A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::io::SeekFrom;

use binrw::{binread, NullString};

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
