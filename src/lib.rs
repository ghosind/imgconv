/// Command-line interface (CLI) argument parsing and subcommand dispatch.
pub mod cli;
/// Image format converters for both raster and vector formats.
pub mod converter;
/// Core abstractions: format definitions, traits, conversion dispatch, and converter registry.
pub mod core;
/// Custom error types for image conversion operations.
pub mod error;
/// Utility functions: image encoding helpers and unified output controller.
pub mod utils;
