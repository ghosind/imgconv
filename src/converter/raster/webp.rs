use std::path::Path;

use crate::core::format::ImageFormat;
use crate::core::traits::{ImageConverter, ImageProcessor};
use crate::error::convert::ImageConvertError;
use crate::converter::raster::util::convert;

/// Converter for WEBP input images.
///
/// Implements [`ImageConverter`] by delegating to the shared raster utility.
pub struct WEBPConverter;

impl ImageConverter for WEBPConverter {
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

  fn create_test_webp(dir: &tempfile::TempDir) -> std::path::PathBuf {
    let path = dir.path().join("input.webp");
    let img = DynamicImage::new_rgba8(2, 2);
    img.save(&path).unwrap();
    path
  }

  #[test]
  fn webp_converter_converts_to_png() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_webp(&dir);
    let output = dir.path().join("out.png");
    let result = WEBPConverter.convert(&input, &output, ImageFormat::PNG, vec![]);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn webp_converter_converts_to_jpg() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_webp(&dir);
    let output = dir.path().join("out.jpg");
    let result = WEBPConverter.convert(&input, &output, ImageFormat::JPG, vec![]);
    assert!(result.is_ok());
    assert!(output.exists());
  }
}
