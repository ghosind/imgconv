/// Unified output controller that respects silent/quiet mode.
///
/// In quiet mode (via `-Q`/`--quiet`), only error messages are printed.
/// All informational, success, and warning messages are suppressed.
pub struct Output {
  quiet: bool,
}

impl Output {
  /// Creates a new `Output` controller.
  ///
  /// Pass `true` to suppress non-error output (silent mode).
  pub fn new(quiet: bool) -> Self {
    Self { quiet }
  }

  /// Returns whether silent mode is active.
  pub fn is_quiet(&self) -> bool {
    self.quiet
  }

  /// Prints an informational / progress message to stdout.
  ///
  /// Suppressed in quiet mode.
  pub fn info(&self, msg: &str) {
    if !self.quiet {
      println!("{}", msg);
    }
  }

  /// Prints a success message (with a checkmark) to stdout.
  ///
  /// Suppressed in quiet mode.
  pub fn success(&self, msg: &str) {
    if !self.quiet {
      println!("✔ {}", msg);
    }
  }

  /// Prints a warning message (with a warning icon) to stderr.
  ///
  /// Suppressed in quiet mode.
  pub fn warn(&self, msg: &str) {
    if !self.quiet {
      eprintln!("⚠  {}", msg);
    }
  }

  /// Prints an error message (with a cross mark) to stderr.
  ///
  /// Errors are **always** printed, even in quiet mode.
  pub fn error(&self, msg: &str) {
    eprintln!("✗ Error: {}", msg);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn quiet_mode_suppresses_info() {
    let out = Output::new(true);
    assert!(out.is_quiet());
    // No output expected — just verify no panic
    out.info("should not appear");
    out.success("should not appear");
    out.warn("should not appear");
  }

  #[test]
  fn normal_mode_allows_all() {
    let out = Output::new(false);
    assert!(!out.is_quiet());
    // Output expected — verify no panic
    out.info("info message");
    out.success("success message");
    out.warn("warning message");
    out.error("error message");
  }

  #[test]
  fn quiet_mode_still_shows_errors() {
    let out = Output::new(true);
    // Error should still print even in quiet mode
    out.error("critical error");
  }

  #[test]
  fn output_new_quiet_true() {
    let out = Output::new(true);
    assert!(out.is_quiet());
  }

  #[test]
  fn output_new_quiet_false() {
    let out = Output::new(false);
    assert!(!out.is_quiet());
  }
}
