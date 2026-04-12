# LiteMark

A lightweight photo parameter frame tool for photography enthusiasts.

## Features

- 📸 **Extract EXIF data** - ISO, aperture, shutter speed, focal length, camera, lens
- 🖼️ **Frame mode** - Add bottom frame with shooting parameters and logo
- 🎨 **Template system** - JSON-based customizable layouts
- 🔤 **Professional font rendering** - Using rusttype, supports custom fonts
- 🖼️ **Logo support** - Automatic scaling and positioning
- 📱 **Batch processing** - Process entire directories with concurrency
- 🔒 **Privacy-first** - All processing happens locally

## Installation

### From Source

```bash
git clone https://github.com/26huitailang/lite-mark-core.git
cd lite-mark-core
cargo build --release
```

## Usage

### Basic

```bash
# Add frame to single image
litemark add -i input.jpg -t classic -o output.jpg

# With author name
litemark add -i input.jpg -t classic -o output.jpg --author "Photographer"

# Batch process
litemark batch -i ./photos/ -t classic -o ./output/

# List templates
litemark templates
```

### Template Variables

- `{Author}`, `{ISO}`, `{Aperture}`, `{Shutter}`, `{Focal}`, `{Camera}`, `{Lens}`, `{DateTime}`

### Chinese Font Support

```bash
# Use --font parameter
litemark add -i photo.jpg -o output.jpg --author "张三" \
  --font "/System/Library/Fonts/PingFang.ttc"

# Or set environment variable
export LITEMARK_FONT="/path/to/NotoSansCJK-Regular.ttc"
```

## Documentation

- [Architecture](litemark-core/ARCHITECTURE.md) - Technical design and rendering principles
- [Chinese Font Guide](examples/chinese_font_guide.md) - Chinese font configuration
- [Project Roadmap](plan.md) - Future plans and milestones

## Project Structure

```
litemark-core/      # Core library (memory-based API)
litemark-cli/       # CLI client
litemark-wasm/      # WASM bindings
templates/          # JSON templates
test_images/        # Test images
```

## Development

```bash
# Build
cargo build --workspace

# Test
cargo test --workspace --exclude litemark-wasm

# Demo
make demo
```

## License

MIT License - see [LICENSE](LICENSE) file.
