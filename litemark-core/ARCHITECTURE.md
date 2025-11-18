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
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                  External Dependencies                   │
│    image, libheif-rs, rusttype, kamadak-exif, serde     │
└─────────────────────────────────────────────────────────┘
```

## 模块详解

### 1. image_io 模块

**职责**：图像编解码，所有操作基于内存

**关键函数**：
- `decode_image(data: &[u8]) -> Result<DynamicImage>`
- `encode_image(image: &DynamicImage, format: ImageFormat) -> Result<Vec<u8>>`
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
- `extract_from_bytes(data: &[u8]) -> Result<ExifData>`
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
    pub items: Vec<TemplateItem>,
    pub frame_height_ratio: f32,  // 边框高度比例
    pub primary_font_ratio: f32,   // 主字体比例
    pub secondary_font_ratio: f32, // 副字体比例
    pub padding_ratio: f32,        // 边距比例
    // ...
}
```

**关键函数**：
- `from_json(json: &str) -> Result<Template>`
- `to_json(&self) -> Result<String>`
- `substitute_variables(&self, vars: &HashMap<String, String>) -> Template`
- `create_builtin_templates() -> Vec<Template>`

**设计要点**：
- 完全基于比例的尺寸系统，适配任意分辨率
- 内置模板硬编码，无需外部文件
- JSON 序列化方便跨语言传递（Web、CLI）

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
    font: Font<'static>,
}
```

**关键函数**：
- `new() -> Result<Self>`（默认字体）
- `from_font_bytes(data: Option<&[u8]>) -> Result<Self>`（自定义字体）
- `render_watermark_with_logo_bytes(...) -> Result<()>`

**渲染流程**：
```
1. 计算边框尺寸（基于短边 × frame_height_ratio）
2. 扩展画布（原图高度 + 边框高度）
3. 复制原图到上部
4. 绘制白色边框背景
5. 分类文本项（左栏 vs 右栏）
6. 渲染 Logo（可选）
7. 渲染文本（rusttype 矢量字体）
8. 绘制分隔线
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
  Column 1  Column 2  3   Column 4
```

**字体渲染**：
- 默认嵌入思源黑体（compile-time `include_bytes!`）
- 支持运行时注入自定义字体字节数据
- 使用 rusttype 进行矢量渲染
- 抗锯齿处理，alpha 阈值 > 10

**Logo 渲染**：
- 从字节数据加载（`image::load_from_memory`）
- 保持纵横比缩放至目标高度
- Alpha 通道混合（支持透明背景）
- 失败时静默跳过（不中断流程）

## 设计模式

### 1. Builder Pattern（隐式）

```rust
WatermarkRenderer::new()                    // 默认配置
WatermarkRenderer::from_font_bytes(Some(font)) // 自定义字体
```

### 2. Strategy Pattern

不同的图像格式（JPEG、PNG、HEIC）有不同的解码策略，但对外统一接口：
```rust
decode_image(data) // 内部自动选择解码策略
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

// 自定义字体：泄漏到 'static（rusttype 要求）
let leaked: &'static [u8] = Box::leak(font_data.to_vec().into_boxed_slice());
```

**泄漏理由**：
- rusttype 的 `Font` 需要 `'static` 生命周期
- 字体数据在程序运行期间一直使用，泄漏是可接受的
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

## 测试策略

### 单元测试

每个模块的 `#[cfg(test)] mod tests` 包含：
- 数据结构的构造和转换
- 边界条件测试
- 错误处理验证

### 集成测试

`tests/integration_test.rs` 包含：
- 完整的端到端流程测试
- 多模块协作测试
- 真实场景模拟

### 测试覆盖目标

- 单元测试覆盖率 > 80%
- 所有公开 API 都有集成测试
- 边界条件和错误路径必须覆盖

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

## 未来扩展点

### 1. 新增水印风格

在 `renderer` 中添加新的布局算法，保持接口不变：
```rust
impl WatermarkRenderer {
    fn render_style_classic(...) { }
    fn render_style_modern(...) { }  // 新增
    fn render_style_minimal(...) { } // 新增
}
```

### 2. 支持更多图像格式

在 `image_io` 中添加新的解码器：
```rust
if is_heic_format(data) { ... }
else if is_avif_format(data) { ... } // 新增
else { ... }
```

### 3. GPU 加速渲染

引入 `wgpu` 进行 GPU 加速（可选特性）：
```rust
#[cfg(feature = "gpu-acceleration")]
mod gpu_renderer;
```

### 4. 增量渲染

支持部分区域更新，降低内存占用：
```rust
pub fn render_watermark_region(
    &self,
    image: &mut DynamicImage,
    region: Rect,
    ...
)
```

## 版本演进

### v0.2.0（当前）

- ✅ 基于内存的 API
- ✅ EXIF 字节流提取
- ✅ 字体和 Logo 字节数组输入
- ✅ 集成测试覆盖

### v0.3.0（计划）

- ⏳ WASM 绑定层
- ⏳ 性能基准测试
- ⏳ 更多内置模板

### v1.0.0（目标）

- ⏳ 稳定的公开 API
- ⏳ 完整的文档
- ⏳ 生产级错误处理
