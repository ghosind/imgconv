use std::path::Path;

use crate::core::format::ImageFormat;
use crate::core::traits::ImageConverter;
use crate::error::convert::ImageConvertError;
use crate::converter::raster::util::convert;

pub struct JPGConverter;

impl ImageConverter for JPGConverter {
  fn convert(
    &self,
    input_path: &Path,
    output_path: &Path,
    target_format: ImageFormat,
  ) -> Result<(), ImageConvertError> {
    convert(input_path, output_path, target_format)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use image::DynamicImage;
  use crate::core::traits::ImageConverter;

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
    let result = JPGConverter.convert(&input, &output, ImageFormat::PNG);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn jpg_converter_converts_to_webp() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_jpg(&dir);
    let output = dir.path().join("out.webp");
    let result = JPGConverter.convert(&input, &output, ImageFormat::WEBP);
    assert!(result.is_ok());
    assert!(output.exists());
  }
}
