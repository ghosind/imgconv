use std::path::Path;

use crate::converter::raster::util::convert;
use crate::core::format::ImageFormat;
use crate::core::traits::{ImageConverter, ImageProcessor};
use crate::error::convert::ImageConvertError;

/// Converter for ICO input images.
///
/// Implements [`ImageConverter`] by delegating to the shared raster utility.
pub struct ICOConverter;

impl ImageConverter for ICOConverter {
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

  fn create_test_ico(dir: &tempfile::TempDir) -> std::path::PathBuf {
    let path = dir.path().join("input.ico");
    let img = DynamicImage::new_rgba8(2, 2);
    img.save(&path).unwrap();
    path
  }

  #[test]
  fn ico_converter_converts_to_png() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_ico(&dir);
    let output = dir.path().join("out.png");
    let result = ICOConverter.convert(&input, &output, ImageFormat::PNG, vec![]);
    assert!(result.is_ok(), "conversion failed: {:?}", result.err());
    assert!(output.exists());
  }

  #[test]
  fn ico_converter_converts_to_jpg() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_ico(&dir);
    let output = dir.path().join("out.jpg");
    let result = ICOConverter.convert(&input, &output, ImageFormat::JPG, vec![]);
    assert!(result.is_ok(), "conversion failed: {:?}", result.err());
    assert!(output.exists());
  }

  #[test]
  fn ico_converter_converts_to_webp() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_ico(&dir);
    let output = dir.path().join("out.webp");
    let result = ICOConverter.convert(&input, &output, ImageFormat::WEBP, vec![]);
    assert!(result.is_ok(), "conversion failed: {:?}", result.err());
    assert!(output.exists());
  }
}
