use image::imageops::FilterType;

use crate::core::traits::ImageProcessor;
use crate::error::convert::ImageConvertError;

/// A processor that resizes an image to the specified dimensions.
///
/// - When both `width` and `height` are specified, the image is resized exactly
///   (may stretch/distort the image).
/// - When only one dimension is specified, the other is calculated to preserve
///   the original aspect ratio.
/// - When neither is specified, the processor is a no-op.
///
/// # Panic safety
/// Zero dimensions are rejected with an error.
#[derive(Debug)]
pub struct ResizeProcessor {
  width: Option<u32>,
  height: Option<u32>,
}

impl ResizeProcessor {
  pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
    Self { width, height }
  }
}

impl ImageProcessor for ResizeProcessor {
  fn process(&self, img: &mut image::DynamicImage) -> Result<(), ImageConvertError> {
    let (orig_width, orig_height) = (img.width(), img.height());

    // Reject zero dimensions to prevent division-by-zero and resize panics.
    if self.width == Some(0) || self.height == Some(0) {
      return Err(ImageConvertError::ProcessingError(
        "Invalid resize dimensions: width and height must be greater than 0.".into(),
      ));
    }

    let (new_width, new_height) = match (self.width, self.height) {
      (Some(w), Some(h)) => (w, h),
      (Some(w), None) => {
        let ratio = w as f64 / orig_width as f64;
        (w, (orig_height as f64 * ratio).round() as u32)
      }
      (None, Some(h)) => {
        let ratio = h as f64 / orig_height as f64;
        ((orig_width as f64 * ratio).round() as u32, h)
      }
      (None, None) => (orig_width, orig_height),
    };

    if new_width == orig_width && new_height == orig_height {
      return Ok(());
    }

    *img = img.resize_exact(new_width, new_height, FilterType::Lanczos3);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_test_image() -> image::DynamicImage {
    image::DynamicImage::new_rgba8(100, 50)
  }

  #[test]
  fn resize_exact_both_dimensions() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(80), Some(40));
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 80);
    assert_eq!(img.height(), 40);
  }

  #[test]
  fn resize_width_only_preserves_aspect_ratio() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(50), None);
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 50);
    assert_eq!(img.height(), 25); // 50 / (100/50) = 25
  }

  #[test]
  fn resize_height_only_preserves_aspect_ratio() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(None, Some(25));
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 50); // 25 * (100/50) = 50
    assert_eq!(img.height(), 25);
  }

  #[test]
  fn resize_noop_when_same_dimensions() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(100), Some(50));
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 100);
    assert_eq!(img.height(), 50);
  }

  #[test]
  fn resize_noop_when_both_none() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(None, None);
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 100);
    assert_eq!(img.height(), 50);
  }

  #[test]
  fn resize_rejects_zero_width() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(0), None);
    let result = processor.process(&mut img);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("greater than 0"));
  }

  #[test]
  fn resize_rejects_zero_height() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(None, Some(0));
    let result = processor.process(&mut img);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("greater than 0"));
  }

  #[test]
  fn resize_rejects_zero_both() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(0), Some(0));
    let result = processor.process(&mut img);
    assert!(result.is_err());
  }
}
