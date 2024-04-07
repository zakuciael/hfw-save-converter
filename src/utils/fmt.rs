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

use std::fmt::{Debug, Formatter};

static VEC_DEBUG_LIMIT: usize = 10;

enum OrMore<T> {
  Value(T),
  More,
}

impl<T: Debug> Debug for OrMore<T> {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      OrMore::Value(val) => Debug::fmt(val, fmt),
      OrMore::More => write!(fmt, "..."),
    }
  }
}

pub fn debug_vec<V: Debug>(val: &Vec<V>, fmt: &mut Formatter<'_>) -> std::fmt::Result {
  if val.len() <= VEC_DEBUG_LIMIT {
    fmt
      .debug_list()
      .entries(val.iter().take(VEC_DEBUG_LIMIT))
      .finish()
  } else {
    fmt
      .debug_list()
      .entries(
        val
          .iter()
          .take(VEC_DEBUG_LIMIT)
          .map(OrMore::Value)
          .chain(vec![OrMore::More]),
      )
      .finish()
  }
}
