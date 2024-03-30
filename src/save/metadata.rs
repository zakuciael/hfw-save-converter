use educe::Educe;
use image::{ExtendedColorType, ImageEncoder, Rgb, RgbImage};
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use serde::{Deserialize, Serialize};

use crate::save::r#type::SaveType;

static DEFAULT_IMAGE_WIDTH: u32 = 480;
static DEFAULT_IMAGE_HEIGHT: u32 = 270;

#[derive(Serialize, Deserialize, Educe)]
#[educe(Debug)]
pub struct SaveMetadata {
  pub name: String,
  pub title: String,
  pub r#type: SaveType,
  #[educe(Debug(ignore))]
  pub image: Vec<u8>,
}

impl Default for SaveMetadata {
  fn default() -> Self {
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

    SaveMetadata {
      name: "autosave0".to_owned(),
      title: "Reach For The Stars - Level 1 - 00:00:00".to_owned(),
      r#type: SaveType::Autosave,
      image: raw_img,
    }
  }
}
