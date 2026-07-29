use image::DynamicImage;

use crate::converter::options::ConverterOptions;
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
  /// * `options`     - Configuration options including target format and processors.
  ///
  /// # Errors
  /// Returns [`ImageConvertError`] if the input cannot be read, the conversion
  /// fails, or the output cannot be written.
  fn convert(
    &self,
    input_path: &std::path::Path,
    output_path: &std::path::Path,
    options: &ConverterOptions,
  ) -> Result<(), ImageConvertError>;
}

/// Trait for image processors.
///
/// Each image processor implements a specific image processing operation
/// (e.g., resizing, filtering, color adjustments) that can be applied to an image
/// before or after conversion. Processors can be chained together to perform
/// multiple operations in sequence.
pub trait ImageProcessor: std::fmt::Debug {
  /// Processes the given image in place.
  ///
  /// # Arguments
  /// * `img` - A mutable reference to the image to be processed.
  ///
  /// # Errors
  /// Returns [`ImageConvertError`] if the processing operation fails.
  fn process(&self, img: &mut DynamicImage) -> Result<(), ImageConvertError>;
}
