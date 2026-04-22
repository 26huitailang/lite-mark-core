# LiteMark Core 架构设计

本文档描述了 LiteMark Core 库的内部架构和设计决策。

## 整体架构

```
┌─────────────────────────────────────────────────────────┐
│                    Client Applications                   │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐│
│  │   CLI    │  │   Web    │  │   iOS    │  │ Desktop  ││
│  │  Client  │  │  (WASM)  │  │  Client  │  │  Client  ││
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘│
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│              LiteMark Core Library (this)                │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐│
│  │ image_io │  │   exif   │  │  layout  │  │ renderer ││
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘│
│  ┌──────────┐                                            │
│  │  error   │  ← 结构化错误类型 (thiserror)              │
│  └──────────┘                                            │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                  External Dependencies                   │
│    image, libheif-rs, ab_glyph, kamadak-exif, serde, thiserror │
└─────────────────────────────────────────────────────────┘
```

## 模块详解

### 1. image_io 模块

**职责**：图像编解码，所有操作基于内存

**关键函数**：
- `decode_image(data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error>>`
- `encode_image(image: &DynamicImage, format: ImageFormat) -> Result<Vec<u8>, Box<dyn std::error::Error>>`
- `detect_format(data: &[u8]) -> ImageFormat`

**设计要点**：
- 完全基于字节流操作，无文件系统依赖
- HEIC/HEIF 通过魔数检测自动识别
- 输出格式可指定（JPEG、PNG、WebP）

**HEIC 处理流程**：
```
字节数据 → 魔数检测 → HeifContext → 解码RGB → 转换RGBA → DynamicImage
```

### 2. exif 模块

**职责**：从图像字节数据中提取 EXIF 元数据

**核心类型**：
```rust
pub struct ExifData {
    pub iso: Option<u32>,
    pub aperture: Option<f64>,
    pub shutter_speed: Option<String>,
    pub focal_length: Option<f64>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub date_time: Option<String>,
    pub author: Option<String>,
}
```

**关键函数**：
- `extract_from_bytes(data: &[u8]) -> Result<ExifData, Box<dyn std::error::Error>>`
- `to_variables(&self) -> HashMap<String, String>`
- `get_missing_fields(&self) -> Vec<String>`

**设计要点**：
- 使用 `Cursor` 包装字节流进行解析
- 缺失字段返回 `None`，不抛出错误
- 自动格式化数值（光圈 → "f/2.8"，快门 → "1/125"）

**降级策略**：
- 无 EXIF 数据时返回空 `ExifData`
- 部分字段缺失时只影响对应变量

### 3. layout 模块

**职责**：模板定义、序列化、变量替换

**核心类型**：
```rust
pub struct Template {
    pub name: String,
    pub anchor: Anchor,
    pub padding: u32,
    pub items: Vec<TemplateItem>,
    pub frame_height_ratio: f32,
    pub logo_size_ratio: f32,
    pub primary_font_ratio: f32,
    pub secondary_font_ratio: f32,
    pub padding_ratio: f32,
    pub render_mode: RenderMode,  // 新增：渲染模式
    pub background: Option<Background>,
}

pub enum RenderMode {
    BottomFrame,    // 底部纯色框
    GradientFrame,  // 底部渐变过渡
    Overlay,        // 照片内嵌叠加
    Minimal,        // 极简线条
}
```

**关键函数**：
- `from_json(json: &str) -> Result<Self, serde_json::Error>`
- `to_json(&self) -> Result<String, serde_json::Error>`
- `substitute_variables(&self, vars: &HashMap<String, String>) -> Template`
- `create_builtin_templates() -> Vec<Template>`（编译时嵌入 JSON）

**设计要点**：
- 完全基于比例的尺寸系统，适配任意分辨率
- 内置模板通过 `include_str!` 编译时嵌入，无需外部文件
- JSON 序列化方便跨语言传递（Web、CLI）
- 四种渲染模式通过 `render_mode` 字段控制

**变量替换**：
```
模板: "{Camera} • {Lens}"
变量: {"Camera": "Canon R5", "Lens": "RF 24-70"}
结果: "Canon R5 • RF 24-70"
```

### 4. renderer 模块

**职责**：水印渲染引擎

**核心类型**：
```rust
pub struct WatermarkRenderer {
    fonts: FontSet,
}

struct FontSet {  // 定义在 renderer/text.rs
    regular: FontRef<'static>,
    bold: Option<FontRef<'static>>,
}
```

**关键函数**：
- `new() -> Result<Self, Box<dyn std::error::Error>>`（默认字体）
- `from_font_bytes(regular: Option<&[u8]>) -> Result<Self, Box<dyn std::error::Error>>`（自定义字体）
- `from_font_bytes_with_bold(regular, bold) -> Result<Self, Box<dyn std::error::Error>>`（多字重）
- `render_watermark_with_logo_bytes(...) -> Result<(), Box<dyn std::error::Error>>`

**渲染流程**：
```
1. 变量替换（substitute_variables）
2. 按 render_mode 分发：
   - BottomFrame: 扩展画布 → 白底框 → 内容渲染
   - GradientFrame: 扩展画布 → 渐变背景 → 内容渲染
   - Minimal: 扩展画布 → 细线 → 内容渲染
   - Overlay: 不扩展画布 → 右下角半透明背景 → 文字叠加
3. 文本渲染（ab_glyph 矢量字体，支持多字重）
4. Logo 渲染（双线性插值缩放）
```

**布局策略**（四列布局）：
```
┌─────────────────────────────────────────────────────────┐
│                     原始图像                              │
├──────────┬────────┬──┬─────────────────────────────────┤
│ 作者     │ Logo   │░░│ 光圈 | ISO 400 | 1/125          │
│ 相机型号 │        │░░│ 焦距: 50mm                       │
│ 日期时间 │        │░░│                                  │
└──────────┴────────┴──┴─────────────────────────────────┘
  Column 1  Column 2  3   Column 4（右对齐）
```

**字体渲染**：
- 默认嵌入思源黑体（compile-time `include_bytes!`）
- 支持运行时注入自定义字体字节数据（常规体 + 粗体）
- 使用 `ab_glyph` 进行矢量渲染（rusttype 的继任者）
- 字重通过 `FontWeight` 选择：Normal / Bold / Light
- 抗锯齿处理，alpha 混合

**Logo 渲染**：
- 从字节数据加载（`image::load_from_memory`）
- 保持纵横比缩放至目标高度（双线性插值）
- Alpha 通道混合（支持透明背景）
- 失败时静默跳过（不中断流程）

### 5. error 模块（新增）

**职责**：结构化错误类型，替代 `Box<dyn Error>`

**核心类型**：
```rust
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("图像处理错误: {0}")]
    Image(#[from] ImageError),
    #[error("EXIF 解析错误: {0}")]
    Exif(#[from] ExifError),
    #[error("字体错误: {0}")]
    Font(#[from] FontError),
    #[error("模板错误: {0}")]
    Template(#[from] TemplateError),
    #[error("渲染错误: {0}")]
    Render(#[from] RenderError),
}
```

**设计要点**：
- 使用 `thiserror` 自动实现 `Error` trait
- 调用方可精确匹配错误类型（`CoreError::Font` vs `CoreError::Render`）
- 每个子模块有独立的错误枚举
- `#[from]` 自动转换，保持 `?` 操作符的便利性

## 设计模式

### 1. Builder Pattern（隐式）

```rust
WatermarkRenderer::new()                                    // 默认配置
WatermarkRenderer::from_font_bytes(Some(font))              // 自定义字体
WatermarkRenderer::from_font_bytes_with_bold(reg, bold)     // 多字重
```

### 2. Strategy Pattern

不同的图像格式和渲染模式有独立的处理策略，但对外统一接口：
```rust
decode_image(data)          // 内部自动选择解码策略
render_watermark_with_logo_bytes(image, template, ...)  // 内部按 render_mode 分发
```

### 3. Null Object Pattern

EXIF 数据缺失时返回空对象而非错误：
```rust
let exif = extract_from_bytes(invalid_data)?; // Ok(ExifData::new())
```

### 4. Template Method Pattern

渲染流程固定，但可变部分通过模板配置：
```rust
render_watermark_with_logo_bytes(image, template, variables, logo)
```

## 内存管理

### 字体数据生命周期

```rust
// 嵌入字体：'static 生命周期
let font_data = include_bytes!("../../assets/fonts/...");
let font = FontRef::try_from_slice(font_data)?;

// 自定义字体：也泄漏到 'static（简化生命周期管理）
let leaked: &'static [u8] = Box::leak(font_data.to_vec().into_boxed_slice());
let font = FontRef::try_from_slice(leaked)?;
```

**注意**：
- `ab_glyph` 的 `FontRef` 本身不需要 `'static`，但渲染器设计为长期复用
- 自定义字体数据在程序运行期间一直使用，泄漏是可接受的
- Web/WASM 场景下，整个模块会随着页面卸载而释放

### 图像数据流动

```
客户端读取文件 → Vec<u8> → Core 解码 → DynamicImage（堆分配）
                                      ↓
                              渲染修改（in-place）
                                      ↓
                              Core 编码 → Vec<u8> → 客户端写入文件
```

## 错误处理策略

### 1. 传播式错误（Propagate）

大部分函数返回 `Result<T, Box<dyn std::error::Error>>`，让调用者决定如何处理：
```rust
pub fn decode_image(data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error>>
```

### 2. 降级式错误（Degrade）

EXIF 提取失败时返回空数据而不报错：
```rust
let exif = match exifreader.read_from_container(&mut cursor) {
    Ok(exif) => exif,
    Err(_) => return Ok(ExifData::new()), // 降级
};
```

### 3. 静默式错误（Silent）

Logo 加载失败时跳过渲染：
```rust
let logo_img = match image::load_from_memory(logo_data) {
    Ok(img) => img,
    Err(_) => return Ok(()), // 静默跳过
};
```

## 性能优化

### 1. 避免重复解析

```rust
// ✅ 字体只加载一次
let renderer = WatermarkRenderer::new()?;
for image in images {
    renderer.render_watermark_with_logo_bytes(...)?; // 复用字体
}
```

### 2. 原地修改

```rust
// ✅ 直接修改图像，避免拷贝
pub fn render_watermark_with_logo_bytes(&self, image: &mut DynamicImage, ...)
```

### 3. 最小化分配

```rust
// 预分配 RGBA 数据
let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
```

### 4. 基准测试结果（Release 模式）

| 分辨率 | 耗时 | 备注 |
|--------|------|------|
| 800×600 | ~0.94 ms | 小型图像 |
| 1920×1080 | ~3.97 ms | 标准屏幕 |
| 4000×3000 | ~23.4 ms | 高分辨率照片 |

*目标 < 200ms 已大幅超越。*

## 测试策略

### 单元测试

每个模块的 `#[cfg(test)] mod tests` 包含：
- 数据结构的构造和转换
- 边界条件测试
- 错误处理验证

### 集成测试

`tests/` 包含：
- 完整的端到端流程测试（`pipeline_tests.rs`）
- 多模块协作测试（`template_tests.rs`）
- 回归测试（`regression_tests.rs`）
- **视觉回归测试**（`visual_regression_tests.rs`，新增）

### 视觉回归测试

- 每个内置模板在 1920×1080 下生成参考图
- CI 对比 PR 前后的像素差异
- 允许 0.5% 像素容差（抗锯齿差异）
- 更新参考图：`UPDATE_REFS=1 cargo test --test integration -- visual`

### 测试覆盖目标

- 单元测试覆盖率 > 80%
- 所有公开 API 都有集成测试
- 边界条件和错误路径必须覆盖
- 四种渲染模式都有视觉回归测试

## 平台兼容性

### 支持的平台

- ✅ Linux x86_64
- ✅ macOS x86_64/ARM64
- ✅ Windows x86_64
- ✅ WASM32（浏览器）

### 条件编译

```rust
#[cfg(target_arch = "wasm32")]
// WASM 特定代码

#[cfg(not(target_arch = "wasm32"))]
// 原生平台代码
```

### WASM 限制

- 无多线程（Rayon 不可用）
- 无文件系统（std::fs 不可用）
- 内存限制（大图像需要分块处理）

## 版本演进

### v0.2.0（当前）

- ✅ 基于内存的 API
- ✅ EXIF 字节流提取
- ✅ 字体和 Logo 字节数组输入
- ✅ 集成测试覆盖
- ✅ `ab_glyph` 字体引擎（替代 `rusttype`）
- ✅ 多字重字体支持（Regular + Bold）
- ✅ 四种渲染模式（BottomFrame / GradientFrame / Overlay / Minimal）
- ✅ 颜色 alpha 通道支持（`#RRGGBBAA`）
- ✅ 右对齐与视觉层级排版
- ✅ Logo 双线性插值缩放
- ✅ 视觉回归测试 + 性能基准
- ✅ WASM 编译通过
- ⚠️ `CoreError` 已定义（`thiserror`），但公共 API 仍返回 `Box<dyn std::error::Error>`

### v0.3.0（目标）

- ⏳ 公共 API 全面迁移到 `CoreError`
- ⏳ 稳定的公开 API
- ⏳ 完整的用户文档

### v1.0.0（远期）

- ⏳ GPU 加速渲染（可选特性）
