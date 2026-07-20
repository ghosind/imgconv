/// Converter registry — maps [`ImageFormat`](crate::core::format::ImageFormat)
/// to the appropriate [`ImageConverter`](crate::core::traits::ImageConverter) implementation.
pub mod convert;
/// High-level dispatch logic that validates inputs and orchestrates conversion.
pub mod dispatcher;
/// Supported image format enum and associated utilities.
pub mod format;
/// The [`ImageConverter`](crate::core::traits::ImageConverter) trait defining the conversion interface.
pub mod traits;
