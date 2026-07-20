use clap::Parser;

use crate::cli::args::{Cli, Commands};
use crate::cli::convert::convert;

/// Parses CLI arguments and executes the appropriate subcommand.
///
/// This is the top-level entry point called from `main()`. It uses `clap`
/// to parse command-line arguments and dispatches to the matching handler.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Convert(args) => convert(&cli, args),
  }
}
