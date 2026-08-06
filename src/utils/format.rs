use image::ImageFormat;

/// Formats a byte count into a human-readable string (e.g., `"1.25 MiB"`).
pub fn format_file_size(bytes: u64) -> String {
  const KB: f64 = 1024.0;
  const MB: f64 = KB * 1024.0;
  const GB: f64 = MB * 1024.0;

  let size = bytes as f64;
  if size >= GB {
    return format!("{:.2} GiB", size / GB)
  } else if size >= MB {
    return format!("{:.2} MiB", size / MB)
  } else if size >= KB {
    return format!("{:.2} KiB", size / KB)
  } else {
    return format!("{} B", bytes)
  }
}

/// Returns an uppercase, human-readable name for a detected image format.
pub fn format_image_format_name(fmt: ImageFormat) -> &'static str {
  match fmt {
    ImageFormat::Avif => "AVIF",
    ImageFormat::Bmp => "BMP",
    ImageFormat::Ico => "ICO",
    ImageFormat::Jpeg => "JPEG",
    ImageFormat::Png => "PNG",
    ImageFormat::Tiff => "TIFF",
    ImageFormat::WebP => "WEBP",
    _ => "UNKNOWN",
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn format_file_size_bytes() {
    assert_eq!(format_file_size(0), "0 B");
    assert_eq!(format_file_size(512), "512 B");
  }

  #[test]
  fn format_file_size_kib() {
    assert_eq!(format_file_size(1024), "1.00 KiB");
    assert_eq!(format_file_size(1536), "1.50 KiB");
  }

  #[test]
  fn format_file_size_mib() {
    assert_eq!(format_file_size(1024 * 1024), "1.00 MiB");
    assert_eq!(format_file_size(1024 * 1024 * 1024), "1.00 GiB");
  }
}
