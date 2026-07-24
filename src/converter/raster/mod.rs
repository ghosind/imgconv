/// AVIF image converter.
pub mod avif;
/// BMP image converter.
pub mod bmp;
/// ICO image converter.
pub mod ico;
/// JPEG image converter.
pub mod jpg;
/// PNG image converter.
pub mod png;
/// TIFF image converter.
pub mod tiff;
/// WEBP image converter.
pub mod webp;
/// Shared utility for raster image conversion (via the `image` crate).
pub mod util;

pub use avif::AVIFConverter;
pub use bmp::BMPConverter;
pub use ico::ICOConverter;
pub use jpg::JPGConverter;
pub use png::PNGConverter;
pub use tiff::TIFFConverter;
pub use webp::WEBPConverter;
