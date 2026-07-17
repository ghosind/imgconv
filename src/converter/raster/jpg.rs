use std::path::Path;

use crate::core::format::ImageFormat;
use crate::core::traits::ImageConverter;
use crate::error::convert::ImageConvertError;
use crate::converter::raster::util::convert;

pub struct JPGConverter;

impl ImageConverter for JPGConverter {
  fn convert(
    &self,
    input_path: &Path,
    output_path: &Path,
    target_format: ImageFormat,
  ) -> Result<(), ImageConvertError> {
    convert(input_path, output_path, target_format)
  }
}
