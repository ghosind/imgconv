/// Entry point of the image conversion tool.
///
/// Delegates to the CLI runner, which handles all argument parsing,
/// output control, and error reporting.
fn main() {
  imgconv::cli::runner::run();
}
