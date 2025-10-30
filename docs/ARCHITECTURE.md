# LiteMark 架构文档

本文档描述 LiteMark 的核心架构和渲染原理，帮助开发者快速理解代码结构和实现细节。

## 目录

1. [整体架构](#整体架构)
2. [核心模块](#核心模块)
3. [渲染流程](#渲染流程)
4. [字体渲染原理](#字体渲染原理)
5. [扩展开发指南](#扩展开发指南)

## 整体架构

```
┌─────────────────────────────────────────────────────────┐
│                     CLI (main.rs)                        │
│  - 参数解析 (clap)                                        │
│  - 命令分发 (add/batch/templates/show-template)          │
└──────────────────┬──────────────────────────────────────┘
                   │
        ┌──────────┴──────────┐
        │                     │
┌───────▼────────┐   ┌────────▼────────┐
│  exif_reader   │   │     layout      │
│                │   │                 │
│  - EXIF 解析    │   │  - 模板加载      │
│  - 数据提取     │   │  - 变量替换      │
│  - 结构化输出   │   │  - 布局计算      │
└───────┬────────┘   └────────┬────────┘
        │                     │
        └──────────┬──────────┘
                   │
          ┌────────▼────────┐
          │    renderer     │
          │                 │
          │  - 相框生成      │
          │  - 字体渲染      │
          │  - Logo 渲染     │
          └────────┬────────┘
                   │
          ┌────────▼────────┐
          │       io        │
          │                 │
          │  - 图片加载      │
          │  - 图片保存      │
          │  - 批量处理      │
          └─────────────────┘
```

## 核心模块

### 1. EXIF Reader (`src/exif_reader/mod.rs`)

负责从图片中提取 EXIF 元数据。

**数据结构：**
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

**实现方式：**
- 使用 `kamadak-exif` 库进行真实 EXIF 解析
- 支持 JPEG、PNG、TIFF 等格式
- 优雅处理无 EXIF 数据的情况

**支持的 EXIF 字段：**

| 字段 | EXIF 标签 | 数据类型 | 说明 |
|------|-----------|----------|------|
| iso | PhotographicSensitivity | SHORT/LONG | ISO 感光度 |
| aperture | FNumber | RATIONAL | 光圈值（f-number） |
| shutter_speed | ExposureTime | RATIONAL | 曝光时间（自动格式化） |
| focal_length | FocalLength | RATIONAL | 焦距（mm） |
| camera_model | Model | ASCII | 相机型号 |
| lens_model | LensModel | ASCII | 镜头型号 |
| date_time | DateTimeOriginal | ASCII | 拍摄时间 |
| author | Artist | ASCII | 作者/摄影师 |

**错误处理策略：**
- 文件无法打开：返回错误
- 文件无 EXIF 数据：返回空的 ExifData（所有字段为 None）
- 特定字段缺失：该字段设为 None，其他字段正常解析
- EXIF 数据损坏：记录警告，返回部分解析的数据

**快门速度格式化：**
```rust
fn format_shutter_speed(exposure_time: f64) -> String {
    if exposure_time >= 1.0 {
        format!("{}s", exposure_time as u32)  // 如 "2s"
    } else {
        let denominator = (1.0 / exposure_time).round() as u32;
        format!("1/{}", denominator)  // 如 "1/125"
    }
}
```

### 2. Layout Engine (`src/layout/mod.rs`)

模板引擎，负责解析 JSON 模板和执行变量替换。

**模板结构：**
```rust
pub struct Template {
    pub name: String,
    pub anchor: Anchor,      // 定位点（当前用于框架，实际不需要）
    pub padding: u32,
    pub items: Vec<TemplateItem>,  // logo 和文字项
    pub background: Option<Background>,
}
```

**变量替换：**
- 支持 `{Author}`, `{ISO}`, `{Aperture}`, `{Shutter}`, `{Focal}`, `{Camera}` 等
- 变量映射来自 EXIF 数据和 CLI 参数

### 3. Renderer (`src/renderer/mod.rs`)

核心渲染引擎，实现相框模式的图像合成。

**关键组件：**

#### 3.1 相框生成
```rust
pub fn render_watermark(
    &self,
    image: &mut DynamicImage,
    template: &Template,
    variables: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>>
```

**渲染流程：**
1. 创建更大的画布（原图高度 + 底部相框 100px）
2. 将原图复制到新画布顶部
3. 绘制底部相框背景（白色）
4. 渲染 Logo（居中，相框上半部分）
5. 渲染文字参数（居中，相框下半部分）

#### 3.2 字体渲染（rusttype）

使用 `rusttype` 库进行专业字体渲染：

```rust
let scale = Scale::uniform(font_size as f32);
let v_metrics = self.font.v_metrics(scale);
let baseline_y = y as f32 - v_metrics.ascent;
let offset = point(x as f32, baseline_y);

let glyphs: Vec<_> = self.font.layout(text, scale, offset).collect();

for glyph in glyphs {
    if let Some(bounding_box) = glyph.pixel_bounding_box() {
        glyph.draw(|px, py, v| {
            // v 是 alpha 值 (0.0-1.0)
            let alpha = (v * 255.0) as u8;
            // 绘制像素
        });
    }
}
```

**特点：**
- 支持 TTF 字体文件
- 支持多语言（中文、英文等）
- 抗锯齿渲染
- 自动处理基线对齐

#### 3.3 Logo 渲染

```rust
fn render_logo(
    &self,
    image: &mut RgbaImage,
    logo_path: &str,
    center_x: i32,
    center_y: i32,
    width: i32,
    height: i32,
) -> Result<(), Box<dyn std::error::Error>>
```

**功能：**
- 自动加载 logo 图片
- 按比例缩放以适配指定尺寸
- 居中显示
- 文件不存在时显示占位符

### 4. IO Module (`src/io/mod.rs`)

文件操作模块，处理图片的加载和保存。

**功能：**
- 图片加载（`load_image`）
- 图片保存（`save_image`）
- 批量处理路径生成（`create_output_path`）

## 渲染流程

### 完整渲染流程

```
1. CLI 解析参数
   ↓
2. 加载图片 (io::load_image)
   ↓
3. 提取 EXIF 数据 (exif_reader::extract_exif_data)
   ↓
4. 加载模板 (layout::load_template)
   ↓
5. 变量替换 (template.substitute_variables)
   ↓
6. 创建渲染器 (renderer::WatermarkRenderer::new)
   ↓
7. 执行渲染 (renderer.render_watermark)
   ├─ 创建相框画布
   ├─ 复制原图
   ├─ 绘制相框背景
   ├─ 渲染 Logo（居中）
   └─ 渲染文字参数（居中）
   ↓
8. 保存结果 (io::save_image)
```

### 相框生成详细流程

```rust
// 1. 计算新画布尺寸
let original_width = image.width();
let original_height = image.height();
let bottom_frame_height = 100;  // 底部相框高度
let new_height = original_height + bottom_frame_height;

// 2. 创建新画布
let mut frame_image = RgbaImage::new(new_width, new_height);

// 3. 复制原图到顶部
for y in 0..original_height {
    for x in 0..original_width {
        frame_image.put_pixel(x, y, *original_rgba.get_pixel(x, y));
    }
}

// 4. 绘制相框背景
let frame_y = original_height;
for y in 0..bottom_frame_height {
    for x in 0..new_width {
        frame_image.put_pixel(x, frame_y + y, WHITE);
    }
}

// 5. 渲染 Logo（居中，相框上半部分）
let logo_y = frame_y + bottom_frame_height / 2 - 15;
render_logo(&mut frame_image, logo_path, center_x, logo_y, 30, 30)?;

// 6. 渲染文字（居中，相框下半部分）
let text_y = frame_y + bottom_frame_height / 2 + 20;
render_text_simple(&mut frame_image, text, center_x, text_y, font_size, BLACK);
```

## 字体渲染原理

### 字体加载

使用 `rusttype` 库加载 TTF 字体：

```rust
let font_data = include_bytes!("../../assets/fonts/DejaVuSans.ttf");
let font = Font::try_from_bytes(font_data)?;
```

字体文件通过 `include_bytes!` 宏编译时嵌入到二进制中，确保运行时可用。

### 文本布局

rusttype 的文本布局过程：

1. **缩放设置：** `Scale::uniform(font_size)` - 统一缩放
2. **垂直度量：** `v_metrics(scale)` - 获取字体的上升、下降、行高等
3. **基线计算：** `baseline_y = y - v_metrics.ascent` - 计算文本基线位置
4. **字形布局：** `font.layout(text, scale, offset)` - 生成字形列表
5. **像素绘制：** `glyph.draw(callback)` - 回调函数绘制每个像素

### 抗锯齿处理

rusttype 提供的 alpha 值范围是 0.0-1.0，用于实现抗锯齿：

```rust
glyph.draw(|px, py, v| {
    let alpha = (v * 255.0) as u8;
    if alpha > 10 {  // 阈值过滤，避免过淡像素
        let pixel_color = Rgba([r, g, b, 255]);
        image.put_pixel(px, py, pixel_color);
    }
});
```

### 多语言支持

- DejaVu Sans 字体支持基本拉丁字符
- 中文等复杂字符需要更大体积的字体文件
- 未来可扩展支持 Noto Sans CJK 等中文字体

## 扩展开发指南

### 添加新模板

1. 在 `templates/` 目录创建 JSON 文件：

```json
{
  "name": "CustomTemplate",
  "anchor": "bottom-left",
  "padding": 0,
  "items": [
    {"type": "logo", "value": "path/to/logo.png"},
    {"type": "text", "value": "{Author}", "font_size": 20, "color": "#000000"},
    {"type": "text", "value": "{Camera} | {ISO}", "font_size": 14, "color": "#000000"}
  ]
}
```

2. 在 `layout/mod.rs` 的 `create_builtin_templates` 中添加：

```rust
Template {
    name: "CustomTemplate".to_string(),
    // ... 配置
}
```

### 添加新的 EXIF 变量

1. 在 `exif_reader/mod.rs` 的 `ExifData` 结构体中添加字段
2. 在 `extract_exif_data` 中提取数据
3. 在 `main.rs` 的变量映射中添加对应的格式化逻辑

### 修改相框样式

编辑 `src/renderer/mod.rs` 的 `render_watermark` 函数：

- 修改 `bottom_frame_height` 改变相框高度
- 修改 `render_frame_background` 改变背景颜色
- 调整 Logo 和文字的位置计算

### 自定义字体

1. 将字体文件放入 `assets/fonts/` 目录
2. 修改 `renderer/mod.rs` 的字体加载路径
3. 重新编译（字体会被嵌入到二进制中）

### 性能优化建议

1. **批量处理：** 使用并行处理（`rayon` crate）
2. **字体缓存：** 复用 `WatermarkRenderer` 实例避免重复加载字体
3. **大图处理：** 考虑流式处理或分块渲染
4. **内存管理：** 及时释放不需要的图像数据

## 测试

运行测试：

```bash
cargo test
```

添加新功能时记得：
1. 编写单元测试
2. 添加集成测试（使用 `test_images/` 中的测试图片）
3. 更新文档

## 下一步开发

- [ ] 真实 EXIF 解析实现
- [ ] iOS 集成（FFI 绑定）
- [ ] WASM 支持
- [ ] 更多模板样式
- [ ] 性能优化（并行处理）

