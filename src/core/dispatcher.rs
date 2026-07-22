use std::path::Path;

use crate::core::convert;
use crate::core::format::ImageFormat;
use crate::error::convert::ImageConvertError;

/// Validates inputs and orchestrates the full conversion workflow.
///
/// Steps performed:
/// 1. Checks that the input file exists.
/// 2. Rejects self-overwrites.
/// 3. Determines the input format from the file extension.
/// 4. Validates the input-to-output format compatibility.
/// 5. Ensures the output file does not already exist (unless `overwrite` is set).
/// 6. Looks up the appropriate converter and runs the conversion.
pub fn dispatch(
  input_path: &Path,
  output_path: &Path,
  output_format: ImageFormat,
  overwrite: bool,
) -> Result<(), ImageConvertError> {
  if !input_path.exists() {
    return Err(ImageConvertError::FileNotFound(
      input_path.display().to_string(),
    ));
  }

  // Prevent self-overwrite: input and output must not be the same file.
  // This guards against data loss when the user specifies the same path
  // for both input and output (e.g., imgconv convert a.png -f png -O).
  if input_path == output_path {
    return Err(ImageConvertError::ProcessingError(
      "Same input and output paths are not allowed.".into(),
    ));
  }

  let input_format = ImageFormat::from_extension(input_path)?;

  ImageFormat::validate(input_format, output_format)?;

  if !overwrite && output_path.exists() {
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

  fn create_temp_svg(dir: &tempfile::TempDir) -> std::path::PathBuf {
    let path = dir.path().join("test.svg");
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64">
      <rect width="64" height="64" fill="blue"/>
    </svg>"#;
    std::fs::write(&path, svg).unwrap();
    path
  }

  #[test]
  fn dispatch_file_not_found() {
    let result = dispatch(
      Path::new("/nonexistent/path/file.png"),
      Path::new("/tmp/output.png"),
      ImageFormat::PNG,
      false,
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
    let result = dispatch(&bad_input, &out, ImageFormat::PNG, false);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ImageConvertError::UnsupportedFormat(_)));
  }

  #[test]
  fn dispatch_svg_output_rejected() {
    let (input, _dir) = create_temp_png();
    let out = _dir.path().join("out.svg");
    let result = dispatch(&input, &out, ImageFormat::SVG, false);
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
    let result = dispatch(&input, &out, ImageFormat::PNG, false);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ImageConvertError::FileExists(_)));
  }

  #[test]
  fn dispatch_success_png_to_jpg() {
    let (input, _dir) = create_temp_png();
    let out = _dir.path().join("out.jpg");
    let result = dispatch(&input, &out, ImageFormat::JPG, false);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn dispatch_success_png_to_webp() {
    let (input, _dir) = create_temp_png();
    let out = _dir.path().join("out.webp");
    let result = dispatch(&input, &out, ImageFormat::WEBP, false);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn dispatch_success_png_to_png() {
    let (input, _dir) = create_temp_png();
    let out = _dir.path().join("out.png");
    let result = dispatch(&input, &out, ImageFormat::PNG, false);
    assert!(result.is_ok());
    assert!(out.exists());
  }

  #[test]
  fn dispatch_success_svg_to_png() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_temp_svg(&dir);
    let out = dir.path().join("out.png");
    let result = dispatch(&input, &out, ImageFormat::PNG, false);
    assert!(result.is_ok());
    assert!(out.exists());
    assert!(out.metadata().unwrap().len() > 0);
  }

  #[test]
  fn dispatch_success_svg_to_jpg() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_temp_svg(&dir);
    let out = dir.path().join("out.jpg");
    let result = dispatch(&input, &out, ImageFormat::JPG, false);
    assert!(result.is_ok());
    assert!(out.exists());
    assert!(out.metadata().unwrap().len() > 0);
  }

  #[test]
  fn dispatch_success_svg_to_webp() {
    let dir = tempfile::tempdir().unwrap();
    let input = create_temp_svg(&dir);
    let out = dir.path().join("out.webp");
    let result = dispatch(&input, &out, ImageFormat::WEBP, false);
    assert!(result.is_ok());
    assert!(out.exists());
    assert!(out.metadata().unwrap().len() > 0);
  }

  #[test]
  fn dispatch_overwrite_existing_file() {
    let (input, _dir) = create_temp_png();
    let out = _dir.path().join("out.png");
    // Create output file in advance (same as dispatch_output_file_already_exists)
    std::fs::write(&out, b"dummy").unwrap();
    // With overwrite=true, this should succeed instead of returning FileExists
    let result = dispatch(&input, &out, ImageFormat::PNG, true);
    assert!(result.is_ok());
    assert!(out.exists());
    // Content should be the converted image, not the dummy bytes
    assert!(out.metadata().unwrap().len() > 10);
  }

  #[test]
  fn dispatch_overwrite_false_rejects_existing() {
    let (input, _dir) = create_temp_png();
    let out = _dir.path().join("out.png");
    std::fs::write(&out, b"dummy").unwrap();
    let result = dispatch(&input, &out, ImageFormat::PNG, false);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ImageConvertError::FileExists(_)));
  }

  #[test]
  fn dispatch_same_file_rejected() {
    let (input, _dir) = create_temp_png();
    // Input and output are the same path — should be rejected
    let result = dispatch(&input, &input, ImageFormat::PNG, false);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Same input and output paths are not allowed"));
  }

  #[test]
  fn dispatch_same_file_rejected_even_with_overwrite() {
    let (input, _dir) = create_temp_png();
    // Even with overwrite=true, same-file should be rejected
    let result = dispatch(&input, &input, ImageFormat::PNG, true);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Same input and output paths are not allowed"));
  }
}
