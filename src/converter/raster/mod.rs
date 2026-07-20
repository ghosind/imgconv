/// JPEG image converter.
pub mod jpg;
/// PNG image converter.
pub mod png;
/// WEBP image converter.
pub mod webp;
/// Shared utility for raster image conversion (via the `image` crate).
pub mod util;

pub use jpg::JPGConverter;
pub use png::PNGConverter;
pub use webp::WEBPConverter;
