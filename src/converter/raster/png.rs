use std::path::Path;

use crate::core::format::ImageFormat;
use crate::core::traits::{ImageConverter, ImageProcessor};
use crate::error::convert::ImageConvertError;
use crate::converter::raster::util::convert;

/// Converter for PNG input images.
///
/// Implements [`ImageConverter`] by delegating to the shared raster utility.
pub struct PNGConverter;

impl ImageConverter for PNGConverter {
  fn convert(
    &self,
    input_path: &Path,
    output_path: &Path,
    target_format: ImageFormat,
    processors: Vec<Box<dyn ImageProcessor>>,
  ) -> Result<(), ImageConvertError> {
    convert(input_path, output_path, target_format, processors)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use image::DynamicImage;
  use crate::core::traits::ImageConverter;

  #[test]
  fn png_converter_converts_to_jpg() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.png");
    DynamicImage::new_rgba8(2, 2).save(&input).unwrap();
    let output = dir.path().join("out.jpg");
    let result = PNGConverter.convert(&input, &output, ImageFormat::JPG, vec![]);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn png_converter_converts_to_webp() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.png");
    DynamicImage::new_rgba8(2, 2).save(&input).unwrap();
    let output = dir.path().join("out.webp");
    let result = PNGConverter.convert(&input, &output, ImageFormat::WEBP, vec![]);
    assert!(result.is_ok());
    assert!(output.exists());
  }
}
