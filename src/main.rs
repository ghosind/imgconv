/// Entry point of the image conversion tool.
///
/// Parses command-line arguments and dispatches to the appropriate subcommand.
/// Exits with code 1 if any error occurs during execution.
fn main() {
  if let Err(e) = imgconv::cli::runner::run() {
    eprintln!("Error: {}", e);
    std::process::exit(1);
  }
}
