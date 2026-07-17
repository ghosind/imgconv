use std::path::Path;

use crate::core::format::ImageFormat;
use crate::core::traits::ImageConverter;
use crate::error::convert::ImageConvertError;

pub struct SVGConverter;

impl ImageConverter for SVGConverter {
  fn convert(
    &self,
    input_path: &Path,
    output_path: &Path,
    target_format: ImageFormat,
  ) -> Result<(), ImageConvertError> {
    // TODO
    unimplemented!()
  }
}
