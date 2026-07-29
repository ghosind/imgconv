use image::imageops::FilterType;

use crate::core::traits::ImageProcessor;
use crate::error::convert::ImageConvertError;

/// The default resampling filter used when no filter is explicitly specified.
pub const DEFAULT_FILTER: &str = "lanczos3";

/// A processor that resizes an image to the specified dimensions.
///
/// - When both `width` and `height` are specified, the image is resized exactly
///   (may stretch/distort the image).
/// - When only one dimension is specified, the other is calculated to preserve
///   the original aspect ratio.
/// - When neither is specified, the processor is a no-op.
///
/// # Filter support
/// Use the `--filter` flag to select a resampling filter. Supported values:
/// `nearest`, `triangle`, `catmullrom`, `gaussian`, `lanczos3` (default).
///
/// # Panic safety
/// Zero dimensions are rejected with an error. Invalid filter names are also
/// rejected with an error.
#[derive(Debug)]
pub struct ResizeProcessor {
  width: Option<u32>,
  height: Option<u32>,
  filter: Option<String>,
}

impl ResizeProcessor {
  pub fn new(width: Option<u32>, height: Option<u32>, filter: Option<String>) -> Self {
    Self { width, height, filter }
  }

  /// Resolves the filter name to a [`FilterType`], or returns an error if the
  /// name is not recognised.
  fn get_filter(&self) -> Result<FilterType, ImageConvertError> {
    match self.filter.as_deref().map(|s| s.to_lowercase()).as_deref() {
      Some("nearest") => Ok(FilterType::Nearest),
      Some("triangle") => Ok(FilterType::Triangle),
      Some("catmullrom") => Ok(FilterType::CatmullRom),
      Some("gaussian") => Ok(FilterType::Gaussian),
      Some("lanczos3") => Ok(FilterType::Lanczos3),
      Some(name) => Err(ImageConvertError::ProcessingError(format!(
        "Unknown filter '{}'.",
        name,
      ))),
      None => Ok(FilterType::Lanczos3), // Default filter
    }
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

    // Validate filter name before attempting resize.
    let filter = self.get_filter()?;

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

    if new_width == orig_width && new_height == orig_height && self.filter.is_none() {
      return Ok(());
    }

    *img = img.resize_exact(new_width, new_height, filter);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_test_image() -> image::DynamicImage {
    image::DynamicImage::new_rgba8(100, 50)
  }

  // --- Basic resize tests ---

  #[test]
  fn resize_exact_both_dimensions() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(80), Some(40), None);
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 80);
    assert_eq!(img.height(), 40);
  }

  #[test]
  fn resize_width_only_preserves_aspect_ratio() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(50), None, None);
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 50);
    assert_eq!(img.height(), 25); // 50 / (100/50) = 25
  }

  #[test]
  fn resize_height_only_preserves_aspect_ratio() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(None, Some(25), None);
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 50); // 25 * (100/50) = 50
    assert_eq!(img.height(), 25);
  }

  #[test]
  fn resize_noop_when_same_dimensions() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(100), Some(50), None);
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 100);
    assert_eq!(img.height(), 50);
  }

  #[test]
  fn resize_noop_when_both_none() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(None, None, None);
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 100);
    assert_eq!(img.height(), 50);
  }

  #[test]
  fn resize_rejects_zero_width() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(0), None, None);
    let result = processor.process(&mut img);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("greater than 0"));
  }

  #[test]
  fn resize_rejects_zero_height() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(None, Some(0), None);
    let result = processor.process(&mut img);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("greater than 0"));
  }

  #[test]
  fn resize_rejects_zero_both() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(0), Some(0), None);
    let result = processor.process(&mut img);
    assert!(result.is_err());
  }

  // --- Filter tests ---

  #[test]
  fn default_filter_is_lanczos3() {
    let processor = ResizeProcessor::new(Some(10), None, None);
    assert_eq!(processor.get_filter().unwrap(), FilterType::Lanczos3);
  }

  #[test]
  fn filter_nearest() {
    let processor = ResizeProcessor::new(Some(10), None, Some("nearest".into()));
    assert_eq!(processor.get_filter().unwrap(), FilterType::Nearest);
  }

  #[test]
  fn filter_triangle() {
    let processor = ResizeProcessor::new(Some(10), None, Some("triangle".into()));
    assert_eq!(processor.get_filter().unwrap(), FilterType::Triangle);
  }

  #[test]
  fn filter_catmullrom() {
    let processor = ResizeProcessor::new(Some(10), None, Some("catmullrom".into()));
    assert_eq!(processor.get_filter().unwrap(), FilterType::CatmullRom);
  }

  #[test]
  fn filter_gaussian() {
    let processor = ResizeProcessor::new(Some(10), None, Some("gaussian".into()));
    assert_eq!(processor.get_filter().unwrap(), FilterType::Gaussian);
  }

  #[test]
  fn filter_lanczos3() {
    let processor = ResizeProcessor::new(Some(10), None, Some("lanczos3".into()));
    assert_eq!(processor.get_filter().unwrap(), FilterType::Lanczos3);
  }

  #[test]
  fn filter_unknown_is_rejected() {
    let processor = ResizeProcessor::new(Some(10), None, Some("bogus".into()));
    let result = processor.get_filter();
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Unknown filter 'bogus'"));
  }

  #[test]
  fn process_rejects_unknown_filter() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(10), None, Some("invalid_filter".into()));
    let result = processor.process(&mut img);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid_filter"));
  }

  #[test]
  fn resize_with_nearest_filter_produces_correct_size() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(40), Some(20), Some("nearest".into()));
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 40);
    assert_eq!(img.height(), 20);
  }

  #[test]
  fn resize_with_lanczos3_filter_produces_correct_size() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(Some(40), Some(20), Some("lanczos3".into()));
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 40);
    assert_eq!(img.height(), 20);
  }

  #[test]
  fn filter_is_case_insensitive() {
    let processor = ResizeProcessor::new(Some(10), None, Some("Lanczos3".into()));
    let result = processor.get_filter();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), FilterType::Lanczos3);
  }

  #[test]
  fn filter_is_case_insensitive_all_uppercase() {
    let processor = ResizeProcessor::new(Some(10), None, Some("NEAREST".into()));
    let result = processor.get_filter();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), FilterType::Nearest);
  }

  #[test]
  fn filter_is_case_insensitive_mixed_case() {
    let processor = ResizeProcessor::new(Some(10), None, Some("CaTmUlLrOm".into()));
    let result = processor.get_filter();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), FilterType::CatmullRom);
  }

  #[test]
  fn resize_with_only_filter() {
    let mut img = make_test_image();
    let processor = ResizeProcessor::new(None, None, Some("nearest".into()));
    processor.process(&mut img).unwrap();
    assert_eq!(img.width(), 100);
    assert_eq!(img.height(), 50);
  }
}
