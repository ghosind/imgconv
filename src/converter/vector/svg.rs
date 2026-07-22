use std::fs;
use std::path::Path;

use usvg::{Tree, Options};
use resvg::{render};
use tiny_skia::{Pixmap, Transform};

use crate::core::format::ImageFormat;
use crate::core::traits::{ImageConverter, ImageProcessor};
use crate::error::convert::ImageConvertError;
use crate::utils::encode::encode_image;

/// Converter for SVG (Scalable Vector Graphics) input images.
///
/// Renders the SVG to a raster bitmap, then encodes the result to the requested output format.
pub struct SVGConverter;

impl ImageConverter for SVGConverter {
  fn convert(
    &self,
    input_path: &Path,
    output_path: &Path,
    target_format: ImageFormat,
    processors: Vec<Box<dyn ImageProcessor>>,
  ) -> Result<(), ImageConvertError> {
    let svg_data = fs::read_to_string(input_path).map_err(|e| {
      ImageConvertError::SVGRenderError(format!("Failed to read SVG file: {}", e))
    })?;

    let opts = Options::default();
    let tree = Tree::from_str(&svg_data, &opts).map_err(|e| {
      ImageConvertError::SVGRenderError(format!("SVG parsing failed: {}", e))
    })?;

    let svg_size = tree.size();
    let width = if svg_size.width() > 0.0 {
      svg_size.width().ceil() as u32
    } else {
      800
    };
    let height = if svg_size.height() > 0.0 {
      svg_size.height().ceil() as u32
    } else {
      600
    };

    let mut pixmap = Pixmap::new(width, height).ok_or_else(|| {
      ImageConvertError::SVGRenderError("Failed to create render canvas".into())
    })?;
    render(&tree, Transform::default(), &mut pixmap.as_mut());

    let png_bytes = pixmap.encode_png().map_err(|e| {
      ImageConvertError::SVGRenderError(format!("Render result PNG encoding failed: {}", e))
    })?;

    let mut img = image::load_from_memory(&png_bytes).map_err(|e| {
      ImageConvertError::SVGRenderError(format!("Failed to load rendered PNG: {}", e))
    })?;

    encode_image(&mut img, target_format, output_path, processors)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::core::traits::ImageConverter;

  const MINIMAL_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
    <rect width="100" height="100" fill="red"/>
  </svg>"#;

  #[test]
  fn svg_converter_to_png() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.svg");
    let output = dir.path().join("output.png");

    std::fs::write(&input, MINIMAL_SVG).unwrap();

    let result = SVGConverter.convert(&input, &output, ImageFormat::PNG, vec![]);
    assert!(result.is_ok());
    assert!(output.exists());
    assert!(output.metadata().unwrap().len() > 0);
  }

  #[test]
  fn svg_converter_to_jpg() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.svg");
    let output = dir.path().join("output.jpg");

    std::fs::write(&input, MINIMAL_SVG).unwrap();

    let result = SVGConverter.convert(&input, &output, ImageFormat::JPG, vec![]);
    assert!(result.is_ok());
    assert!(output.exists());
    assert!(output.metadata().unwrap().len() > 0);
  }

  #[test]
  fn svg_converter_to_webp() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("input.svg");
    let output = dir.path().join("output.webp");

    std::fs::write(&input, MINIMAL_SVG).unwrap();

    let result = SVGConverter.convert(&input, &output, ImageFormat::WEBP, vec![]);
    assert!(result.is_ok());
    assert!(output.exists());
    assert!(output.metadata().unwrap().len() > 0);
  }

  #[test]
  fn svg_converter_invalid_svg() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("bad.svg");
    let output = dir.path().join("output.png");

    std::fs::write(&input, "this is not valid svg").unwrap();

    let result = SVGConverter.convert(&input, &output, ImageFormat::PNG, vec![]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
      err.to_string().contains("SVG render error")
        || err.to_string().contains("SVG parsing")
    );
  }

  #[test]
  fn svg_converter_file_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("nonexistent.svg");
    let output = dir.path().join("output.png");

    let result = SVGConverter.convert(&input, &output, ImageFormat::PNG, vec![]);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("SVG render error"));
  }
}
