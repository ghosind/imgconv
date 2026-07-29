use std::path::Path;

use image::DynamicImage;

use crate::converter::options::ConverterOptions;
use crate::core::format::ImageFormat;
use crate::error::convert::ImageConvertError;

/// Encodes a [`DynamicImage`] to the specified format and writes it to disk.
///
/// Supports in-place processing (e.g., resizing) via `processors` before encoding.
///
/// Supported output formats: PNG, JPG, WEBP.
/// SVG output is rejected with an [`ImageConvertError::UnsupportedFormat`].
pub fn encode_image(
  img: &mut DynamicImage,
  output_path: &Path,
  options: &ConverterOptions,
) -> Result<(), ImageConvertError> {
  for processor in &options.processors {
    processor.process(img)?;
  }

  let format = options.target_format;

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

  fn make_test_image() -> DynamicImage {
    DynamicImage::new_rgba8(4, 4)
  }

  #[test]
  fn encode_png_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.png");
    let opts = ConverterOptions {
      target_format: ImageFormat::PNG,
      processors: vec![],
      overwrite: false,
    };
    let result = encode_image(&mut img, &out, &opts);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_jpg_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.jpg");
    let opts = ConverterOptions {
      target_format: ImageFormat::JPG,
      processors: vec![],
      overwrite: false,
    };
    let result = encode_image(&mut img, &out, &opts);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_webp_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.webp");
    let opts = ConverterOptions {
      target_format: ImageFormat::WEBP,
      processors: vec![],
      overwrite: false,
    };
    let result = encode_image(&mut img, &out, &opts);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_ico_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.ico");
    let opts = ConverterOptions {
      target_format: ImageFormat::ICO,
      processors: vec![],
      overwrite: false,
    };
    let result = encode_image(&mut img, &out, &opts);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_avif_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.avif");
    let opts = ConverterOptions {
      target_format: ImageFormat::AVIF,
      processors: vec![],
      overwrite: false,
    };
    let result = encode_image(&mut img, &out, &opts);
    assert!(result.is_ok(), "AVIF encode failed: {:?}", result.err());
    assert!(out.exists());
  }

  #[test]
  fn encode_bmp_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.bmp");
    let opts = ConverterOptions {
      target_format: ImageFormat::BMP,
      processors: vec![],
      overwrite: false,
    };
    let result = encode_image(&mut img, &out, &opts);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_tiff_works() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.tiff");
    let opts = ConverterOptions {
      target_format: ImageFormat::TIFF,
      processors: vec![],
      overwrite: false,
    };
    let result = encode_image(&mut img, &out, &opts);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_svg_rejected() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.svg");
    let opts = ConverterOptions {
      target_format: ImageFormat::SVG,
      processors: vec![],
      overwrite: false,
    };
    let result = encode_image(&mut img, &out, &opts);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("not supported"));
  }

  #[test]
  fn encode_to_nonexistent_directory_fails() {
    let mut img = make_test_image();
    let opts = ConverterOptions {
      target_format: ImageFormat::PNG,
      processors: vec![],
      overwrite: false,
    };
    let result = encode_image(
      &mut img,
      std::path::Path::new("/nonexistent_dir_xyz/test.png"),
      &opts,
    );
    assert!(result.is_err());
  }

  #[test]
  fn encode_with_resize_processor() {
    let mut img = image::DynamicImage::new_rgba8(100, 50);
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.png");
    let processor = crate::processor::resize::ResizeProcessor::new(Some(32), None);
    let opts = ConverterOptions {
      target_format: ImageFormat::PNG,
      processors: vec![Box::new(processor)],
      overwrite: false,
    };
    let result = encode_image(&mut img, &out, &opts);
    assert!(result.is_ok());
    assert!(out.exists());
    // Verify the resize was applied
    let saved = image::open(&out).unwrap();
    assert_eq!(saved.width(), 32);
    assert_eq!(saved.height(), 16); // aspect ratio preserved
  }
}
