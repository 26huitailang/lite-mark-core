# LiteMark

A lightweight photo parameter frame tool for photography enthusiasts.

## Features

- 📸 **Extract EXIF data** - Real-time extraction of ISO, aperture, shutter speed, focal length, etc.
  - ✅ Supports JPEG, PNG, TIFF formats
  - ✅ Auto-detects camera model and lens information
  - ✅ Compatible with images without EXIF data
  - ✅ Smart formatting (e.g., "1/125", "f/2.8", "50mm")
- 🖼️ **Frame mode** - Add bottom frame with shooting parameters and logo
- 🎨 **Template system** - Customizable layouts with JSON-based configuration
- 🔤 **Professional font rendering** - Using rusttype for high-quality text
  - ⚠️ Note: Default font (DejaVu Sans) supports English only
  - 💡 For Chinese text, use `--font` parameter or set `LITEMARK_FONT` environment variable
- 🖼️ **Logo support** - Automatic scaling and positioning
  - ✅ Customizable via `--logo` parameter or `LITEMARK_LOGO` environment variable
  - ✅ Priority: CLI > ENV > Template
- 📱 **Batch processing** - Process entire directories
- 🔒 **Privacy-first** - All processing happens locally, no cloud upload
- 🎯 **Simple CLI interface** - Easy to use command-line tool

## Installation

### From Source

```bash
git clone https://github.com/26huitailang/lite-mark-core.git
cd lite-mark-core
cargo build --release
```

### Binary Downloads

Download pre-built binaries from [GitHub Releases](https://github.com/26huitailang/lite-mark-core/releases).

## Usage

### Basic Usage

```bash
# Add frame to a single image
litemark add -i input.jpg -t classic -o output.jpg

# Add frame with custom author name
litemark add -i input.jpg -t classic -o output.jpg --author "Photographer Name"

# Batch process a directory
litemark batch -i /path/to/photos/ -t classic -o /path/to/output/

# List available templates
litemark templates

# Show template details
litemark show-template classic
```

### Frame Mode

LiteMark adds a bottom frame to your photos with:
- **Logo** (centered at top of frame) - camera brand, custom logo, etc.
- **Photographer name** - customizable author name
- **Shooting parameters** - ISO, aperture, shutter speed, focal length, etc.

### Template System

LiteMark uses JSON-based templates for flexible frame layouts:

```json
{
  "name": "ClassicParam",
  "anchor": "bottom-left",
  "padding": 0,
  "items": [
    {"type": "logo", "value": "path/to/logo.png"},
    {"type": "text", "value": "{Author}", "font_size": 20, "color": "#000000"},
    {"type": "text", "value": "{Aperture} | ISO {ISO} | {Shutter}", "font_size": 16, "color": "#000000"}
  ]
}
```

### Available Variables

- `{Author}` - Photographer name
- `{ISO}` - ISO sensitivity
- `{Aperture}` - Aperture value (f-number)
- `{Shutter}` - Shutter speed
- `{Focal}` - Focal length
- `{Camera}` - Camera model
- `{Lens}` - Lens model
- `{DateTime}` - Photo capture time

### Chinese Font Support

**Important:** The default font (DejaVu Sans) only supports English characters.

For Chinese text in watermarks:

```bash
# Use --font parameter
litemark add -i photo.jpg -o output.jpg --author "张三" \
  --font "/System/Library/Fonts/PingFang.ttc"

# Or set environment variable
export LITEMARK_FONT="/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"
litemark add -i photo.jpg -o output.jpg --author "李四"
```

📚 **See detailed guide:** [Chinese Font Configuration Guide](examples/chinese_font_guide.md)

### Logo Customization

You can specify a custom logo for your watermarks using the `--logo` parameter or `LITEMARK_LOGO` environment variable:

**Using command-line parameter:**
```bash
# Specify logo file path
litemark add -i photo.jpg -o output.jpg --logo my_logo.png

# Use absolute path
litemark add -i photo.jpg -o output.jpg --logo /path/to/brand_logo.png

# Batch processing with custom logo
litemark batch -i photos/ -t classic -o output/ --logo company_logo.png
```

**Using environment variable:**
```bash
# Linux/macOS - Temporary
export LITEMARK_LOGO="/Users/username/logos/my_logo.png"
litemark add -i photo.jpg -o output.jpg

# Linux/macOS - Permanent (add to ~/.bashrc or ~/.zshrc)
echo 'export LITEMARK_LOGO="/path/to/default_logo.png"' >> ~/.zshrc

# Windows PowerShell
$env:LITEMARK_LOGO="C:\logos\my_logo.png"
litemark add -i photo.jpg -o output.jpg
```

**Priority order:**
1. `--logo` CLI parameter (highest priority)
2. `LITEMARK_LOGO` environment variable
3. Logo path defined in template
4. No logo (skip logo rendering)

**Supported logo formats:**
- PNG (recommended for transparency)
- JPEG
- GIF
- WebP
- BMP

## Built-in Templates

1. **ClassicParam** - Bottom frame with centered logo, photographer name, and shooting parameters
2. **Modern** - Modern style frame layout (coming soon)
3. **Minimal** - Minimal frame layout (coming soon)

All templates display:
- Logo in center (top of frame)
- Photographer name (below logo)
- Shooting parameters (ISO, aperture, shutter speed, etc.)

## Documentation

- [Architecture Guide](docs/ARCHITECTURE.md) - Rendering principles and code structure
- [Development Guide](docs/DEVELOPMENT.md) - Setup and contribution guidelines
- [EXIF Extraction Guide](examples/exif_extraction.md) - How EXIF data extraction works
- [Chinese Font Guide](examples/chinese_font_guide.md) - Configure Chinese fonts for watermarks

## Development

### Project Structure

```
src/
├── main.rs           # CLI entry point
├── lib.rs            # Library entry
├── exif_reader/      # EXIF data extraction
├── layout/           # Template engine (JSON parsing, variable substitution)
├── renderer/         # Image rendering (frame generation, font rendering, logo)
└── io/              # File I/O operations
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with example
cargo run -- add -i test_images/800x600_landscape.jpg -t classic -o output.jpg --author "Photographer"
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [x] CLI tool with frame mode
- [x] **Real EXIF data extraction** (kamadak-exif integration)
- [x] Professional font rendering (rusttype)
- [x] Logo support with parameterization
- [x] Template system
- [x] Batch processing
- [x] Comprehensive unit tests
- [ ] Chinese font bundling or better font configuration
- [ ] iOS App integration
- [ ] Web interface (WASM)
- [ ] More template options
- [ ] HEIC/RAW format support
- [ ] Template marketplace

## Support

- 📧 Email: [your-email@example.com]
- 🐛 Issues: [GitHub Issues](https://github.com/26huitailang/lite-mark-core/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/26huitailang/lite-mark-core/discussions)
