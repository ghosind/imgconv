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
