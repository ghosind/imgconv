use std::path::Path;

use crate::core::format::ImageFormat;
use crate::error::convert::ImageConvertError;
use crate::utils::encode::encode_image;

pub(crate) fn convert(
  input_path: &Path,
  output_path: &Path,
  target_format: ImageFormat,
) -> Result<(), ImageConvertError> {
  let img = image::open(input_path)?;

  encode_image(&img, target_format, output_path)
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
    let result = convert(&input, &output, ImageFormat::JPG);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn convert_png_to_webp() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_png(&dir);
    let output = dir.path().join("out.webp");
    let result = convert(&input, &output, ImageFormat::WEBP);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn convert_png_to_png() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_png(&dir);
    let output = dir.path().join("out.png");
    let result = convert(&input, &output, ImageFormat::PNG);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn convert_nonexistent_input_fails() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("missing.png");
    let output = dir.path().join("out.png");
    let result = convert(&input, &output, ImageFormat::PNG);
    assert!(result.is_err());
  }
}
