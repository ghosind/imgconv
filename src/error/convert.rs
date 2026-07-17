use std::error::{Error as StdError};
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
