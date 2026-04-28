# LiteMark Core Library

LiteMark 核心库，提供纯粹的图像水印处理能力，支持多平台复用（CLI、Web、Desktop）。

## 功能特性

- **平台无关**：所有接口基于内存操作，不依赖文件系统
- **格式支持**：支持 JPEG、PNG、GIF、BMP、WebP、HEIC/HEIF 解码；编码输出支持 JPEG、PNG、WebP
- **EXIF 提取**：从图像字节数据中提取拍摄参数
- **模板引擎**：灵活的 JSON 模板系统
- **水印渲染**：支持中英文文本、Logo、自定义字体

## 核心模块

### image_io - 图像编解码

```rust
use litemark_core::image_io;

// 从字节数据解码图像
let image_bytes = std::fs::read("photo.jpg")?;
let image = image_io::decode_image(&image_bytes)?;

// 编码为 JPEG（支持 Jpeg、Png、WebP）
let output_bytes = image_io::encode_image(&image, image::ImageFormat::Jpeg)?;
```

### exif - EXIF 提取

```rust
use litemark_core::exif;

// 从字节数据提取 EXIF
let image_bytes = std::fs::read("photo.jpg")?;
let exif_data = exif::extract_from_bytes(&image_bytes)?;

// 转换为模板变量
let variables = exif_data.to_variables();
println!("ISO: {:?}", variables.get("ISO"));
```

### layout - 模板引擎

```rust
use litemark_core::layout;

// 获取内置模板
let templates = layout::create_builtin_templates();
let classic = &templates[0];

// 从 JSON 解析自定义模板
let json = r#"{"name": "Custom", ...}"#;
let template = layout::Template::from_json(json)?;
```

### renderer - 水印渲染

```rust
use litemark_core::renderer::WatermarkRenderer;

// 创建渲染器（使用默认字体）
let renderer = WatermarkRenderer::new()?;

// 或使用自定义字体
let font_bytes = std::fs::read("custom.ttf")?;
let renderer = WatermarkRenderer::from_font_bytes(Some(&font_bytes))?;

// 渲染水印
let mut image = image_io::decode_image(&image_bytes)?;
renderer.render_watermark_with_logo_bytes(&mut image, &template, &variables, None)?;

// 编码输出
let output = image_io::encode_image(&image, image::ImageFormat::Jpeg)?;
```

## 设计原则

### 1. 无副作用

所有函数保持纯函数特性，不进行文件系统操作：

```rust
// ✅ 正确：基于内存的接口
fn decode_image(data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error>>;

// ❌ 错误：依赖文件路径
fn load_image(path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>>;
```

### 2. 平台无关

不依赖特定平台的 API：

```rust
// ✅ 正确：字节数组输入
let font_bytes = /* 从任何来源获取 */;
let renderer = WatermarkRenderer::from_font_bytes(Some(&font_bytes))?;

// ❌ 错误：依赖文件系统
let renderer = WatermarkRenderer::with_font(Some("/path/to/font.ttf"))?;
```

### 3. 内存安全

所有图像数据通过内存传递：

```rust
// 完整的处理流程，无需文件系统
let image_data = /* 从网络、内存等获取 */;
let mut image = image_io::decode_image(&image_data)?;
let exif = exif::extract_from_bytes(&image_data)?;
renderer.render_watermark_with_logo_bytes(&mut image, &template, &exif.to_variables(), logo_data)?;
let output = image_io::encode_image(&image, ImageFormat::Jpeg)?;
```

## 依赖关系

Core 库仅依赖以下必要的 crate：

- `image`: 图像处理
- `libheif-rs`: HEIC 格式支持
- `ab_glyph`: 字体渲染
- `kamadak-exif`: EXIF 解析
- `serde`, `serde_json`: 数据序列化
- `thiserror`: 错误处理
- `anyhow`: 便捷错误处理（内部使用）

**不包含**：

- ❌ `clap`: CLI 参数解析（属于 CLI 层）
- ❌ `rayon`: 并行处理（属于客户端层）
- ❌ `indicatif`: 进度条（属于客户端层）
- ❌ `walkdir`: 文件遍历（属于客户端层）

## 使用场景

### CLI 应用

CLI 层负责文件系统操作，Core 层提供处理能力：

```rust
// CLI 层：读取文件
let image_data = std::fs::read(input_path)?;
let logo_data = std::fs::read(logo_path).ok();

// Core 层：处理图像
let mut image = litemark_core::image_io::decode_image(&image_data)?;
let exif = litemark_core::exif::extract_from_bytes(&image_data)?;
renderer.render_watermark_with_logo_bytes(&mut image, &template, &exif.to_variables(), logo_data.as_deref())?;

// CLI 层：保存文件
let output = litemark_core::image_io::encode_image(&image, ImageFormat::Jpeg)?;
std::fs::write(output_path, output)?;
```

### Web/WASM 应用

通过 wasm-bindgen 暴露接口：

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_image(
    image_data: &[u8],
    template_json: &str,
    logo_data: Option<Vec<u8>>,
) -> Result<Vec<u8>, JsValue> {
    let mut image = litemark_core::image_io::decode_image(image_data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let exif = litemark_core::exif::extract_from_bytes(image_data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let template = litemark_core::layout::Template::from_json(template_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let renderer = litemark_core::renderer::WatermarkRenderer::new()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    renderer.render_watermark_with_logo_bytes(&mut image, &template, &exif.to_variables(), logo_data.as_deref())
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let output = litemark_core::image_io::encode_image(&image, image::ImageFormat::Jpeg)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(output)
}
```

## 测试

运行测试：

```bash
cd litemark-core
cargo test
```

## 许可证

MIT License
