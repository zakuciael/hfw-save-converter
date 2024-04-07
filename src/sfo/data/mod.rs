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

use binrw::{binread, NullString};
use derivative::Derivative;

use crate::sfo::data::format::SFODataFormat;

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
