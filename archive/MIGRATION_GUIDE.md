# LiteMark 模块化迁移指南

本文档指导如何将现有的 LiteMark 单体项目迁移到 Core + CLI 分层架构。

## 迁移概览

### 架构变化

**之前**：
```
litemark/
├── src/
│   ├── main.rs           # CLI + 核心逻辑混合
│   ├── exif_reader/
│   ├── io/
│   ├── layout/
│   └── renderer/
```

**之后**：
```
litemark-core/            # 核心库（新）
├── src/
│   ├── lib.rs
│   ├── image_io.rs      # 纯内存 API
│   ├── exif.rs          # 纯内存 API
│   ├── layout.rs
│   └── renderer.rs      # 支持字节数组

litemark-cli/             # CLI 客户端（待创建）
├── src/
│   ├── main.rs
│   ├── commands.rs
│   ├── batch.rs
│   └── utils.rs
```

## 第一阶段：Core 库已完成

### ✅ 已完成的工作

1. **创建 litemark-core 项目**
   - 位置：`/litemark-core/`
   - 配置：`litemark-core/Cargo.toml`

2. **迁移并重构模块**
   - `layout` → `litemark-core/src/layout.rs`（无修改）
   - `exif_reader` → `litemark-core/src/exif.rs`（新增 `extract_from_bytes`）
   - `io` → `litemark-core/src/image_io.rs`（新增 `decode_image`、`encode_image`）
   - `renderer` → `litemark-core/src/renderer.rs`（新增 `from_font_bytes`、`render_watermark_with_logo_bytes`）

3. **配置 Workspace**
   - 根目录 `Cargo.toml` 已更新为 Workspace 配置
   - 保留向后兼容性

### 关键 API 变化

#### EXIF 提取

**旧版本（文件路径）**：
```rust
// src/exif_reader/mod.rs
pub fn extract_exif_data(image_path: &str) -> Result<ExifData, Box<dyn std::error::Error>>
```

**新版本（字节流）**：
```rust
// litemark-core/src/exif.rs
pub fn extract_from_bytes(image_data: &[u8]) -> Result<ExifData, Box<dyn std::error::Error>>
```

#### 图像 I/O

**旧版本（文件路径）**：
```rust
// src/io/mod.rs
pub fn load_image(path: &str) -> Result<DynamicImage, Box<dyn std::error::Error>>
pub fn save_image(image: &DynamicImage, path: &str) -> Result<(), Box<dyn std::error::Error>>
```

**新版本（字节流）**：
```rust
// litemark-core/src/image_io.rs
pub fn decode_image(data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error>>
pub fn encode_image(image: &DynamicImage, format: ImageFormat) -> Result<Vec<u8>, Box<dyn std::error::Error>>
```

#### 渲染器

**旧版本（文件路径）**：
```rust
// src/renderer/mod.rs
pub fn with_font(font_path: Option<&str>) -> Result<Self, Box<dyn std::error::Error>>
pub fn render_watermark(..., logo_override: Option<&str>) -> Result<(), Box<dyn std::error::Error>>
```

**新版本（字节数组）**：
```rust
// litemark-core/src/renderer.rs
pub fn from_font_bytes(font_data: Option<&[u8]>) -> Result<Self, Box<dyn std::error::Error>>
pub fn render_watermark_with_logo_bytes(..., logo_data: Option<&[u8]>) -> Result<(), Box<dyn std::error::Error>>
```

## 第二阶段：创建 CLI 客户端（待执行）

### 任务清单

#### 1. 创建 litemark-cli 项目

```bash
cargo new --bin litemark-cli
cd litemark-cli
```

在 `litemark-cli/Cargo.toml` 中添加依赖：
```toml
[dependencies]
# Core 库
litemark-core = { path = "../litemark-core" }

# CLI 专属依赖
clap = { version = "4.5.51", features = ["derive"] }
rayon = "1.8"
indicatif = "0.18.3"
walkdir = "2.4"
num_cpus = "1.16"
anyhow = "1.0"
```

#### 2. 迁移 main.rs

将现有的 `src/main.rs` 拆分为：

**litemark-cli/src/main.rs**（命令行解析）：
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "litemark")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { /* ... */ },
    Batch { /* ... */ },
    Templates,
    ShowTemplate { template: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Add { ... } => commands::add(...),
        Commands::Batch { ... } => commands::batch(...),
        Commands::Templates => commands::list_templates(),
        Commands::ShowTemplate { template } => commands::show_template(&template),
    }
    
    Ok(())
}
```

#### 3. 创建 commands.rs

**litemark-cli/src/commands.rs**：
```rust
use litemark_core::{image_io, exif, layout, renderer::WatermarkRenderer};
use std::collections::HashMap;

pub fn add(
    input: &str,
    template: &str,
    output: &str,
    author: Option<&str>,
    font: Option<&str>,
    logo: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing image: {}", input);

    // CLI 层：文件读取
    let image_data = std::fs::read(input)?;
    let font_data = font.map(|p| std::fs::read(p)).transpose()?;
    let logo_data = logo.map(|p| std::fs::read(p)).transpose()?;

    // Core 层：图像处理
    let mut image = image_io::decode_image(&image_data)?;
    let exif_data = exif::extract_from_bytes(&image_data)?;
    
    let mut variables = exif_data.to_variables();
    if let Some(author_name) = author {
        variables.insert("Author".to_string(), author_name.to_string());
    }

    let template = load_template(template)?;
    let renderer = WatermarkRenderer::from_font_bytes(font_data.as_deref())?;
    renderer.render_watermark_with_logo_bytes(
        &mut image,
        &template,
        &variables,
        logo_data.as_deref(),
    )?;

    // CLI 层：文件写入
    let output_bytes = image_io::encode_image(&image, image::ImageFormat::Jpeg)?;
    std::fs::write(output, output_bytes)?;

    println!("Saved watermarked image: {}", output);
    Ok(())
}

pub fn list_templates() {
    let templates = layout::create_builtin_templates();
    for template in templates {
        println!("  • {}", template.name);
    }
}

pub fn show_template(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let template = load_template(name)?;
    let json = template.to_json()?;
    println!("{}", json);
    Ok(())
}

fn load_template(name: &str) -> Result<layout::Template, Box<dyn std::error::Error>> {
    // 尝试内置模板
    let builtin = layout::create_builtin_templates();
    if let Some(t) = builtin.iter().find(|t| t.name == name) {
        return Ok(t.clone());
    }

    // 尝试文件路径
    if std::path::Path::new(name).exists() {
        let json = std::fs::read_to_string(name)?;
        return layout::Template::from_json(&json);
    }

    Err(format!("Template '{}' not found", name).into())
}
```

#### 4. 创建 batch.rs

**litemark-cli/src/batch.rs**：
```rust
use litemark_core::{image_io, exif, layout, renderer::WatermarkRenderer};
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

pub fn process_batch(
    input_dir: &str,
    template_name: &str,
    output_dir: &str,
    author: Option<&str>,
    font: Option<&str>,
    logo: Option<&str>,
    concurrency: Option<usize>,
    show_progress: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 创建输出目录
    std::fs::create_dir_all(output_dir)?;

    // 查找图像文件
    let images = find_images(input_dir)?;
    if images.is_empty() {
        println!("⚠️  No images found");
        return Ok(());
    }

    println!("Found {} images", images.len());

    // 预加载资源
    let font_data = font.map(|p| std::fs::read(p)).transpose()?;
    let logo_data = logo.map(|p| std::fs::read(p)).transpose()?;
    let template = super::commands::load_template(template_name)?;
    let renderer = WatermarkRenderer::from_font_bytes(font_data.as_deref())?;

    // 配置并发
    let cpus = num_cpus::get();
    let threads = concurrency.unwrap_or(cpus * 2).max(1).min(32);
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()?;

    // 进度条
    let pb = if show_progress {
        Some(ProgressBar::new(images.len() as u64))
    } else {
        None
    };

    // 并行处理
    let results: Vec<_> = images
        .par_iter()
        .map(|path| {
            let result = process_single_in_batch(
                path,
                &template,
                &renderer,
                output_dir,
                author,
                logo_data.as_deref(),
            );
            if let Some(ref pb) = pb {
                pb.inc(1);
            }
            result
        })
        .collect();

    if let Some(pb) = pb {
        pb.finish();
    }

    // 统计结果
    let succeeded = results.iter().filter(|r| r.is_ok()).count();
    let failed = results.len() - succeeded;
    println!("\n✓ Succeeded: {}", succeeded);
    println!("✗ Failed: {}", failed);

    Ok(())
}

fn find_images(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    use walkdir::WalkDir;
    let exts = ["jpg", "jpeg", "png", "heic", "heif"];
    
    let images: Vec<String> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| exts.contains(&s.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();

    Ok(images)
}

fn process_single_in_batch(
    input_path: &str,
    template: &layout::Template,
    renderer: &WatermarkRenderer,
    output_dir: &str,
    author: Option<&str>,
    logo_data: Option<&[u8]>,
) -> Result<(), String> {
    // 读取文件
    let image_data = std::fs::read(input_path).map_err(|e| e.to_string())?;
    
    // Core 处理
    let mut image = image_io::decode_image(&image_data).map_err(|e| e.to_string())?;
    let exif_data = exif::extract_from_bytes(&image_data).map_err(|e| e.to_string())?;
    
    let mut variables = exif_data.to_variables();
    if let Some(author_name) = author {
        variables.insert("Author".to_string(), author_name.to_string());
    }
    
    renderer
        .render_watermark_with_logo_bytes(&mut image, template, &variables, logo_data)
        .map_err(|e| e.to_string())?;
    
    // 生成输出路径
    let file_name = std::path::Path::new(input_path)
        .file_stem()
        .unwrap()
        .to_string_lossy();
    let output_path = format!("{}/{}_watermarked.jpg", output_dir, file_name);
    
    // 保存文件
    let output_bytes = image_io::encode_image(&image, image::ImageFormat::Jpeg)
        .map_err(|e| e.to_string())?;
    std::fs::write(&output_path, output_bytes).map_err(|e| e.to_string())?;
    
    Ok(())
}
```

#### 5. 更新 Workspace

在根目录 `Cargo.toml` 中添加：
```toml
[workspace]
members = [
    "litemark-core",
    "litemark-cli",  # 新增
]
```

## 第三阶段：验证迁移（待执行）

### 测试步骤

1. **编译 Core 库**：
```bash
cd litemark-core
cargo build --release
cargo test
```

2. **编译 CLI 客户端**：
```bash
cd litemark-cli
cargo build --release
```

3. **功能测试**：
```bash
# 单图处理
./target/release/litemark add -i test.jpg -t classic -o output.jpg

# 批量处理
./target/release/litemark batch -i ./photos/ -t classic -o ./output/

# 列出模板
./target/release/litemark templates
```

4. **性能对比**：
```bash
# 测试批量处理性能
time ./target/release/litemark batch -i ./photos/ -t classic -o ./output/
```

## 常见问题

### Q1: 原有的 CLI 命令是否还能使用？

A: 可以。根目录保留了原有的单体项目配置，可以继续使用 `cargo run -- add ...` 运行。但建议逐步迁移到新的 CLI 客户端。

### Q2: Core 库如何处理文件系统操作？

A: Core 库**不处理**文件系统。所有文件读写由客户端层（CLI、Web）负责，Core 只处理内存中的字节数据。

### Q3: 如何在 Web 项目中使用 Core 库？

A: 创建 WASM 绑定层：
```rust
// litemark-web/wasm/src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_image(data: &[u8], template_json: &str) -> Result<Vec<u8>, JsValue> {
    let mut image = litemark_core::image_io::decode_image(data)?;
    let template = litemark_core::layout::Template::from_json(template_json)?;
    // ...
    Ok(output_bytes)
}
```

### Q4: 字体和 Logo 如何传递？

A: 通过字节数组传递：
```rust
// CLI: 从文件读取
let font_bytes = std::fs::read("font.ttf")?;
let logo_bytes = std::fs::read("logo.png")?;

// Web: 从 File API 获取
let font_bytes = /* ArrayBuffer from File input */;
let logo_bytes = /* ArrayBuffer from File input */;

// Core: 统一接口
renderer.from_font_bytes(Some(&font_bytes))?;
renderer.render_watermark_with_logo_bytes(..., Some(&logo_bytes))?;
```

## 迁移检查清单

### Core 库
- [x] 创建 litemark-core 项目
- [x] 迁移 layout 模块
- [x] 重构 exif 模块（新增字节流接口）
- [x] 重构 io 模块（新增编解码接口）
- [x] 重构 renderer 模块（支持字节数组）
- [x] 编写集成测试
- [x] 配置 Workspace

### CLI 客户端（待完成）
- [ ] 创建 litemark-cli 项目
- [ ] 迁移命令行参数解析
- [ ] 实现单图处理命令
- [ ] 实现批量处理命令
- [ ] 实现模板管理命令
- [ ] 添加功能测试
- [ ] 性能基准测试

### Web 客户端（未来）
- [ ] 创建 litemark-web/wasm 项目
- [ ] 实现 wasm-bindgen 绑定
- [ ] 创建 Vue 3 前端
- [ ] 集成 WASM 模块
- [ ] 部署到静态托管

## 参考资料

- [Core 库 README](litemark-core/README.md)
- [Core 库架构设计](litemark-core/ARCHITECTURE.md)
- [设计文档](/data/.task/design.md)
