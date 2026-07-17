fn main() {
  if let Err(e) = imgconv::cli::runner::run() {
    eprintln!("Error: {}", e);
    std::process::exit(1);
  }
}
