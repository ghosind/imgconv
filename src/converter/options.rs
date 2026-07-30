use crate::core::format::ImageFormat;
use crate::core::traits::ImageProcessor;

/// Options for configuring the image conversion process.
///
/// This struct consolidates all conversion parameters that were previously
/// passed as individual arguments through the dispatch and convert chain.
#[derive(Debug)]
pub struct ConverterOptions {
  /// The target image format for the conversion.
  pub target_format: ImageFormat,
  /// A list of image processors to apply during conversion (e.g., resizing, filtering).
  pub processors: Vec<Box<dyn ImageProcessor>>,
  /// Whether to overwrite the output file if it already exists.
  pub overwrite: bool,
  /// Output image quality (1–100). `None` means do not apply lossy compression.
  pub quality: Option<u8>,
}
