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

use core::convert::Into;
use std::fmt::{Display, Formatter};

use binrw::error::BacktraceFrame;
use binrw::Error;
use color_eyre::eyre::eyre;
use color_eyre::Report;

pub trait ToReport {
  fn to_report(&self) -> Report;
}

impl ToReport for Error {
  fn to_report(&self) -> Report {
    match &self {
      Error::Backtrace(backtrace) => {
        if let Some((first, rest)) = backtrace.frames.split_first() {
          let mut report = eyre!("{}", BacktraceFrameWrapper::from(first));

          for frame in rest {
            report = report.wrap_err(BacktraceFrameWrapper::from(frame).to_string())
          }

          report.wrap_err(backtrace.error.to_string())
        } else {
          eyre!("{}", &backtrace.error)
        }
      }
      _ => eyre!("{}", &self),
    }
  }
}

struct BacktraceFrameWrapper<'a>(&'a BacktraceFrame);

impl<'a> From<&'a BacktraceFrame> for BacktraceFrameWrapper<'a> {
  fn from(value: &'a BacktraceFrame) -> BacktraceFrameWrapper<'a> {
    BacktraceFrameWrapper(value)
  }
}

impl<'a> Display for BacktraceFrameWrapper<'a> {
  fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
    let message = match &self.0 {
      BacktraceFrame::Full { message: msg, .. } | BacktraceFrame::Message(msg) => msg.clone(),
      BacktraceFrame::Custom(ctx) => ctx.to_string().into(),
    };

    match &self.0 {
      BacktraceFrame::Full { file, line, .. } => write!(fmt, "{message}\n     at {file}:{line}"),
      BacktraceFrame::Message(_) | BacktraceFrame::Custom(_) => write!(fmt, "{message}"),
    }
  }
}
