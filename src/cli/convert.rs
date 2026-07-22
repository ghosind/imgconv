use std::path::PathBuf;

use clap::{Args};

use crate::cli::args::{Cli};
use crate::core::format::ImageFormat;
use crate::core::dispatcher;
use crate::core::traits::ImageProcessor;
use crate::processor::resize::ResizeProcessor;
use crate::utils::output::Output;

/// Arguments for the `convert` subcommand.
///
/// Specifies the input file, an optional output path, and the other conversion parameters.
#[derive(Args, Debug)]
#[command(disable_help_flag = true)]
pub struct ConvertArgs {
  /// Show help for the convert subcommand.
  #[arg(long, action = clap::ArgAction::Help)]
  pub help: Option<bool>,
  /// Path to the input image file.
  pub input: String,

  /// Path to the output image file (`-o` / `--output`).
  /// If omitted, the output path is derived from the input file name and target format.
  #[arg(short = 'o', long)]
  pub output: Option<String>,

  /// Target output format (`-f` / `--format`). Supported values: `png`, `jpg`, `jpeg`, `webp`.
  /// If omitted, the format is inferred from the output file extension. Defaults to `png`.
  #[arg(short = 'f', long)]
  pub format: Option<String>,

  /// Target width in pixels (`-w` / `--width`).
  /// When only one dimension is specified, the other is auto-calculated to preserve aspect ratio.
  #[arg(short = 'w', long)]
  pub width: Option<u32>,

  /// Target height in pixels (`-h` / `--height`).
  /// When only one dimension is specified, the other is auto-calculated to preserve aspect ratio.
  #[arg(short = 'h', long)]
  pub height: Option<u32>,
}

/// Executes the image conversion workflow for the `convert` subcommand.
///
/// Determines the output path and target format, then delegates the actual
/// conversion to the core dispatcher.
pub fn convert(cli: &Cli, args: &ConvertArgs) -> Result<(), Box<dyn std::error::Error>> {
  let out = Output::new(cli.quiet);

  let processors = build_processors(args);

  let input_path = std::path::Path::new(&args.input);
  let output_path = determine_output_path(args)?;

  let input_format = ImageFormat::from_extension(input_path)?;
  let output_format = if args.format.is_some() {
    ImageFormat::from_str(&args.format.as_ref().unwrap())?
  } else {
    ImageFormat::from_extension(&output_path)?
  };

  out.info(&format!(
    "Converting {} → {}  [{} → {}]",
    args.input,
    output_path.display(),
    input_format.extension(),
    output_format.extension(),
  ));

  dispatcher::dispatch(
    input_path,
    &output_path,
    output_format,
    cli.overwrite,
    processors,
  )?;

  out.success(&format!("Converted: {} → {}", args.input, output_path.display()));

  Ok(())
}

/// Builds a list of image processors based on the provided conversion arguments.
fn build_processors(args: &ConvertArgs) -> Vec<Box<dyn ImageProcessor>> {
  let mut processors: Vec<Box<dyn ImageProcessor>> = Vec::new();

  if args.width.is_some() || args.height.is_some() {
    processors.push(Box::new(ResizeProcessor::new(
      args.width,
      args.height,
    )));
  }

  processors
}

/// Resolves the output file path from the conversion arguments.
///
/// Priority:
/// 1. Explicit `--output` path if provided.
/// 2. Input path with extension replaced by the specified `--format`.
/// 3. Input path with extension replaced by `png` (default).
fn determine_output_path(args: &ConvertArgs) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
  if let Some(output) = &args.output {
    Ok(PathBuf::from(output))
  } else if let Some(fmt) = &args.format {
    let input_path = std::path::Path::new(&args.input);
    let mut new_path = input_path.to_path_buf();
    new_path.set_extension(fmt);
    Ok(new_path)
  } else {
    let input_path = std::path::Path::new(&args.input);
    let mut new_path = input_path.to_path_buf();
    new_path.set_extension("png");
    Ok(new_path)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::cli::args::{Commands};

  #[test]
  fn determine_output_explicit() {
    let args = ConvertArgs {
      input: "input.jpg".into(),
      output: Some("custom.png".into()),
      format: Some("png".into()),
      width: None,
      height: None,
      help: None,
    };
    let result = determine_output_path(&args).unwrap();
    assert_eq!(result, PathBuf::from("custom.png"));
  }

  #[test]
  fn determine_output_auto_from_input() {
    let args = ConvertArgs {
      input: "photo.jpg".into(),
      output: None,
      format: Some("png".into()),
      width: None,
      height: None,
      help: None,
    };
    let result = determine_output_path(&args).unwrap();
    assert_eq!(result, PathBuf::from("photo.png"));
  }

  #[test]
  fn determine_output_auto_no_extension() {
    let args = ConvertArgs {
      input: "file_without_ext".into(),
      output: None,
      format: Some("png".into()),
      width: None,
      height: None,
      help: None,
    };
    let result = determine_output_path(&args).unwrap();
    assert_eq!(result, PathBuf::from("file_without_ext.png"));
  }

  #[test]
  fn determine_output_explicit_overrides_input() {
    let args = ConvertArgs {
      input: "input.svg".into(),
      output: Some("output.jpg".into()),
      format: Some("png".into()),
      width: None,
      height: None,
      help: None,
    };
    let result = determine_output_path(&args).unwrap();
    assert_eq!(result, PathBuf::from("output.jpg"));
  }

  #[test]
  fn determine_output_with_defaults() {
    let args = ConvertArgs {
      input: "image.jpg".into(),
      output: None,
      format: None,
      width: None,
      height: None,
      help: None,
    };
    let result = determine_output_path(&args).unwrap();
    assert_eq!(result, PathBuf::from("image.png"));
  }

  #[test]
  fn convert_function_via_run_path() {
    // Test the full convert path indirectly via the Cli struct
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("test.png");
    image::DynamicImage::new_rgba8(2, 2).save(&input).unwrap();
    let output = dir.path().join("result.jpg");

    let args = ConvertArgs {
      input: input.to_str().unwrap().to_string(),
      output: Some(output.to_str().unwrap().to_string()),
      format: Some("jpg".to_string()),
      width: None,
      height: None,
      help: None,
    };

    let cli = Cli {
      help: None,
      quiet: false,
      overwrite: false,
      command: Commands::Convert(args),
    };

    match &cli.command {
      Commands::Convert(args) => {
        let out_path = determine_output_path(args).unwrap();
        let result = crate::core::dispatcher::dispatch(
          std::path::Path::new(&args.input),
          &out_path,
          ImageFormat::from_str(&args.format.as_ref().unwrap()).unwrap(),
          false,
          vec![],
        );
        assert!(result.is_ok());
        assert!(out_path.exists());
      }
    }
  }
}
