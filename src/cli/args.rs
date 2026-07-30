use clap::{ArgAction, Parser, Subcommand};

use crate::cli::convert::ConvertArgs;

/// CLI argument structure for the application.
///
/// Provides global flags (e.g., `--quiet`) and dispatches to subcommands.
#[derive(Parser, Debug)]
#[command(name = "imgconv", version, disable_help_flag = true)]
pub struct Cli {
  /// Show help information for the application.
  #[arg(long, action = ArgAction::Help)]
  pub help: Option<bool>,

  /// Quiet/silent output mode (-Q/--quiet).
  #[arg(short = 'Q', long, global = true)]
  pub quiet: bool,

  /// Overwrite existing files without prompting (-O/--overwrite).
  #[arg(short = 'O', long, global = true)]
  pub overwrite: bool,

  /// The subcommand to execute.
  #[command(subcommand)]
  pub command: Commands,
}

/// Available subcommands for the application.
#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Convert an image from one format to another with the specified options.
  Convert(ConvertArgs),
}

#[cfg(test)]
mod tests {
  use super::*;
  use clap::Parser;

  #[test]
  fn parse_basic_convert() {
    let cli = Cli::try_parse_from([
      "imgconv", "convert", "input.png",
    ]).unwrap();
    match &cli.command {
      Commands::Convert(args) => {
        assert_eq!(args.input, "input.png");
        assert!(args.format.is_none());
        assert!(args.output.is_none());
      }
    }
  }

  #[test]
  fn parse_convert_with_output() {
    let cli = Cli::try_parse_from([
      "imgconv", "convert", "input.jpg", "-o", "output.png",
    ]).unwrap();
    match &cli.command {
      Commands::Convert(args) => {
        assert_eq!(args.input, "input.jpg");
        assert_eq!(args.output.as_deref(), Some("output.png"));
      }
    }
  }

  #[test]
  fn parse_convert_with_format() {
    let cli = Cli::try_parse_from([
      "imgconv", "convert", "input.png", "-f", "jpg",
    ]).unwrap();
    match &cli.command {
      Commands::Convert(args) => {
        assert_eq!(args.format.as_deref(), Some("jpg"));
      }
    }
  }

  #[test]
  fn parse_convert_with_long_flags() {
    let cli = Cli::try_parse_from([
      "imgconv", "convert", "input.webp",
      "--output", "result.jpg",
      "--format", "jpg",
    ]).unwrap();
    match &cli.command {
      Commands::Convert(args) => {
        assert_eq!(args.input, "input.webp");
        assert_eq!(args.output.as_deref(), Some("result.jpg"));
        assert_eq!(args.format.as_deref(), Some("jpg"));
      }
    }
  }

  #[test]
  fn parse_quiet_flag() {
    let cli = Cli::try_parse_from([
      "imgconv", "-Q", "convert", "input.png",
    ]).unwrap();
    assert!(cli.quiet);
  }

  #[test]
  fn parse_quiet_long_flag() {
    let cli = Cli::try_parse_from([
      "imgconv", "--quiet", "convert", "input.png",
    ]).unwrap();
    assert!(cli.quiet);
  }

  #[test]
  fn parse_missing_subcommand_is_error() {
    let result = Cli::try_parse_from(["imgconv"]);
    assert!(result.is_err());
  }

  #[test]
  fn cli_debug_format() {
    let cli = Cli::try_parse_from(["imgconv", "convert", "input.png"]).unwrap();
    let debug_str = format!("{:?}", cli);
    assert!(debug_str.contains("Cli"));
    assert!(debug_str.contains("Convert"));
  }

  #[test]
  fn parse_overwrite_short_flag() {
    let cli = Cli::try_parse_from([
      "imgconv", "-O", "convert", "input.png",
    ]).unwrap();
    assert!(cli.overwrite);
  }

  #[test]
  fn parse_overwrite_long_flag() {
    let cli = Cli::try_parse_from([
      "imgconv", "--overwrite", "convert", "input.png",
    ]).unwrap();
    assert!(cli.overwrite);
  }

  #[test]
  fn parse_overwrite_defaults_to_false() {
    let cli = Cli::try_parse_from([
      "imgconv", "convert", "input.png",
    ]).unwrap();
    assert!(!cli.overwrite);
  }

  #[test]
  fn parse_combined_flags() {
    let cli = Cli::try_parse_from([
      "imgconv", "-Q", "-O", "convert", "input.png", "-f", "jpg",
    ]).unwrap();
    assert!(cli.quiet);
    assert!(cli.overwrite);
    match &cli.command {
      Commands::Convert(args) => {
        assert_eq!(args.input, "input.png");
        assert_eq!(args.format.as_deref(), Some("jpg"));
      }
    }
  }

  #[test]
  fn parse_quality_short_flag() {
    let cli = Cli::try_parse_from([
      "imgconv", "convert", "input.png", "-q", "85",
    ]).unwrap();
    match &cli.command {
      Commands::Convert(args) => {
        assert_eq!(args.quality, Some(85));
      }
    }
  }

  #[test]
  fn parse_quality_long_flag() {
    let cli = Cli::try_parse_from([
      "imgconv", "convert", "input.png", "--quality", "50",
    ]).unwrap();
    match &cli.command {
      Commands::Convert(args) => {
        assert_eq!(args.quality, Some(50));
      }
    }
  }

  #[test]
  fn parse_quality_defaults_to_none() {
    let cli = Cli::try_parse_from([
      "imgconv", "convert", "input.png",
    ]).unwrap();
    match &cli.command {
      Commands::Convert(args) => {
        assert_eq!(args.quality, None);
      }
    }
  }

  #[test]
  fn parse_quality_min_boundary() {
    let cli = Cli::try_parse_from([
      "imgconv", "convert", "input.png", "-q", "1",
    ]).unwrap();
    match &cli.command {
      Commands::Convert(args) => {
        assert_eq!(args.quality, Some(1));
      }
    }
  }

  #[test]
  fn parse_quality_max_boundary() {
    let cli = Cli::try_parse_from([
      "imgconv", "convert", "input.png", "-q", "100",
    ]).unwrap();
    match &cli.command {
      Commands::Convert(args) => {
        assert_eq!(args.quality, Some(100));
      }
    }
  }

  #[test]
  fn parse_quality_zero_rejected() {
    let result = Cli::try_parse_from([
      "imgconv", "convert", "input.png", "-q", "0",
    ]);
    assert!(result.is_err());
  }

  #[test]
  fn parse_quality_above_max_rejected() {
    let result = Cli::try_parse_from([
      "imgconv", "convert", "input.png", "-q", "101",
    ]);
    assert!(result.is_err());
  }

  #[test]
  fn parse_quality_non_numeric_rejected() {
    let result = Cli::try_parse_from([
      "imgconv", "convert", "input.png", "-q", "abc",
    ]);
    assert!(result.is_err());
  }

  #[test]
  fn parse_quality_alongside_other_flags() {
    let cli = Cli::try_parse_from([
      "imgconv", "convert", "input.png", "-f", "jpg", "-q", "75", "-w", "100",
    ]).unwrap();
    match &cli.command {
      Commands::Convert(args) => {
        assert_eq!(args.format.as_deref(), Some("jpg"));
        assert_eq!(args.quality, Some(75));
        assert_eq!(args.width, Some(100));
      }
    }
  }
}
