use std::path::Path;

use crate::core::format::ImageFormat;
use crate::error::convert::ImageConvertError;
use crate::utils::encode::encode_image;

pub(crate) fn convert(
  input_path: &Path,
  output_path: &Path,
  target_format: ImageFormat,
) -> Result<(), ImageConvertError> {
  let mut img = image::open(input_path)?;

  encode_image(&img, target_format, output_path)
}
