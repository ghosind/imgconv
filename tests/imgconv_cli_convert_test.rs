use std::process::Command;

/// Helper: create a minimal PNG in a temp directory
fn create_test_png(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
  let path = dir.join(name);
  let img = image::DynamicImage::new_rgba8(4, 4);
  img.save(&path).unwrap();
  path
}

/// Helper: create a minimal ICO in a temp directory
fn create_test_ico(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
  let path = dir.join(name);
  let img = image::DynamicImage::new_rgba8(4, 4);
  img.save(&path).unwrap();
  path
}

/// Helper: create a minimal SVG in a temp directory
fn create_test_svg(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
  let path = dir.join(name);
  let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64">
    <rect width="64" height="64" fill="blue"/>
  </svg>"#;
  std::fs::write(&path, svg).unwrap();
  path
}

/// Helper: create a minimal AVIF in a temp directory
fn create_test_avif(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
  let path = dir.join(name);
  let img = image::DynamicImage::new_rgba8(4, 4);
  img.save(&path).unwrap();
  path
}

/// Helper: create a minimal BMP in a temp directory
fn create_test_bmp(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
  let path = dir.join(name);
  let img = image::DynamicImage::new_rgba8(4, 4);
  img.save(&path).unwrap();
  path
}

/// Helper: create a minimal TIFF in a temp directory
fn create_test_tiff(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
  let path = dir.join(name);
  let img = image::DynamicImage::new_rgba8(4, 4);
  img.save(&path).unwrap();
  path
}

#[test]
fn cli_convert_svg_default() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_svg(dir.path(), "test.svg");
  let expected_output = dir.path().join("test.png");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(expected_output.exists());
  assert!(expected_output.metadata().unwrap().len() > 0);
}

#[test]
fn cli_convert_png_to_jpg_by_output() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.jpg");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
}

#[test]
fn cli_convert_png_to_jpg_by_format() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("input.jpg");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-f")
    .arg("jpg")
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
}

#[test]
fn cli_convert_png_to_webp() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.webp");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
}

#[test]
fn cli_convert_ico_to_png() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_ico(dir.path(), "icon.ico");
  let output = dir.path().join("icon.png");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
  assert!(output.metadata().unwrap().len() > 0);
}

#[test]
fn cli_convert_svg_to_png() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_svg(dir.path(), "icon.svg");
  let output = dir.path().join("icon.png");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
  assert!(output.metadata().unwrap().len() > 0);
}

#[test]
fn cli_convert_svg_to_jpg() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_svg(dir.path(), "logo.svg");
  let output = dir.path().join("logo.jpg");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
  assert!(output.metadata().unwrap().len() > 0);
}

#[test]
fn cli_convert_svg_to_webp() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_svg(dir.path(), "art.svg");
  let output = dir.path().join("art.webp");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
  assert!(output.metadata().unwrap().len() > 0);
}

#[test]
fn cli_convert_png_to_avif() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.avif");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success(), "PNG to AVIF conversion failed");
  assert!(output.exists());
  assert!(output.metadata().unwrap().len() > 0);
}

#[test]
fn cli_convert_png_to_bmp() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.bmp");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
  assert!(output.metadata().unwrap().len() > 0);
}

#[test]
fn cli_convert_png_to_tiff() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.tiff");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
  assert!(output.metadata().unwrap().len() > 0);
}

#[test]
fn cli_convert_avif_to_png() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_avif(dir.path(), "input.avif");
  let output = dir.path().join("output.png");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success(), "AVIF to PNG conversion failed");
  assert!(output.exists());
  assert!(output.metadata().unwrap().len() > 0);
}

#[test]
fn cli_convert_bmp_to_png() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_bmp(dir.path(), "input.bmp");
  let output = dir.path().join("output.png");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
  assert!(output.metadata().unwrap().len() > 0);
}

#[test]
fn cli_convert_tiff_to_png() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_tiff(dir.path(), "input.tiff");
  let output = dir.path().join("output.png");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
  assert!(output.metadata().unwrap().len() > 0);
}

#[test]
fn cli_file_not_found_error() {
  let dir = tempfile::tempdir().unwrap();
  let input = dir.path().join("nonexistent.png");
  let output = dir.path().join("output.png");

  let output_result = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .output()
    .unwrap();

  assert!(!output_result.status.success());
  let stderr = String::from_utf8_lossy(&output_result.stderr);
  assert!(stderr.contains("File not found") || stderr.contains("Error"));
}

#[test]
fn cli_svg_output_rejected() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  // Output with .svg is still rejected (SVG output not supported)
  let output = dir.path().join("output.svg");

  let output_result = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .output()
    .unwrap();

  assert!(!output_result.status.success());
  let stderr = String::from_utf8_lossy(&output_result.stderr);
  assert!(stderr.contains("SVG") && stderr.contains("not supported"));
}

#[test]
fn cli_help_output() {
  let output = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("--help")
    .output()
    .unwrap();

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("Usage") && stdout.contains("Commands"));
}

#[test]
fn cli_version_output() {
  let output = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("--version")
    .output()
    .unwrap();

  assert!(output.status.success());
}

#[test]
fn cli_quiet_mode() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.jpg");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("-Q")
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
}

#[test]
fn cli_auto_output_name() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "photo.png");

  let output = dir.path().join("photo.jpg");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-f")
    .arg("jpg")
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
}

#[test]
fn cli_overwrite_existing_file() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.png");

  // Create a pre-existing output file
  std::fs::write(&output, b"dummy").unwrap();
  assert!(output.exists());

  // Without -O, should fail
  let result_without_ow = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .output()
    .unwrap();
  assert!(!result_without_ow.status.success());

  // With -O, should succeed
  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("-O")
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
  // Content should be the converted image, not the dummy bytes
  assert!(output.metadata().unwrap().len() > 10);
}

#[test]
fn cli_overwrite_long_flag() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.png");

  // Create a pre-existing output file
  std::fs::write(&output, b"dummy").unwrap();

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("--overwrite")
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
  assert!(output.metadata().unwrap().len() > 10);
}

#[test]
fn cli_same_file_rejected() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "same.png");
  // Same input and output path
  let output = input.clone();

  let output_result = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("-O")
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .output()
    .unwrap();

  assert!(!output_result.status.success());
  let stderr = String::from_utf8_lossy(&output_result.stderr);
  assert!(
    stderr.contains("Input and output paths are the same")
      || stderr.contains("Error"),
    "Expected same-file rejection, got: {}",
    stderr
  );
}

#[test]
fn cli_help_mentions_overwrite() {
  let output = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("--help")
    .output()
    .unwrap();

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("--overwrite") || stdout.contains("-O"));
  assert!(stdout.contains("Overwrite"));
}

#[test]
fn cli_convert_with_width_resize() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.png");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .arg("-w")
    .arg("32")
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());

  // Verify the output image was actually resized to width 32
  let img = image::open(&output).unwrap();
  assert_eq!(img.width(), 32);
}

#[test]
fn cli_convert_with_height_resize() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.png");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .arg("-h")
    .arg("32")
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());

  let img = image::open(&output).unwrap();
  assert_eq!(img.height(), 32);
}

#[test]
fn cli_convert_resize_with_both_dimensions() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.png");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .arg("-w")
    .arg("64")
    .arg("-h")
    .arg("64")
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());

  let img = image::open(&output).unwrap();
  assert_eq!(img.width(), 64);
  assert_eq!(img.height(), 64);
}

#[test]
fn cli_convert_resize_zero_width_rejected() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.png");

  let output_result = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .arg("-w")
    .arg("0")
    .output()
    .unwrap();

  assert!(!output_result.status.success());
  let stderr = String::from_utf8_lossy(&output_result.stderr);
  assert!(
    stderr.contains("greater than 0") || stderr.contains("Error"),
    "Expected zero-dimension rejection, got: {}",
    stderr
  );
}

#[test]
fn cli_convert_resize_zero_height_rejected() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.png");

  let output_result = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .arg("-h")
    .arg("0")
    .output()
    .unwrap();

  assert!(!output_result.status.success());
  let stderr = String::from_utf8_lossy(&output_result.stderr);
  assert!(
    stderr.contains("greater than 0") || stderr.contains("Error"),
    "Expected zero-dimension rejection, got: {}",
    stderr
  );
}

#[test]
fn cli_help_shows_width_height_options() {
  let output = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg("--help")
    .output()
    .unwrap();

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("--width") || stdout.contains("-w"));
  assert!(stdout.contains("--height") || stdout.contains("-h"));
}

#[test]
fn cli_convert_jpg_with_quality() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.jpg");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .arg("-q")
    .arg("80")
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
  // Verify it's a valid JPEG
  let img = image::open(&output).unwrap();
  assert_eq!(img.width(), 4);
  assert_eq!(img.height(), 4);
}

#[test]
fn cli_convert_jpg_quality_via_long_flag() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");
  let output = dir.path().join("output.jpg");

  let status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(output.to_str().unwrap())
    .arg("--quality")
    .arg("60")
    .status()
    .unwrap();

  assert!(status.success());
  assert!(output.exists());
}

#[test]
fn cli_convert_jpg_quality_affects_file_size() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");

  // Low quality
  let low_out = dir.path().join("low.jpg");
  let low_status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(low_out.to_str().unwrap())
    .arg("-q")
    .arg("10")
    .status()
    .unwrap();
  assert!(low_status.success());

  // High quality
  let high_out = dir.path().join("high.jpg");
  let high_status = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-o")
    .arg(high_out.to_str().unwrap())
    .arg("-q")
    .arg("95")
    .status()
    .unwrap();
  assert!(high_status.success());

  let low_size = low_out.metadata().unwrap().len();
  let high_size = high_out.metadata().unwrap().len();
  assert!(
    high_size > low_size,
    "high quality ({}) should produce larger file than low quality ({})",
    high_size,
    low_size,
  );
}

#[test]
fn cli_convert_quality_zero_rejected() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");

  let output_result = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-q")
    .arg("0")
    .output()
    .unwrap();

  assert!(!output_result.status.success());
  let stderr = String::from_utf8_lossy(&output_result.stderr);
  assert!(stderr.contains("error:"), "Expected error for quality=0, got: {}", stderr);
}

#[test]
fn cli_convert_quality_above_max_rejected() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "input.png");

  let output_result = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("convert")
    .arg(input.to_str().unwrap())
    .arg("-q")
    .arg("101")
    .output()
    .unwrap();

  assert!(!output_result.status.success());
  let stderr = String::from_utf8_lossy(&output_result.stderr);
  assert!(stderr.contains("error:"), "Expected error for quality=101, got: {}", stderr);
}

#[test]
fn cli_info_png() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_png(dir.path(), "info.png");

  let output_result = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("info")
    .arg(input.to_str().unwrap())
    .output()
    .unwrap();

  assert!(output_result.status.success());
  let stdout = String::from_utf8_lossy(&output_result.stdout);
  assert!(stdout.contains("File:"), "Expected File line, got: {}", stdout);
  assert!(stdout.contains("Format:"), "Expected Format line, got: {}", stdout);
  assert!(stdout.contains("Width:"), "Expected Width line, got: {}", stdout);
  assert!(stdout.contains("Height:"), "Expected Height line, got: {}", stdout);
  assert!(stdout.contains("4 px"), "Expected 4x4 dimensions, got: {}", stdout);
}

#[test]
fn cli_info_svg() {
  let dir = tempfile::tempdir().unwrap();
  let input = create_test_svg(dir.path(), "info.svg");

  let output_result = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("info")
    .arg(input.to_str().unwrap())
    .output()
    .unwrap();

  assert!(output_result.status.success());
  let stdout = String::from_utf8_lossy(&output_result.stdout);
  assert!(stdout.contains("SVG"), "Expected SVG format, got: {}", stdout);
  assert!(stdout.contains("Width:"), "Expected Width line, got: {}", stdout);
  assert!(stdout.contains("Height:"), "Expected Height line, got: {}", stdout);
}

#[test]
fn cli_info_file_not_found() {
  let dir = tempfile::tempdir().unwrap();
  let missing = dir.path().join("missing.png");

  let output_result = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("info")
    .arg(missing.to_str().unwrap())
    .output()
    .unwrap();

  assert!(!output_result.status.success());
  let stderr = String::from_utf8_lossy(&output_result.stderr);
  assert!(
    stderr.contains("File not found") || stderr.contains("Error"),
    "Expected error output, got: {}",
    stderr,
  );
}

#[test]
fn cli_info_unsupported_extension() {
  let dir = tempfile::tempdir().unwrap();
  let input = dir.path().join("file.xyz");
  std::fs::write(&input, b"whatever").unwrap();

  let output_result = Command::new(env!("CARGO_BIN_EXE_imgconv"))
    .arg("info")
    .arg(input.to_str().unwrap())
    .output()
    .unwrap();

  assert!(!output_result.status.success());
  let stderr = String::from_utf8_lossy(&output_result.stderr);
  assert!(
    stderr.contains("Unsupported image format") || stderr.contains("Error"),
    "Expected error output, got: {}",
    stderr,
  );
}
