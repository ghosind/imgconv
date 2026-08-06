use std::path::Path;

use clap::Args;
use image::ImageDecoder;

use crate::cli::args::Cli;
use crate::core::format::ImageFormat;
use crate::error::convert::ImageConvertError;
use crate::utils::format::{format_file_size, format_image_format_name};
use crate::utils::output::Output;

/// Arguments for the `info` subcommand.
///
/// Specifies the input file whose metadata should be inspected.
#[derive(Args, Debug)]
#[command(disable_help_flag = true)]
pub struct InfoArgs {
  /// Show help for the info subcommand.
  #[arg(long, action = clap::ArgAction::Help)]
  pub help: Option<bool>,

  /// Path to the image file to inspect.
  pub input: String,
}

/// Executes the image info workflow for the `info` subcommand.
///
/// Reads the image header/metadata (without fully decoding pixel data where
/// possible) and prints a summary of the file: size, format, dimensions,
/// color type, and alpha presence.
pub fn info(cli: &Cli, args: &InfoArgs) -> Result<(), Box<dyn std::error::Error>> {
  let out = Output::new(cli.quiet);
  let input_path = Path::new(&args.input);

  if !input_path.exists() {
    return Err(ImageConvertError::FileNotFound(args.input.clone()).into());
  }

  let format = ImageFormat::from_extension(input_path)?;
  let file_size = std::fs::metadata(input_path)?.len();

  out.info(&format!("File:   {}", args.input));
  out.info(&format!(
    "Size:   {} ({} bytes)",
    format_file_size(file_size),
    file_size,
  ));

  match format {
    ImageFormat::SVG => svg_info(&out, input_path)?,
    _ => raster_info(&out, input_path, format)?,
  }

  Ok(())
}

/// Prints metadata for raster image formats.
fn raster_info(
  out: &Output,
  input_path: &Path,
  format: ImageFormat,
) -> Result<(), ImageConvertError> {
  let mut reader = image::ImageReader::open(input_path)?;
  reader = reader.with_guessed_format()?;
  let detected = reader.format();
  let decoder = reader.into_decoder()?;

  let (width, height) = decoder.dimensions();
  let color_type = decoder.color_type();
  let bits_per_channel = color_type.bits_per_pixel() / color_type.channel_count() as u16;

  out.info(&format!(
    "Format: {}",
    detected
      .map(format_image_format_name)
      .unwrap_or_else(|| format.extension()),
  ));
  out.info(&format!("Width:  {} px", width));
  out.info(&format!("Height: {} px", height));
  out.info(&format!(
    "Color:  {:?} ({} bpc, {} channel{})",
    color_type,
    bits_per_channel,
    color_type.channel_count(),
    if color_type.channel_count() == 1 { "" } else { "s" },
  ));
  out.info(&format!(
    "Alpha:  {}",
    if color_type.has_alpha() { "yes" } else { "no" },
  ));

  // Warn when the actual file content does not match the file extension.
  if let (Some(expected), Some(detected)) = (format.image_format(), detected) {
    if expected != detected {
      out.warn(&format!(
        "Content is {} but the file extension is .{}",
        format_image_format_name(detected),
        format.extension(),
      ));
    }
  }

  Ok(())
}

/// Prints metadata for SVG (vector) images.
fn svg_info(out: &Output, input_path: &Path) -> Result<(), ImageConvertError> {
  let svg_data = std::fs::read_to_string(input_path).map_err(|e| {
    ImageConvertError::SVGRenderError(format!("Failed to read SVG file: {}", e))
  })?;

  let tree = usvg::Tree::from_str(&svg_data, &usvg::Options::default()).map_err(|e| {
    ImageConvertError::SVGRenderError(format!("SVG parsing failed: {}", e))
  })?;

  let size = tree.size();
  let width = size.width().ceil() as u32;
  let height = size.height().ceil() as u32;

  out.info("Format: SVG");
  out.info(&format!("Width:  {} px", width));
  out.info(&format!("Height: {} px", height));
  out.info("Color:  n/a (vector)");
  out.info("Alpha:  n/a (vector)");

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use clap::Parser;

  use crate::cli::args::Commands;

  #[test]
  fn info_raster_png() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("test.png");
    image::DynamicImage::new_rgba8(3, 5).save(&input).unwrap();

    let cli = Cli::try_parse_from(["imgconv", "info", input.to_str().unwrap()]).unwrap();
    match &cli.command {
      Commands::Info(args) => {
        let result = info(&cli, args);
        assert!(result.is_ok());
      }
      _ => panic!("expected Info command"),
    }
  }

  #[test]
  fn info_svg() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("test.svg");
    std::fs::write(
      &input,
      r#"<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64">
        <rect width="64" height="64" fill="blue"/>
      </svg>"#,
    )
    .unwrap();

    let cli = Cli::try_parse_from(["imgconv", "info", input.to_str().unwrap()]).unwrap();
    match &cli.command {
      Commands::Info(args) => {
        let result = info(&cli, args);
        assert!(result.is_ok());
      }
      _ => panic!("expected Info command"),
    }
  }

  #[test]
  fn info_file_not_found() {
    let cli = Cli::try_parse_from(["imgconv", "info", "/nonexistent/img.png"]).unwrap();
    match &cli.command {
      Commands::Info(args) => {
        let result = info(&cli, args);
        assert!(result.is_err());
      }
      _ => panic!("expected Info command"),
    }
  }

  #[test]
  fn info_unsupported_extension() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("file.xyz");
    std::fs::write(&input, b"whatever").unwrap();

    let cli = Cli::try_parse_from(["imgconv", "info", input.to_str().unwrap()]).unwrap();
    match &cli.command {
      Commands::Info(args) => {
        let result = info(&cli, args);
        assert!(result.is_err());
      }
      _ => panic!("expected Info command"),
    }
  }
}
