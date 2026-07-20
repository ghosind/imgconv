use clap::{Parser, Subcommand};

use crate::cli::convert::ConvertArgs;

#[derive(Parser, Debug)]
#[command(name = "imgconv", version)]
pub struct Cli {
  /// Quiet/silent output mode (-Q/--quiet)
  #[arg(short = 'Q', long, global = true)]
  pub quiet: bool,

  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Image format conversion
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
}
