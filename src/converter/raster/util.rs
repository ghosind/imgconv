use std::path::Path;

use crate::converter::options::ConverterOptions;
use crate::error::convert::ImageConvertError;
use crate::utils::encode::encode_image;

/// Opens a raster image from disk and encodes it to the target format.
///
/// This is a shared helper used by the raster converters (PNG, JPG, WEBP) to avoid duplicating
/// the same logic.
pub(crate) fn convert(
  input_path: &Path,
  output_path: &Path,
  options: &ConverterOptions,
) -> Result<(), ImageConvertError> {
  let mut img = image::open(input_path)?;

  encode_image(&mut img, output_path, options)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::format::ImageFormat;
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
    let opts = ConverterOptions {
      target_format: ImageFormat::JPG,
      processors: vec![],
      overwrite: false,
      quality: None,
    };
    let result = convert(&input, &output, &opts);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn convert_png_to_webp() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_png(&dir);
    let output = dir.path().join("out.webp");
    let opts = ConverterOptions {
      target_format: ImageFormat::WEBP,
      processors: vec![],
      overwrite: false,
      quality: None,
    };
    let result = convert(&input, &output, &opts);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn convert_png_to_png() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_png(&dir);
    let output = dir.path().join("out.png");
    let opts = ConverterOptions {
      target_format: ImageFormat::PNG,
      processors: vec![],
      overwrite: false,
      quality: None,
    };
    let result = convert(&input, &output, &opts);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn convert_nonexistent_input_fails() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("missing.png");
    let output = dir.path().join("out.png");
    let opts = ConverterOptions {
      target_format: ImageFormat::PNG,
      processors: vec![],
      overwrite: false,
      quality: None,
    };
    let result = convert(&input, &output, &opts);
    assert!(result.is_err());
  }
}
