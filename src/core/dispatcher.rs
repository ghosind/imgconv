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

#[cfg(test)]
mod tests {
  use super::*;

  fn create_temp_png() -> (std::path::PathBuf, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.png");
    // Create a minimal valid PNG
    let img = image::DynamicImage::new_rgba8(1, 1);
    img.save(&path).unwrap();
    (path, dir)
  }

  #[test]
  fn dispatch_file_not_found() {
    let result = dispatch(
      Path::new("/nonexistent/path/file.png"),
      Path::new("/tmp/output.png"),
    );
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ImageConvertError::FileNotFound(_)));
  }

  #[test]
  fn dispatch_unsupported_input_extension() {
    let (input, _dir) = create_temp_png();
    // rename to unsupported extension
    let bad_input = _dir.path().join("test.xyz");
    std::fs::rename(&input, &bad_input).unwrap();
    let out = _dir.path().join("out.png");
    let result = dispatch(&bad_input, &out);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ImageConvertError::UnsupportedFormat(_)));
  }

  #[test]
  fn dispatch_svg_output_rejected() {
    let (input, _dir) = create_temp_png();
    let out = _dir.path().join("out.svg");
    let result = dispatch(&input, &out);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("SVG output is not supported"));
  }

  #[test]
  fn dispatch_output_file_already_exists() {
    let (input, _dir) = create_temp_png();
    let out = _dir.path().join("out.png");
    // Create output file in advance
    std::fs::write(&out, b"dummy").unwrap();
    let result = dispatch(&input, &out);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ImageConvertError::FileExists(_)));
  }

  #[test]
  fn dispatch_success_png_to_jpg() {
    let (input, _dir) = create_temp_png();
    let out = _dir.path().join("out.jpg");
    let result = dispatch(&input, &out);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn dispatch_success_png_to_webp() {
    let (input, _dir) = create_temp_png();
    let out = _dir.path().join("out.webp");
    let result = dispatch(&input, &out);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn dispatch_success_png_to_png() {
    let (input, _dir) = create_temp_png();
    let out = _dir.path().join("out.png");
    let result = dispatch(&input, &out);
    assert!(result.is_ok());
    assert!(out.exists());
  }
}
