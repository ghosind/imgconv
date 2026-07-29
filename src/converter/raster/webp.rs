use std::path::Path;

use crate::converter::options::ConverterOptions;
use crate::converter::raster::util::convert;
use crate::core::traits::ImageConverter;
use crate::error::convert::ImageConvertError;

/// Converter for WEBP input images.
///
/// Implements [`ImageConverter`] by delegating to the shared raster utility.
pub struct WEBPConverter;

impl ImageConverter for WEBPConverter {
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
    let opts = ConverterOptions {
      target_format: ImageFormat::PNG,
      processors: vec![],
      overwrite: false,
    };
    let result = WEBPConverter.convert(&input, &output, &opts);
    assert!(result.is_ok());
    assert!(output.exists());
  }

  #[test]
  fn webp_converter_converts_to_jpg() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_test_webp(&dir);
    let output = dir.path().join("out.jpg");
    let opts = ConverterOptions {
      target_format: ImageFormat::JPG,
      processors: vec![],
      overwrite: false,
    };
    let result = WEBPConverter.convert(&input, &output, &opts);
    assert!(result.is_ok());
    assert!(output.exists());
  }
}
