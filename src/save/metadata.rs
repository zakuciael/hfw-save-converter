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

use std::path::PathBuf;

#[derive(Debug)]
pub struct SaveMetadata {
  pub file_name: String,
  pub title: String,
  pub sub_title: String,
  pub checksum: u32,
  pub image_path: Option<PathBuf>,
}

impl SaveMetadata {
  pub fn new<F, T, S>(
    file_name: F,
    title: T,
    sub_title: S,
    checksum: u32,
    image_path: Option<PathBuf>,
  ) -> Self
  where
    F: AsRef<str>,
    T: AsRef<str>,
    S: AsRef<str>,
  {
    Self {
      file_name: file_name.as_ref().to_owned(),
      title: title.as_ref().to_owned(),
      sub_title: sub_title.as_ref().to_owned(),
      checksum,
      image_path,
    }
  }
}

impl Default for SaveMetadata {
  fn default() -> Self {
    /*
    static DEFAULT_IMAGE_WIDTH: u32 = 480;
    static DEFAULT_IMAGE_HEIGHT: u32 = 270;

    let mut img_buf = RgbImage::new(DEFAULT_IMAGE_WIDTH, DEFAULT_IMAGE_HEIGHT);

    for pixel in img_buf.pixels_mut() {
      *pixel = Rgb([0, 0, 0]);
    }

    let mut raw_img = vec![];
    PngEncoder::new_with_quality(&mut raw_img, CompressionType::Best, FilterType::NoFilter)
      .write_image(
        &img_buf,
        DEFAULT_IMAGE_WIDTH,
        DEFAULT_IMAGE_HEIGHT,
        ExtendedColorType::Rgb8,
      )
      .expect("Should correctly generate default save image");
      */

    SaveMetadata {
      file_name: "autosave0".to_owned(),
      title: "Reach For The Stars - Level 1 - 00:00:00".to_owned(),
      sub_title: "Autosave".to_owned(),
      checksum: 105182377u32, // 0xA9, 0xF4, 0x44, 0x06
      image_path: None,
    }
  }
}
