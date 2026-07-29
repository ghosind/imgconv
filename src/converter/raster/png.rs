use std::path::Path;

use crate::converter::options::ConverterOptions;
use crate::converter::raster::util::convert;
use crate::core::traits::ImageConverter;
use crate::error::convert::ImageConvertError;

/// Converter for PNG input images.
///
/// Implements [`ImageConverter`] by delegating to the shared raster utility.
pub struct PNGConverter;

impl ImageConverter for PNGConverter {
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

  #[test]
  fn png_converter_converts_to_jpg() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.png");
    DynamicImage::new_rgba8(2, 2).save(&input).unwrap();
    let output = dir.path().join("out.jpg");
    let opts = ConverterOptions {
      target_format: ImageFormat::JPG,
      processors: vec![],
      overwrite: false,
    };
    let result = PNGConverter.convert(&input, &output, &opts);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn png_converter_converts_to_webp() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.png");
    DynamicImage::new_rgba8(2, 2).save(&input).unwrap();
    let output = dir.path().join("out.webp");
    let opts = ConverterOptions {
      target_format: ImageFormat::WEBP,
      processors: vec![],
      overwrite: false,
    };
    let result = PNGConverter.convert(&input, &output, &opts);
    assert!(result.is_ok());
    assert!(output.exists());
  }
}
