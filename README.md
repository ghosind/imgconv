# imgconv

A fast and simple CLI tool for converting images between different formats.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/) (edition 2024 or later)

### Build from source

```bash
git clone https://github.com/ghosind/imgconv.git
cd imgconv
cargo build --release
```

The binary will be located at `target/release/imgconv`. You can copy it to a directory in your `$PATH`, for example:

```bash
cp target/release/imgconv /usr/local/bin/
```

## Usage

### Basic conversion

Convert an image from one format to another:

```bash
# Convert input.png to output.jpg
imgconv convert input.png -o output.jpg

# Convert input.jpg to PNG (default format)
imgconv convert input.jpg

# Convert input.webp to output.png
imgconv convert input.webp -o output.png
```

### Specify output format

Use the `-f` / `--format` flag to specify the target format when you don't provide an output file:

```bash
# Convert input.png to JPEG, saved as input.jpg
imgconv convert input.png -f jpg

# Convert input.jpg to WebP, saved as input.webp
imgconv convert input.jpg -f webp
```

### Resize images

Use `-w` / `--width` and `-h` / `--height` to resize the image during conversion. The Lanczos3 filter is used for high-quality resampling.

When only one dimension is specified, the other is automatically calculated to preserve the original aspect ratio:

```bash
# Resize to exactly 800×600 (may distort)
imgconv convert input.png -o output.jpg -w 800 -h 600

# Scale to width 800, height auto-calculated
imgconv convert input.png -o output.jpg -w 800

# Scale to height 600, width auto-calculated
imgconv convert input.jpg -o output.png -h 600
```

### SVG rasterization

SVG files can be used as input and will be rasterized to any raster output format:

```bash
# Rasterize SVG to PNG
imgconv convert icon.svg -o icon.png

# Rasterize SVG to JPEG
imgconv convert logo.svg -f jpg

# Rasterize and resize SVG in one step
imgconv convert diagram.svg -o diagram.webp -w 1200
```

### Overwrite protection

By default, `imgconv` refuses to overwrite an existing output file. Use `-O` / `--overwrite` to force overwriting:

```bash
# This will fail if output.png already exists
imgconv convert input.jpg -o output.png

# Force overwrite
imgconv convert input.jpg -o output.png -O
```

### Quiet mode

Suppress non-error output with `-Q` / `--quiet`:

```bash
imgconv convert input.png -o output.jpg -Q
```

## Commands & Options

### Global flags

| Flag | Description |
|------|-------------|
| `-Q`, `--quiet` | Enable quiet/silent output mode (only errors are printed) |
| `-O`, `--overwrite` | Overwrite existing output files without prompting |

### `convert` subcommand

Convert an image file to another format.

| Argument / Flag | Description |
|-----------------|-------------|
| `<input>` | Path to the input image file **(required)** |
| `-o`, `--output` | Path to the output image file |
| `-f`, `--format` | Target output format (default: `png`). Supported: `avif`, `bmp`, `jpg`/`jpeg`, `png`, `tiff`, `webp` |
| `-w`, `--width` | Target width in pixels. When only width is given, height is auto-calculated to preserve aspect ratio |
| `-h`, `--height` | Target height in pixels. When only height is given, width is auto-calculated to preserve aspect ratio |

If neither `--output` nor `--format` is specified, the output defaults to a PNG file with the same base name as the input.

## Supported Formats

### Input Formats

| Format | Extension | Status |
|--------|-----------|--------|
| AVIF | `.avif` | ✅ Supported |
| BMP | `.bmp` | ✅ Supported |
| JPEG | `.jpg`, `.jpeg` | ✅ Supported |
| PNG | `.png` | ✅ Supported |
| TIFF | `.tif`, `.tiff` | ✅ Supported |
| WebP | `.webp` | ✅ Supported |
| SVG | `.svg` | ✅ Supported (rasterized to output format) |

### Output Formats

| Format | Extension | Status |
|--------|-----------|--------|
| AVIF | `.avif` | ✅ Supported |
| BMP | `.bmp` | ✅ Supported |
| JPEG | `.jpg` | ✅ Supported |
| PNG | `.png` | ✅ Supported (default) |
| TIFF | `.tiff` | ✅ Supported |
| WebP | `.webp` | ✅ Supported |

> **Note:** SVG output is not supported. SVG files can only be used as input and will be rasterized to a raster output format.

## Examples

```bash
# Basic format conversion
imgconv convert photo.jpg -o photo.webp

# Convert to JPEG with format flag
imgconv convert photo.png -f jpg

# SVG rasterization
imgconv convert icon.svg -o icon.png

# Resize while converting
imgconv convert large.png -o small.jpg -w 400

# Convert and resize to exact dimensions
imgconv convert input.bmp -o output.png -w 1920 -h 1080

# Force overwrite existing file
imgconv convert input.png -o existing.jpg -O

# Quiet conversion with resize
imgconv convert input.jpg -f webp -w 800 -Q
```

## License

This project is licensed under the [MIT License](LICENSE).
