use std::path::Path;

use image::DynamicImage;

use crate::core::format::ImageFormat;
use crate::core::traits::ImageProcessor;
use crate::error::convert::ImageConvertError;

/// Encodes a [`DynamicImage`] to the specified format and writes it to disk.
///
/// Supports in-place processing (e.g., resizing) via `processors` before encoding.
///
/// Supported output formats: PNG, JPG, WEBP.
/// SVG output is rejected with an [`ImageConvertError::UnsupportedFormat`].
pub fn encode_image(
  img: &mut DynamicImage,
  format: ImageFormat,
  output_path: &Path,
  processors: Vec<Box<dyn ImageProcessor>>,
) -> Result<(), ImageConvertError> {
  for processor in processors {
    processor.process(img)?;
  }

  let file = std::fs::File::create(output_path)?;
  let mut writer = std::io::BufWriter::new(file);

  match format {
    ImageFormat::SVG => {
      return Err(ImageConvertError::UnsupportedFormat(
        "SVG output is not supported.".into(),
      ));
    }
    format => {
      if let Some(image_format) = format.image_format() {
        img.write_to(&mut writer, image_format)?;
      } else {
        return Err(ImageConvertError::UnsupportedFormat(format!(
          "Output format {:?} is not supported.",
          format
        )));
      }
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
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.png");
    let result = encode_image(&mut img, ImageFormat::PNG, &out, vec![]);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_jpg_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.jpg");
    let result = encode_image(&mut img, ImageFormat::JPG, &out, vec![]);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_webp_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.webp");
    let result = encode_image(&mut img, ImageFormat::WEBP, &out, vec![]);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_ico_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.ico");
    let result = encode_image(&mut img, ImageFormat::ICO, &out, vec![]);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_avif_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.avif");
    let result = encode_image(&mut img, ImageFormat::AVIF, &out, vec![]);
    assert!(result.is_ok(), "AVIF encode failed: {:?}", result.err());
    assert!(out.exists());
  }

  #[test]
  fn encode_bmp_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.bmp");
    let result = encode_image(&mut img, ImageFormat::BMP, &out, vec![]);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_tiff_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.tiff");
    let result = encode_image(&mut img, ImageFormat::TIFF, &out, vec![]);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_svg_rejected() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.svg");
    let result = encode_image(&mut img, ImageFormat::SVG, &out, vec![]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not supported"));
  }

  #[test]
  fn encode_to_nonexistent_directory_fails() {
    let mut img = make_test_image();
    let result = encode_image(
      &mut img,
      ImageFormat::PNG,
      std::path::Path::new("/nonexistent_dir_xyz/test.png"),
      vec![],
    );
    assert!(result.is_err());
  }

  #[test]
  fn encode_with_resize_processor() {
    let mut img = image::DynamicImage::new_rgba8(100, 50);
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.png");
    let processor = crate::processor::resize::ResizeProcessor::new(Some(32), None);
    let result = encode_image(&mut img, ImageFormat::PNG, &out, vec![Box::new(processor)]);
    assert!(result.is_ok());
    assert!(out.exists());
    // Verify the resize was applied
    let saved = image::open(&out).unwrap();
    assert_eq!(saved.width(), 32);
    assert_eq!(saved.height(), 16); // aspect ratio preserved
  }
}
