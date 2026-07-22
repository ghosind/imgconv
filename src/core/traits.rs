use image::DynamicImage;

use crate::core::format::ImageFormat;
use crate::error::convert::ImageConvertError;

/// Trait for image format converters.
///
/// Each image format (PNG, JPG, WEBP, SVG) provides its own implementation
/// that knows how to read its input format and produce the requested output.
pub trait ImageConverter {
  /// Converts an image from one format to another.
  ///
  /// # Arguments
  /// * `input_path`  - Path to the source image file.
  /// * `output_path` - Path where the converted image will be written.
  /// * `target_format` - The desired output image format.
  ///
  /// # Errors
  /// Returns [`ImageConvertError`] if the input cannot be read, the conversion
  /// fails, or the output cannot be written.
  fn convert(
    &self,
    input_path: &std::path::Path,
    output_path: &std::path::Path,
    target_format: ImageFormat,
    processors: Vec<Box<dyn ImageProcessor>>,
  ) -> Result<(), ImageConvertError>;
}

pub trait ImageProcessor: std::fmt::Debug {
  fn process(&self, img: &mut DynamicImage) -> Result<(), ImageConvertError>;
}
