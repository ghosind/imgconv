use std::path::Path;

use crate::converter::options::ConverterOptions;
use crate::converter::raster::util::convert;
use crate::core::traits::ImageConverter;
use crate::error::convert::ImageConvertError;

/// Converter for JPEG input images.
///
/// Implements [`ImageConverter`] by delegating to the shared raster utility.
pub struct JPGConverter;

impl ImageConverter for JPGConverter {
  fn convert(
    &self,
    input_path: &Path,
    output_path: &Path,
    options: &ConverterOptions,
  ) -> Result<(), ImageConvertError> {
    convert(input_path, output_path, options)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::format::ImageFormat;
  use image::DynamicImage;

  fn create_test_jpg(dir: &tempfile::TempDir) -> std::path::PathBuf {
    let path = dir.path().join("input.jpg");
    let img = DynamicImage::new_rgb8(2, 2);
    img.save(&path).unwrap();
    path
  }

  #[test]
  fn jpg_converter_converts_to_png() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_jpg(&dir);
    let output = dir.path().join("out.png");
    let opts = ConverterOptions {
      target_format: ImageFormat::PNG,
      processors: vec![],
      overwrite: false,
    };
    let result = JPGConverter.convert(&input, &output, &opts);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn jpg_converter_converts_to_webp() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_jpg(&dir);
    let output = dir.path().join("out.webp");
    let opts = ConverterOptions {
      target_format: ImageFormat::WEBP,
      processors: vec![],
      overwrite: false,
    };
    let result = JPGConverter.convert(&input, &output, &opts);
    assert!(result.is_ok());
    assert!(output.exists());
  }
}
