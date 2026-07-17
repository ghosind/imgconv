use std::path::Path;

use image::DynamicImage;

use crate::core::format::ImageFormat;
use crate::error::convert::ImageConvertError;

pub fn encode_image(
  img: &DynamicImage,
  format: ImageFormat,
  output_path: &Path,
) -> Result<(), ImageConvertError> {
  let file = std::fs::File::create(output_path)?;
  let mut writer = std::io::BufWriter::new(file);

  match format {
    ImageFormat::PNG => {
      img.write_to(&mut writer, image::ImageFormat::Png)?;
    }
    ImageFormat::JPG => {
      let mut encoder = image::codecs::jpeg::JpegEncoder::new(&mut writer);
      encoder.encode_image(img)?;
    }
    ImageFormat::WEBP => {
      img.write_to(&mut writer, image::ImageFormat::WebP)?;
    }
    ImageFormat::SVG => {
      return Err(ImageConvertError::UnsupportedFormat(
        "SVG output is not supported.".into(),
      ));
    }
  }

  Ok(())
}
