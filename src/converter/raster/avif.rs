use std::path::Path;

use crate::converter::raster::util::convert;
use crate::core::format::ImageFormat;
use crate::core::traits::{ImageConverter, ImageProcessor};
use crate::error::convert::ImageConvertError;

/// Converter for AVIF input images.
///
/// Implements [`ImageConverter`] by delegating to the shared raster utility.
pub struct AVIFConverter;

impl ImageConverter for AVIFConverter {
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

  fn create_test_avif(dir: &tempfile::TempDir) -> std::path::PathBuf {
    let path = dir.path().join("input.avif");
    let img = DynamicImage::new_rgba8(2, 2);
    img.save(&path).unwrap();
    path
  }

  #[test]
  fn avif_converter_converts_to_png() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_avif(&dir);
    let output = dir.path().join("out.png");
    let result = AVIFConverter.convert(&input, &output, ImageFormat::PNG, vec![]);
    assert!(result.is_ok(), "conversion failed: {:?}", result.err());
    assert!(output.exists());
  }

  #[test]
  fn avif_converter_converts_to_jpg() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_avif(&dir);
    let output = dir.path().join("out.jpg");
    let result = AVIFConverter.convert(&input, &output, ImageFormat::JPG, vec![]);
    assert!(result.is_ok(), "conversion failed: {:?}", result.err());
    assert!(output.exists());
  }

  #[test]
  fn avif_converter_converts_to_webp() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_avif(&dir);
    let output = dir.path().join("out.webp");
    let result = AVIFConverter.convert(&input, &output, ImageFormat::WEBP, vec![]);
    assert!(result.is_ok(), "conversion failed: {:?}", result.err());
    assert!(output.exists());
  }
}
