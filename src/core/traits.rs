use std::path::Path;

use image::DynamicImage;

use crate::core::format::ImageFormat;
use crate::error::convert::ImageConvertError;

pub trait ImageConverter {
  fn convert(
    &self,
    input_path: &std::path::Path,
    output_path: &std::path::Path,
    target_format: ImageFormat,
  ) -> Result<(), ImageConvertError>;
}
