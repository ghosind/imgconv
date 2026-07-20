use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageConvertError {
  #[error("file exists: {0}")]
  FileExists(String),

  #[error("File not found: {0}")]
  FileNotFound(String),

  #[error("IO error: {0}")]
  IO(#[from] std::io::Error),

  #[error("Processing error: {0}")]
  ProcessingError(String),

  #[error("SVG render error: {0}")]
  SVGRenderError(String),

  /// Unsupported image format error
  #[error("Unsupported image format: {0}")]
  UnsupportedFormat(String),
}

impl From<image::ImageError> for ImageConvertError {
  fn from(err: image::ImageError) -> Self {
    match err {
      _ => ImageConvertError::ProcessingError(err.to_string()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn file_exists_display() {
    let err = ImageConvertError::FileExists("test.png".into());
    assert!(err.to_string().contains("test.png"));
    assert!(err.to_string().contains("file exists"));
  }

  #[test]
  fn file_not_found_display() {
    let err = ImageConvertError::FileNotFound("missing.png".into());
    assert!(err.to_string().contains("missing.png"));
    assert!(err.to_string().contains("File not found"));
  }

  #[test]
  fn io_error_display() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "no such file");
    let err = ImageConvertError::IO(io_err);
    assert!(err.to_string().contains("IO error"));
  }

  #[test]
  fn processing_error_display() {
    let err = ImageConvertError::ProcessingError("something went wrong".into());
    assert!(err.to_string().contains("something went wrong"));
    assert!(err.to_string().contains("Processing error"));
  }

  #[test]
  fn svg_render_error_display() {
    let err = ImageConvertError::SVGRenderError("render failed".into());
    assert!(err.to_string().contains("render failed"));
    assert!(err.to_string().contains("SVG render error"));
  }

  #[test]
  fn unsupported_format_display() {
    let err = ImageConvertError::UnsupportedFormat("bmp".into());
    assert!(err.to_string().contains("bmp"));
    assert!(err.to_string().contains("Unsupported image format"));
  }

  #[test]
  fn from_io_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::Other, "test");
    let conv_err: ImageConvertError = io_err.into();
    assert!(matches!(conv_err, ImageConvertError::IO(_)));
  }

  #[test]
  fn from_image_error() {
    // Create an invalid "image" file that image crate will reject
    let dir = tempfile::tempdir().unwrap();
    let bad_file = dir.path().join("bad.png");
    std::fs::write(&bad_file, b"this is not a valid image").unwrap();
    let result = image::open(&bad_file);
    if let Err(img_err) = result {
      let conv_err: ImageConvertError = img_err.into();
      assert!(matches!(conv_err, ImageConvertError::ProcessingError(_)));
      assert!(conv_err.to_string().contains("Processing error"));
    }
  }

  #[test]
  fn error_is_std_error() {
    fn takes_error(_e: &dyn std::error::Error) {}
    let err = ImageConvertError::UnsupportedFormat("test".into());
    takes_error(&err);
  }

  #[test]
  fn error_debug_format() {
    let err = ImageConvertError::FileNotFound("x".into());
    let debug_str = format!("{:?}", err);
    assert!(debug_str.contains("FileNotFound"));
  }
}
