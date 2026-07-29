use crate::core::format::ImageFormat;
use crate::core::traits::ImageProcessor;

/// Options for configuring the image conversion process.
///
/// This struct consolidates all conversion parameters that were previously
/// passed as individual arguments through the dispatch and convert chain.
#[derive(Debug)]
pub struct ConverterOptions {
  pub target_format: ImageFormat,
  pub processors: Vec<Box<dyn ImageProcessor>>,
  pub overwrite: bool,
}
