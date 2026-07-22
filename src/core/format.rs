use std::path::Path;

use crate::error::convert::ImageConvertError;

/// Supported image formats for input and output.
///
/// Each variant represents a distinct image file format that the application
/// can read and/or write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
  // Raster formats (input and output supported)

  /// AV1 Image File Format (AVIF).
  AVIF,
  /// Bitmap format (BMP).
  BMP,
  /// JPEG format (supports both `.jpg` and `.jpeg` extensions).
  JPG,
  /// Portable Network Graphics format.
  PNG,
  /// Tagged Image File Format.
  TIFF,
  /// WebP format.
  WEBP,

  // Vector format (input-only; output not supported)

  /// Scalable Vector Graphics format (input-only; SVG output is not supported).
  SVG,
}

impl ImageFormat {
  /// Infers the image format from a file's extension.
  ///
  /// Returns `UnsupportedFormat` if the extension is missing or unrecognized.
  pub fn from_extension(path: &Path) -> Result<Self, ImageConvertError> {
    match path.extension().and_then(|e| e.to_str()) {
      Some(ext) => match ext.to_lowercase().as_str() {
        "avif" => Ok(ImageFormat::AVIF),
        "bmp" => Ok(ImageFormat::BMP),
        "jpg" | "jpeg" => Ok(ImageFormat::JPG),
        "png" => Ok(ImageFormat::PNG),
        "tif" | "tiff" => Ok(ImageFormat::TIFF),
        "webp" => Ok(ImageFormat::WEBP),
        "svg" => Ok(ImageFormat::SVG),
        _ => Err(ImageConvertError::UnsupportedFormat(ext.into())),
      },
      None => Err(ImageConvertError::UnsupportedFormat(
        "Cannot determine file extension. Please ensure the input file has a valid suffix.".into(),
      )),
    }
  }

  /// Parses an image format from a string (e.g., `"png"`, `"jpg"`, `"webp"`, `"svg"`).
  ///
  /// Case-insensitive. Returns `UnsupportedFormat` if the string does not match.
  pub fn from_str(s: &str) -> Result<Self, ImageConvertError> {
    match s.to_lowercase().as_str() {
      "avif" => Ok(ImageFormat::AVIF),
      "bmp" => Ok(ImageFormat::BMP),
      "jpg" | "jpeg" => Ok(ImageFormat::JPG),
      "png" => Ok(ImageFormat::PNG),
      "tif" | "tiff" => Ok(ImageFormat::TIFF),
      "webp" => Ok(ImageFormat::WEBP),
      "svg" => Ok(ImageFormat::SVG),
      _ => Err(ImageConvertError::UnsupportedFormat(s.into())),
    }
  }

  /// Returns the standard file extension for this format (e.g., `"png"`, `"jpg"`, `"webp"`, `"svg"`).
  pub fn extension(&self) -> &str {
    match self {
      ImageFormat::AVIF => "avif",
      ImageFormat::BMP => "bmp",
      ImageFormat::JPG => "jpg",
      ImageFormat::PNG => "png",
      ImageFormat::TIFF => "tiff",
      ImageFormat::WEBP => "webp",
      ImageFormat::SVG => "svg",
    }
  }

  /// Validates that the combination of input and target formats is supported.
  ///
  /// Currently, SVG output is rejected since the application cannot encode to SVG.
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::Path;

  #[test]
  fn from_extension_jpg() {
    assert_eq!(
      ImageFormat::from_extension(Path::new("photo.jpg")).unwrap(),
      ImageFormat::JPG
    );
  }

  #[test]
  fn from_extension_jpeg() {
    assert_eq!(
      ImageFormat::from_extension(Path::new("photo.jpeg")).unwrap(),
      ImageFormat::JPG
    );
  }

  #[test]
  fn from_extension_png() {
    assert_eq!(
      ImageFormat::from_extension(Path::new("image.png")).unwrap(),
      ImageFormat::PNG
    );
  }

  #[test]
  fn from_extension_webp() {
    assert_eq!(
      ImageFormat::from_extension(Path::new("img.webp")).unwrap(),
      ImageFormat::WEBP
    );
  }

  #[test]
  fn from_extension_svg() {
    assert_eq!(
      ImageFormat::from_extension(Path::new("icon.svg")).unwrap(),
      ImageFormat::SVG
    );
  }

  #[test]
  fn from_extension_case_insensitive() {
    assert_eq!(
      ImageFormat::from_extension(Path::new("IMAGE.PNG")).unwrap(),
      ImageFormat::PNG
    );
  }

  #[test]
  fn from_extension_unsupported() {
    let err = ImageFormat::from_extension(Path::new("file.gif")).unwrap_err();
    assert!(matches!(err, ImageConvertError::UnsupportedFormat(_)));
    assert!(err.to_string().contains("gif"));
  }

  #[test]
  fn from_extension_no_extension() {
    let err = ImageFormat::from_extension(Path::new("file_without_ext")).unwrap_err();
    assert!(matches!(err, ImageConvertError::UnsupportedFormat(_)));
    assert!(err.to_string().contains("Cannot determine file extension"));
  }

  #[test]
  fn from_str_png() {
    assert_eq!(ImageFormat::from_str("png").unwrap(), ImageFormat::PNG);
  }

  #[test]
  fn from_str_jpg() {
    assert_eq!(ImageFormat::from_str("jpg").unwrap(), ImageFormat::JPG);
  }

  #[test]
  fn from_str_jpeg() {
    assert_eq!(ImageFormat::from_str("jpeg").unwrap(), ImageFormat::JPG);
  }

  #[test]
  fn from_str_webp() {
    assert_eq!(ImageFormat::from_str("webp").unwrap(), ImageFormat::WEBP);
  }

  #[test]
  fn from_str_svg() {
    assert_eq!(ImageFormat::from_str("svg").unwrap(), ImageFormat::SVG);
  }

  #[test]
  fn from_str_case_insensitive() {
    assert_eq!(ImageFormat::from_str("PNG").unwrap(), ImageFormat::PNG);
  }

  #[test]
  fn from_str_unsupported() {
    let err = ImageFormat::from_str("gif").unwrap_err();
    assert!(matches!(err, ImageConvertError::UnsupportedFormat(_)));
    assert!(err.to_string().contains("gif"));
  }

  #[test]
  fn extension_png() {
    assert_eq!(ImageFormat::PNG.extension(), "png");
  }

  #[test]
  fn extension_jpg() {
    assert_eq!(ImageFormat::JPG.extension(), "jpg");
  }

  #[test]
  fn extension_webp() {
    assert_eq!(ImageFormat::WEBP.extension(), "webp");
  }

  #[test]
  fn extension_svg() {
    assert_eq!(ImageFormat::SVG.extension(), "svg");
  }

  #[test]
  fn validate_svg_target_is_error() {
    let result = ImageFormat::validate(ImageFormat::PNG, ImageFormat::SVG);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("SVG output is not supported"));
  }

  #[test]
  fn validate_png_target_is_ok() {
    assert!(ImageFormat::validate(ImageFormat::JPG, ImageFormat::PNG).is_ok());
  }

  #[test]
  fn validate_jpg_target_is_ok() {
    assert!(ImageFormat::validate(ImageFormat::PNG, ImageFormat::JPG).is_ok());
  }

  #[test]
  fn validate_webp_target_is_ok() {
    assert!(ImageFormat::validate(ImageFormat::PNG, ImageFormat::WEBP).is_ok());
  }
}
