use clap::{Args};

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
  #[arg(short = 'f', long, default_value = "png")]
  pub format: String,
}
