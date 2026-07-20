use clap::Parser;

use crate::cli::args::{Cli, Commands};
use crate::cli::convert::convert;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Convert(args) => convert(&cli, args),
  }
}
