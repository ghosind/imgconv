use std::process::Command;

/// Helper: create a minimal PNG in a temp directory
fn create_test_png(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
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
