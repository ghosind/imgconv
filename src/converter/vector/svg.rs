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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::traits::ImageConverter;

  #[test]
  #[should_panic(expected = "not implemented")]
  fn svg_converter_panics() {
    let _ = SVGConverter.convert(
      std::path::Path::new("input.svg"),
      std::path::Path::new("output.png"),
      ImageFormat::PNG,
    );
  }
}
