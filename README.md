# LiteMark

A lightweight photo parameter frame tool for photography enthusiasts.

## Features

- 📸 Extract EXIF data (ISO, aperture, shutter speed, focal length, etc.)
- 🖼️ Frame mode: Add bottom frame with shooting parameters and logo
- 🎨 Template system with customizable layouts (JSON-based)
- 🔤 Professional font rendering with rusttype (supports English, Chinese, etc.)
- 🖼️ Logo support with automatic scaling
- 📱 Batch processing support
- 🔒 Privacy-first: all processing happens locally
- 🎯 Simple CLI interface

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

## Built-in Templates

1. **ClassicParam** - Bottom frame with centered logo, photographer name, and shooting parameters
2. **Modern** - Modern style frame layout (coming soon)
3. **Minimal** - Minimal frame layout (coming soon)

All templates display:
- Logo in center (top of frame)
- Photographer name (below logo)
- Shooting parameters (ISO, aperture, shutter speed, etc.)

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
- [x] Professional font rendering (rusttype)
- [x] Logo support
- [x] Multi-language support (English, Chinese)
- [x] Template system
- [x] Batch processing
- [ ] iOS App integration
- [ ] Web interface (WASM)
- [ ] More template options
- [ ] HEIC/RAW format support
- [ ] Template marketplace

## Support

- 📧 Email: [your-email@example.com]
- 🐛 Issues: [GitHub Issues](https://github.com/26huitailang/lite-mark-core/issues)
- 💬 Discussions: [GitHub Discussions](https://github.com/26huitailang/lite-mark-core/discussions)
