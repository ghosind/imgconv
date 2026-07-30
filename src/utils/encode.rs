use std::path::Path;

use image::{DynamicImage, ImageEncoder, ExtendedColorType};
use image::codecs::jpeg::JpegEncoder;

use crate::converter::options::ConverterOptions;
use crate::core::format::ImageFormat;
use crate::error::convert::ImageConvertError;

/// Encodes a [`DynamicImage`] to the specified format and writes it to disk.
///
/// Supports in-place processing (e.g., resizing) via `processors` before encoding.
///
/// Supported output formats: PNG, JPG, WEBP, AVIF, BMP, ICO, TIFF.
/// SVG output is rejected with an [`ImageConvertError::UnsupportedFormat`].
///
/// When a `quality` value is set in `options`, it is used for lossy formats
/// that support it (JPEG only for now). For other formats the encoder's default is used.
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
    ImageFormat::JPG => {
      if let Some(q) = options.quality {
        let rgb = img.to_rgb8();
        let (width, height) = rgb.dimensions();
        JpegEncoder::new_with_quality(&mut writer, q)
          .write_image(&rgb, width, height, ExtendedColorType::Rgb8)?;
      } else {
        img.write_to(&mut writer, format.image_format().unwrap())?;
      }
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
      quality: None,
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
      quality: None,
    };
    let result = encode_image(&mut img, &out, &opts);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn encode_jpg_with_quality() {
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test_quality.jpg");
    let opts = ConverterOptions {
      target_format: ImageFormat::JPG,
      processors: vec![],
      overwrite: false,
      quality: Some(85),
    };
    let result = encode_image(&mut img, &out, &opts);
    assert!(result.is_ok());
    assert!(out.exists());
    // Should produce a valid JPEG
    let decoded = image::open(&out).unwrap();
    assert_eq!(decoded.width(), 4);
    assert_eq!(decoded.height(), 4);
  }

  #[test]
  fn encode_jpg_high_quality_larger_than_low() {
    let mut img = DynamicImage::new_rgba8(100, 100);
    let dir = tempfile::tempdir().unwrap();

    // Encode with low quality
    let low_out = dir.path().join("low.jpg");
    let low_opts = ConverterOptions {
      target_format: ImageFormat::JPG,
      processors: vec![],
      overwrite: false,
      quality: Some(10),
    };
    encode_image(&mut img, &low_out, &low_opts).unwrap();

    // Encode with high quality
    let high_out = dir.path().join("high.jpg");
    let high_opts = ConverterOptions {
      target_format: ImageFormat::JPG,
      processors: vec![],
      overwrite: false,
      quality: Some(95),
    };
    encode_image(&mut img, &high_out, &high_opts).unwrap();

    let low_size = low_out.metadata().unwrap().len();
    let high_size = high_out.metadata().unwrap().len();
    assert!(
      high_size > low_size,
      "high quality ({}) should produce larger file than low quality ({})",
      high_size,
      low_size,
    );
  }

  #[test]
  fn encode_png_with_quality_ignored() {
    // PNG is lossless — quality is ignored, but should still produce valid output
    let mut img = make_test_image();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("test.png");
    let opts = ConverterOptions {
      target_format: ImageFormat::PNG,
      processors: vec![],
      overwrite: false,
      quality: Some(50),
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
      quality: None,
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
      quality: None,
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
      quality: None,
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
      quality: None,
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
      quality: None,
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
      quality: None,
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
      quality: None,
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
    let processor = crate::processor::resize::ResizeProcessor::new(Some(32), None, None);
    let opts = ConverterOptions {
      target_format: ImageFormat::PNG,
      processors: vec![Box::new(processor)],
      overwrite: false,
      quality: None,
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
