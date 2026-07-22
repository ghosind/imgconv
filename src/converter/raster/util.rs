use std::path::Path;

use crate::core::format::ImageFormat;
use crate::core::traits::ImageProcessor;
use crate::error::convert::ImageConvertError;
use crate::utils::encode::encode_image;

/// Opens a raster image from disk and encodes it to the target format.
///
/// This is a shared helper used by the raster converters (PNG, JPG, WEBP) to avoid duplicating
/// the same logic.
pub(crate) fn convert(
  input_path: &Path,
  output_path: &Path,
  target_format: ImageFormat,
  processors: Vec<Box<dyn ImageProcessor>>,
) -> Result<(), ImageConvertError> {
  let mut img = image::open(input_path)?;

  encode_image(&mut img, target_format, output_path, processors)
}

#[cfg(test)]
mod tests {
  use super::*;
  use image::DynamicImage;

  fn create_test_png(dir: &tempfile::TempDir) -> std::path::PathBuf {
    let path = dir.path().join("input.png");
    let img = DynamicImage::new_rgba8(4, 4);
    img.save(&path).unwrap();
    path
  }

  #[test]
  fn convert_png_to_jpg() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_png(&dir);
    let output = dir.path().join("out.jpg");
    let result = convert(&input, &output, ImageFormat::JPG, vec![]);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn convert_png_to_webp() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_png(&dir);
    let output = dir.path().join("out.webp");
    let result = convert(&input, &output, ImageFormat::WEBP, vec![]);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn convert_png_to_png() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_png(&dir);
    let output = dir.path().join("out.png");
    let result = convert(&input, &output, ImageFormat::PNG, vec![]);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn convert_nonexistent_input_fails() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("missing.png");
    let output = dir.path().join("out.png");
    let result = convert(&input, &output, ImageFormat::PNG, vec![]);
    assert!(result.is_err());
  }
}
