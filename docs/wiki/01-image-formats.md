# 图像格式与编解码

> 理解图像格式是理解 LiteMark 图像 I/O 模块的基础。不同格式有不同的压缩方式、特性支持和适用场景。

---

## 1. 位图与矢量图

### 1.1 两种图像类型

```
位图（Bitmap / Raster）              矢量图（Vector）
┌─────────────────────────┐         ┌─────────────────────────┐
│  ■ □ ■ □ □ ■ □ ■ □ □   │         │                         │
│  □ ■ □ □ ■ □ □ □ ■ □   │         │    ┌─────┐              │
│  □ □ □ ■ □ □ ■ □ □ ■   │         │    │ 矩形 │    ◯ 圆形    │
│  ■ □ □ □ □ ■ □ □ ■ □   │         │    └─────┘              │
│                         │         │         ～ 曲线         │
│  由像素网格组成           │         │  由数学公式描述           │
│  放大后变模糊             │         │  放大后依然清晰           │
└─────────────────────────┘         └─────────────────────────┘
```

| 特性 | 位图 | 矢量图 |
|------|------|--------|
| 存储方式 | 每个像素的颜色值 | 形状、路径、填充的数学描述 |
| 缩放效果 | 放大变模糊（锯齿） | 任意缩放都清晰 |
| 文件大小 | 通常较大 | 通常较小 |
| 适用场景 | 照片、复杂图像 | Logo、图标、字体 |
| 常见格式 | JPEG, PNG, GIF, WebP | SVG, AI, EPS |

LiteMark 处理的是**位图**（照片），但渲染文字时底层使用的是**矢量字体**（见 [04-font-rendering.md](04-font-rendering.md)）。

---

## 2. JPEG：有损压缩的王者

### 2.1 核心特点

- **有损压缩**：牺牲部分细节换取更小的文件体积
- **不支持透明度**：只有 RGB 三通道
- **可调节质量**：质量越高，文件越大，画质越好

### 2.2 压缩原理（简化）

JPEG 压缩分三步：

```
原始图像 (RGB)
      │
      ▼
┌─────────────┐
│  色彩空间转换 │  ← RGB → YCbCr（亮度 + 两个色度通道）
└─────────────┘
      │
      ▼
┌─────────────┐
│   下采样     │  ← 色度通道分辨率降低（人眼对亮度更敏感）
└─────────────┘
      │
      ▼
┌─────────────┐
│  DCT + 量化  │  ← 离散余弦变换，丢弃高频细节
└─────────────┘
      │
      ▼
┌─────────────┐
│  熵编码      │  ← Huffman 编码进一步压缩
└─────────────┘
      │
      ▼
  JPEG 文件
```

**关键 insight**：人眼对亮度变化敏感，对颜色变化不敏感。JPEG 通过降低色度精度来压缩，而尽量保留亮度信息。

### 2.3 质量参数

LiteMark 输出 JPEG 时设置质量为 **90%**：

```rust
ImageFormat::Jpeg => ImageOutputFormat::Jpeg(90),
```

| 质量 | 视觉表现 | 文件大小 | 适用场景 |
|------|---------|---------|---------|
| 100 | 几乎无损 | 最大 | 存档 |
| 90 | 肉眼难辨损失 | 适中 | **LiteMark 默认** |
| 75 | 轻微压缩痕迹 | 较小 | 网页 |
| 50 | 明显块状伪影 | 很小 | 预览缩略图 |

---

## 3. PNG：无损透明的选择

### 3.1 核心特点

- **无损压缩**：不丢失任何像素信息
- **支持透明度**：RGBA 四通道
- **文件较大**：无损压缩率有限

### 3.2 压缩方式

PNG 使用 **DEFLATE** 算法（结合 LZ77 和 Huffman 编码）：

```
原始像素行
    │
    ▼
过滤（Filter）← 用相邻像素预测当前像素，存储差值
    │
    ▼
DEFLATE 压缩 ← LZ77 查找重复模式 + Huffman 编码
    │
    ▼
PNG 文件
```

**过滤**是关键：相邻像素通常颜色相近，存储差值而非绝对值能产生大量重复数据，提高压缩率。

### 3.3 何时用 PNG

LiteMark 中 PNG 的使用场景：
1. **Logo 输入**：PNG 支持透明背景，Logo 可叠加到任意背景上
2. **测试图像**：测试中生成 PNG 进行像素级对比（无损保证一致性）

---

## 4. WebP：Google 的双模格式

### 4.1 核心特点

WebP 是 Google 开发的格式，**同时支持有损和无损压缩**：

| 模式 | 对标格式 | 优势 |
|------|---------|------|
| 有损 WebP | JPEG | 同等质量下文件更小（约 25%~35%） |
| 无损 WebP | PNG | 同等质量下文件更小（约 26%） |
| 有损+透明 | — | JPEG 不支持透明，WebP 支持 |

### 4.2 LiteMark 中的支持

```rust
ImageFormat::WebP => ImageOutputFormat::WebP,
```

LiteMark 支持 WebP 作为输出格式，但具体编码质量由 `image` 库控制。

---

## 5. 文件格式检测：魔数（Magic Number）

### 5.1 什么是魔数

文件开头的几个字节是**魔数（Magic Number / File Signature）**，用于标识文件类型。操作系统和程序通过魔数而非扩展名来判断文件格式。

### 5.2 常见图像魔数

| 格式 | 魔数（Hex） | ASCII 表示 |
|------|------------|-----------|
| JPEG | `FF D8 FF` | — |
| PNG | `89 50 4E 47 0D 0A 1A 0A` | `‰PNG....` |
| GIF | `47 49 46 38` | `GIF8` |
| WebP | `52 49 46 46` | `RIFF` |
| BMP | `42 4D` | `BM` |

### 5.3 LiteMark 中的格式检测

LiteMark 使用 `image` 库的 `guess_format` 函数：

```rust
pub fn detect_format(image_data: &[u8]) -> ImageFormat {
    if is_heic_format(image_data) {
        return ImageFormat::Jpeg;  // HEIC 需要特殊处理
    }
    image::guess_format(image_data).unwrap_or(ImageFormat::Jpeg)
}
```

---

## 6. HEIC/HEIF：Apple 的高效格式

HEIC 需要特殊处理，详见 [07-heic-decoding.md](07-heic-decoding.md)。

---

## 7. Rust `image` 库的内存模型

### 7.1 DynamicImage

`image` 库用 `DynamicImage` 作为统一图像容器：

```rust
pub enum DynamicImage {
    ImageLuma8(GrayImage),      // 8-bit 灰度
    ImageLumaA8(GrayAlphaImage), // 8-bit 灰度 + Alpha
    ImageRgb8(RgbImage),         // 8-bit RGB
    ImageRgba8(RgbaImage),       // 8-bit RGBA  ← LiteMark 主要使用
    // ... 16-bit 变体
}
```

### 7.2 为什么统一转为 RGBA

LiteMark 将所有图像统一转为 **RGBA** 处理：

```rust
// 解码后统一转为 RGBA
let original_rgba = image.to_rgba8();

// 渲染在 RGBA 图像上进行
let mut frame_image = RgbaImage::new(new_width, new_height);

// 最终输出时再按目标格式编码
```

原因：
1. **统一接口**：RGBA 有 Alpha 通道，所有操作（混合、叠加）都能进行
2. **简化逻辑**：无需在 RGB/RGBA/灰度之间频繁转换
3. **内存连续**：`RgbaImage` 底层是 `Vec<u8>`，每 4 字节一个像素

### 7.3 像素布局

```
RgbaImage 内存布局（以 2×2 图像为例）：

像素坐标：          内存中的 Vec<u8>：
┌────────┬────────┐   [R,G,B,A, R,G,B,A, R,G,B,A, R,G,B,A]
│ (0,0)  │ (1,0)  │    ↑0,0    ↑1,0    ↑0,1    ↑1,1
├────────┼────────┤    0..4    4..8    8..12   12..16
│ (0,1)  │ (1,1)  │
└────────┴────────┘

offset = (y * width + x) * 4
pixel = &data[offset .. offset + 4]
```

---

## 8. 编解码流程总结

```
输入文件                    处理                      输出文件
┌─────────┐            ┌──────────────┐            ┌─────────┐
│ photo.  │   decode   │              │   encode   │ output. │
│  jpg    │ ─────────→ │  DynamicImage│ ─────────→ │  jpg    │
└─────────┘            │  (RGBA)      │            └─────────┘
                       │              │
                       │  扩展画布     │
                       │  渲染文字/Logo│
                       │  Alpha 混合   │
                       └──────────────┘
```

---

## 小结

| 格式 | 压缩 | 透明 | 适用场景 | LiteMark 用途 |
|------|------|------|---------|--------------|
| JPEG | 有损 | ❌ | 照片存储 | 默认输出格式（质量90） |
| PNG | 无损 | ✅ | 图标、截图 | Logo 输入、测试参考图 |
| WebP | 有损/无损 | ✅ | 网页优化 | 可选输出格式 |
| HEIC | 有损 | ✅ | iPhone 照片 | 输入解码（需 libheif） |

---

## 延伸阅读

- [07-heic-decoding.md](07-heic-decoding.md) — HEIC/HEIF 格式的深度解析
- [02-pixel-and-color.md](02-pixel-and-color.md) — RGBA 像素与颜色模型
- [05-image-compositing.md](05-image-compositing.md) — 图像合成技术
