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
