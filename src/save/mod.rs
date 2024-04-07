use crate::utils;
use crate::utils::error::ToReport;
use binrw::io::BufReader;
use binrw::{binrw, BinRead, BinWrite, NullString};
use color_eyre::eyre::{bail, WrapErr};
use color_eyre::Report;
use derivative::Derivative;
pub use metadata::SaveMetadata;
use path_absolutize::Absolutize;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, ErrorKind};
use std::path::{Path, PathBuf};
use tracing::{error, info, warn};

mod metadata;

#[binrw]
#[derive(Derivative)]
#[derivative(Debug)]
#[brw(little, magic = b"GGDS")]
pub struct SaveFile {
  #[brw(ignore)]
  path: PathBuf,

  version: u32,
  checksum: u32,
  #[bw(calc = data.len() as u32)]
  data_length: u32,
  #[bw(calc = image.len() as u32)]
  image_length: u32,
  #[brw(pad_size_to = 0x80)]
  title: NullString,
  #[brw(pad_size_to = 0x100)]
  sub_title: NullString,
  #[br(count = data_length)]
  #[derivative(Debug(format_with = "crate::utils::fmt::debug_vec"))]
  data: Vec<u8>,
  #[br(count = image_length)]
  #[derivative(Debug(format_with = "crate::utils::fmt::debug_vec"))]
  image: Vec<u8>,
}

impl SaveFile {
  //noinspection DuplicatedCode
  pub fn open<P: AsRef<Path>>(path: P) -> color_eyre::Result<Self> {
    let mut this =
      Self::read(&mut BufReader::new(File::open(path.as_ref())?)).map_err(|err| err.to_report())?;
    this.path = path.as_ref().to_path_buf();

    Ok(this)
  }

  pub fn generate<I, O>(
    save_file: I,
    output_dir: O,
    metadata: SaveMetadata,
    overwrite: bool,
  ) -> color_eyre::Result<()>
  where
    I: AsRef<Path>,
    O: AsRef<Path>,
  {
    let output_dir = output_dir
      .as_ref()
      .absolutize()
      .wrap_err("failed to resolve output directory path")?;
    let save_data = fs::read(&save_file)
      .wrap_err_with(|| format!("failed to read save data {:?}", save_file.as_ref()))?;
    let image_data = if let Some(image_path) = &metadata.image_path {
      if !image_path.exists() || !image_path.is_file() {
        warn!("No image found for the save file, generating blank image..");
        utils::generate_blank_image()?
      } else {
        fs::read(&image_path)
          .wrap_err_with(|| format!("failed to read save image {:?}", &image_path))?
      }
    } else {
      warn!("No image found for the save file, generating blank image..");
      utils::generate_blank_image()?
    };

    if !output_dir.exists() {
      fs::create_dir_all(&output_dir)
        .wrap_err_with(|| format!("failed to create output directory {:?}", &output_dir))?;
    } else if !output_dir.is_dir() {
      bail!("output path is not a directory");
    }

    let output_file_path = output_dir.join(&metadata.file_name).with_extension("dat");
    let mut writer = BufWriter::new(if overwrite {
      File::create(&output_file_path)
        .wrap_err_with(|| format!("failed to create file {:?}", &output_file_path))?
    } else {
      match File::create_new(&output_file_path) {
        Ok(writer) => writer,
        Err(err) => {
          return match err {
            _ if err.kind() == ErrorKind::AlreadyExists => {
              error!(
                "Unable to generate the save file, file already exists. Use --force to overwrite it."
              );
              Ok(())
            }
            _ => Err(
              Report::from(err).wrap_err(format!("failed to create file {:?}", &output_file_path)),
            ),
          }
        }
      }
    });
    let save = SaveFile {
      path: output_file_path,
      version: 1,
      checksum: metadata.checksum,
      title: metadata.title.into(),
      sub_title: metadata.sub_title.into(),
      data: save_data,
      image: image_data,
    };

    save
      .write(&mut writer)
      .map_err(|err| err.to_report()) // TODO: Add error context
      .inspect(|_| {
        info!("Saved the generated save file to {:?}", &save.path);
      })
  }
}
