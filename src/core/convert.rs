use crate::core::format::ImageFormat;
use crate::core::traits::ImageConverter;
use crate::converter::raster::{AVIFConverter, BMPConverter, JPGConverter, PNGConverter, TIFFConverter, WEBPConverter};
use crate::converter::vector::SVGConverter;

/// Returns the appropriate [`ImageConverter`] implementation for the given format.
///
/// This acts as a simple factory / registry mapping each input format to its converter.
pub fn get_converter(format: ImageFormat) -> Box<dyn ImageConverter> {
  match format {
    ImageFormat::AVIF => Box::new(AVIFConverter),
    ImageFormat::BMP => Box::new(BMPConverter),
    ImageFormat::JPG => Box::new(JPGConverter),
    ImageFormat::PNG => Box::new(PNGConverter),
    ImageFormat::TIFF => Box::new(TIFFConverter),
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
