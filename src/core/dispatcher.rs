use std::path::Path;

use crate::core::convert;
use crate::core::format::ImageFormat;
use crate::core::traits::ImageConverter;
use crate::error::convert::ImageConvertError;

pub fn dispatch(
  input_path: &Path,
  output_path: &Path,
) -> Result<(), ImageConvertError> {
  if !input_path.exists() {
    return Err(ImageConvertError::FileNotFound(
      input_path.display().to_string(),
    ));
  }

  let input_format = ImageFormat::from_extension(input_path)?;
  let output_format = ImageFormat::from_extension(output_path)?;

  ImageFormat::validate(input_format, output_format)?;

  if output_path.exists() {
    return Err(ImageConvertError::FileExists(
      output_path.display().to_string(),
    ));
  }

  let converter = convert::get_converter(input_format);

  converter.convert(input_path, output_path, output_format)
}
