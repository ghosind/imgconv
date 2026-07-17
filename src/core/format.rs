use std::path::Path;

use crate::error::convert::ImageConvertError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
  PNG,
  JPG,
  WEBP,
  SVG,
}

impl ImageFormat {
  pub fn from_extension(path: &Path) -> Result<Self, ImageConvertError> {
    match path.extension().and_then(|e| e.to_str()) {
      Some(ext) => match ext.to_lowercase().as_str() {
        "png" => Ok(ImageFormat::PNG),
        "jpg" | "jpeg" => Ok(ImageFormat::JPG),
        "webp" => Ok(ImageFormat::WEBP),
        "svg" => Ok(ImageFormat::SVG),
        _ => Err(ImageConvertError::UnsupportedFormat(ext.into())),
      },
      None => Err(ImageConvertError::UnsupportedFormat(
        "Cannot determine file extension. Please ensure the input file has a valid suffix.".into(),
      )),
    }
  }

  pub fn from_str(s: &str) -> Result<Self, ImageConvertError> {
    match s.to_lowercase().as_str() {
      "png" => Ok(ImageFormat::PNG),
      "jpg" | "jpeg" => Ok(ImageFormat::JPG),
      "webp" => Ok(ImageFormat::WEBP),
      "svg" => Ok(ImageFormat::SVG),
      _ => Err(ImageConvertError::UnsupportedFormat(s.into())),
    }
  }

  pub fn extension(&self) -> &str {
    match self {
      ImageFormat::PNG => "png",
      ImageFormat::JPG => "jpg",
      ImageFormat::WEBP => "webp",
      ImageFormat::SVG => "svg",
    }
  }

  pub fn validate(
    _input_format: ImageFormat,
    target_format: ImageFormat,
  ) -> Result<(), ImageConvertError> {
    if target_format == ImageFormat::SVG {
      return Err(ImageConvertError::UnsupportedFormat(
        "SVG output is not supported".into(),
      ));
    }
    Ok(())
  }
}
