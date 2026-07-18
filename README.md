# imgconv

A fast and simple CLI tool converting images between different formats.

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

### Quiet mode

Suppress non-essential output:

```bash
imgconv convert input.png -Q
```

## Commands & Options

### Global flags

| Flag | Description |
|------|-------------|
| `-Q`, `--quiet` | Enable quiet/silent output mode |

### `convert` subcommand

Convert an image file to another format.

| Argument / Flag | Description |
|-----------------|-------------|
| `<input>` | Path to the input image file **(required)** |
| `-o`, `--output` | Path to the output image file |
| `-f`, `--format` | Target output format (default: `png`) |

If neither `--output` nor `--format` is specified, the output defaults to a PNG file with the same base name as the input.

### Examples

```bash
# Explicit input and output paths
imgconv convert photo.jpg -o photo.webp

# Convert to JPEG with format flag
imgconv convert photo.png -f jpg

# Convert SVG to PNG
imgconv convert icon.svg -o icon.png

# Quiet conversion
imgconv convert input.bmp -f png -Q
```

## Supported Formats

### Input Formats

| Format | Extension | Status |
|--------|-----------|--------|
| PNG | `.png` | ✅ Supported |
| JPEG | `.jpg`, `.jpeg` | ✅ Supported |
| WebP | `.webp` | ✅ Supported |
| SVG | `.svg` | ✅ Supported (rasterization planned) |

### Output Formats

| Format | Extension | Status |
|--------|-----------|--------|
| PNG | `.png` | ✅ Supported (default) |
| JPEG | `.jpg` | ✅ Supported |
| WebP | `.webp` | ✅ Supported |

> **Note:** SVG output is not currently supported. SVG files can be used as input and will be rasterized to a raster output format (PNG, JPEG, or WebP). Full SVG rasterization support is planned for a future release.

## License

This project is licensed under the [MIT License](LICENSE).
