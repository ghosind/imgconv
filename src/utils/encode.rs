use std::path::Path;

use image::DynamicImage;

use crate::core::format::ImageFormat;
use crate::error::convert::ImageConvertError;

/// Encodes a [`DynamicImage`] to the specified format and writes it to disk.
///
/// Supported output formats: PNG, JPG, WEBP.
/// SVG output is rejected with an [`ImageConvertError::UnsupportedFormat`].
pub fn encode_image(
  img: &DynamicImage,
  format: ImageFormat,
  output_path: &Path,
) -> Result<(), ImageConvertError> {
  let file = std::fs::File::create(output_path)?;
  let mut writer = std::io::BufWriter::new(file);

  match format {
    ImageFormat::PNG => {
      img.write_to(&mut writer, image::ImageFormat::Png)?;
    }
    ImageFormat::JPG => {
      let mut encoder = image::codecs::jpeg::JpegEncoder::new(&mut writer);
      encoder.encode_image(img)?;
    }
    ImageFormat::WEBP => {
      img.write_to(&mut writer, image::ImageFormat::WebP)?;
    }
    ImageFormat::SVG => {
      return Err(ImageConvertError::UnsupportedFormat(
        "SVG output is not supported.".into(),
      ));
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use image::DynamicImage;

  fn make_test_image() -> DynamicImage {
    DynamicImage::new_rgba8(4, 4)
  }

  #[test]
  fn encode_png_works() {
    let img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.png");
    let result = encode_image(&img, ImageFormat::PNG, &out);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_jpg_works() {
    let img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.jpg");
    let result = encode_image(&img, ImageFormat::JPG, &out);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_webp_works() {
    let img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.webp");
    let result = encode_image(&img, ImageFormat::WEBP, &out);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_to_nonexistent_directory_fails() {
    let img = make_test_image();
    let result = encode_image(&img, ImageFormat::PNG, std::path::Path::new("/nonexistent_dir_xyz/test.png"));
    assert!(result.is_err());
  }
}
