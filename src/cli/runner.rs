use clap::Parser;

use crate::cli::args::{Cli, Commands};
use crate::cli::convert::convert;
use crate::utils::output::Output;

/// Parses CLI arguments and executes the appropriate subcommand.
///
/// This is the top-level entry point called from `main()`. It handles
/// argument parsing, output control (respecting `--quiet`), and unified
/// error reporting.
pub fn run() {
  let cli = Cli::parse();
  let out = Output::new(cli.quiet);

  let result = match &cli.command {
    Commands::Convert(args) => convert(&cli, args),
  };

  if let Err(e) = result {
    out.error(&e.to_string());
    std::process::exit(1);
  }
}
