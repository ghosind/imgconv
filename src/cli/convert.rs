use std::path::PathBuf;

use clap::{Args};

use crate::cli::args::{Cli};
use crate::core::format::ImageFormat;
use crate::core::dispatcher;

/// convert subcommand arguments
#[derive(Args, Debug)]
pub struct ConvertArgs {
  /// Input file
  pub input: String,

  /// Output file (-o/--output)
  #[arg(short = 'o', long)]
  pub output: Option<String>,

  /// Target output format, default png
  /// (-f/--format)
  #[arg(short = 'f', long)]
  pub format: Option<String>,
}

pub fn convert(_: &Cli, args: &ConvertArgs) -> Result<(), Box<dyn std::error::Error>> {
  let input_path = std::path::Path::new(&args.input);
  let output_path = determine_output_path(args)?;
  let output_format = if args.format.is_some() {
    ImageFormat::from_str(&args.format.as_ref().unwrap())?
  } else {
    ImageFormat::from_extension(&output_path)?
  };

  dispatcher::dispatch(
    input_path,
    &output_path,
    output_format,
  )?;

  Ok(())
}

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
    };

    let cli = Cli {
      quiet: false,
      command: Commands::Convert(args),
    };

    match &cli.command {
      Commands::Convert(args) => {
        let out_path = determine_output_path(args).unwrap();
        let result = crate::core::dispatcher::dispatch(
          std::path::Path::new(&args.input),
          &out_path,
          ImageFormat::from_str(&args.format.as_ref().unwrap()).unwrap()
        );
        assert!(result.is_ok());
        assert!(out_path.exists());
      }
    }
  }
}
