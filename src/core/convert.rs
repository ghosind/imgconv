use crate::core::format::ImageFormat;
use crate::core::traits::ImageConverter;
use crate::error::convert::ImageConvertError;
use crate::converter::raster::{JPGConverter, PNGConverter, WEBPConverter};
use crate::converter::vector::SVGConverter;

pub fn get_converter(format: ImageFormat) -> Box<dyn ImageConverter> {
  match format {
    ImageFormat::PNG => Box::new(PNGConverter),
    ImageFormat::JPG => Box::new(JPGConverter),
    ImageFormat::WEBP => Box::new(WEBPConverter),
    ImageFormat::SVG => Box::new(SVGConverter),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_converter_returns_png_converter() {
    let _c = get_converter(ImageFormat::PNG);
  }

  #[test]
  fn get_converter_returns_jpg_converter() {
    let _c = get_converter(ImageFormat::JPG);
  }

  #[test]
  fn get_converter_returns_webp_converter() {
    let _c = get_converter(ImageFormat::WEBP);
  }

  #[test]
  fn get_converter_returns_svg_converter() {
    let _c = get_converter(ImageFormat::SVG);
  }
}
