use std::path::PathBuf;

use clap::Parser;

use crate::cli::args::{Cli, Commands};
use crate::cli::convert::ConvertArgs;
use crate::core::dispatcher;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();

  match &cli.command {
    Commands::Convert(args) => convert(&cli, args),
  }
}

fn convert(cli: &Cli, args: &ConvertArgs) -> Result<(), Box<dyn std::error::Error>> {
  let input_path = std::path::Path::new(&args.input);
  let output_path = determine_output_path(args)?;

  dispatcher::dispatch(
    input_path,
    &output_path,
  )?;

  Ok(())
}

fn determine_output_path(args: &ConvertArgs) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
  if let Some(output) = &args.output {
    Ok(PathBuf::from(output))
  } else {
    let input_path = std::path::Path::new(&args.input);
    let mut new_path = input_path.to_path_buf();
    new_path.set_extension("png");
    Ok(new_path)
  }
}
