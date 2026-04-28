# HEIC/HEIF 格式深度解析

> HEIC 是 iPhone 默认的照片格式。LiteMark 通过 `libheif-rs` 支持 HEIC 解码，但这个过程涉及一些独特的容器格式和色彩空间概念。

---

## 1. HEIC 是什么

### 1.1 背景

**HEIC（High Efficiency Image Container）** 是 Apple 在 iOS 11 开始采用的默认照片格式。它基于 **HEIF（High Efficiency Image Format）** 标准。

| 特性 | HEIC | JPEG |
|------|------|------|
| 压缩效率 | 高约 50% | 基准 |
| 支持透明 | ✅ | ❌ |
| 支持多图 | ✅（Live Photo） | ❌ |
| 兼容性 | Apple 生态好 |  universally |

### 1.2 为什么需要专门处理

HEIC 不是简单的"另一种图像格式"，它是一个**容器格式**，内部结构远比 JPEG 复杂：

```
JPEG:  文件头 → 压缩像素流 → 文件尾     (线性结构)
HEIC:  多个 box 嵌套，像文件系统         (树形结构)
```

Rust 的 `image` 库原生不支持 HEIC，因此 LiteMark 需要额外引入 `libheif-rs`。

---

## 2. ISOBMFF 容器结构

### 2.1 Box 模型

HEIC 基于 **ISOBMFF（ISO Base Media File Format）**，核心概念是 **Box**：

```
每个 Box：
┌─────────────┬─────────────┬─────────────┐
│   Size      │    Type     │    Data     │
│  4 bytes    │  4 bytes    │  Size-8     │
└─────────────┴─────────────┴─────────────┘

Size = 8 + Data 长度
Type = "ftyp", "meta", "mdat" 等
```

Box 可以嵌套：一个 Box 的 Data 部分可以包含更多 Box。

### 2.2 关键 Box 类型

```
HEIC 文件顶层结构：
┌─────────────────────────────────────┐
│  ftyp box                           │ ← 文件类型/品牌标识
│  ├── brand: "heic"                  │
│  └── compatible brands              │
├─────────────────────────────────────┤
│  meta box                           │ ← 元数据（Item 描述）
│  ├── hdlr (handler)                 │
│  ├── pitm (primary item)            │
│  ├── iloc (item location)           │
│  └── iinf (item info)               │
├─────────────────────────────────────┤
│  mdat box                           │ ← 实际的压缩图像数据
│  └── HEVC 编码的图像帧               │
└─────────────────────────────────────┘
```

### 2.3 ftyp Box 与魔数检测

LiteMark 通过检测 `ftyp` box 来识别 HEIC：

```rust
fn is_heic_format(data: &[u8]) -> bool {
    if data.len() < 12 {
        return false;
    }
    // ftyp box 从第 4 字节开始
    if &data[4..8] == b"ftyp" {
        let brand = &data[8..12];
        matches!(brand, b"heic" | b"heix" | b"hevc" | b"hevx" | b"mif1")
    } else {
        false
    }
}
```

解析：
- `data[0..4]`：`ftyp` box 的 Size（大端序）
- `data[4..8]`：Box Type = `"ftyp"`
- `data[8..12]`：Major Brand，标识具体格式变体

品牌标识说明：

| Brand | 含义 |
|-------|------|
| `heic` | HEIF 图像，HEVC 编码 |
| `heix` | HEIF 图像，10bit HEVC |
| `hevc` | HEVC 序列 |
| `hevx` | HEVC 序列，10bit |
| `mif1` | 通用 HEIF |

---

## 3. HEVC 编码

### 3.1 什么 HEVC

HEVC（High Efficiency Video Coding，也叫 H.265）是一种视频编码标准。HEIC 用 HEVC 的**帧内编码（Intra）**模式来压缩单张图片。

对比 JPEG 和 HEVC Intra：

| | JPEG | HEVC Intra |
|--|------|-----------|
| 编码单元 | 8×8 块 | 最大 64×64 的 CTU |
| 变换 | DCT | DCT + DST |
| 预测 | 无帧内预测 | 35 种帧内预测模式 |
| 效率 | 基准 | 约 2 倍 |

### 3.2 为什么压缩率更高

HEVC 的核心优势：

1. **更大的编码单元**：64×64 的块能更好地利用大面积平坦区域
2. **帧内预测**：用相邻已解码像素预测当前块，只存储残差
3. **更精细的量化**：根据内容自适应调整压缩强度

---

## 4. LiteMark 的 HEIC 解码流程

### 4.1 完整流程

```
HEIC 文件字节
      │
      ▼
┌─────────────┐
│ HeifContext │  ← libheif-rs 解析容器结构
│::read_from  │
│_bytes()     │
└─────────────┘
      │
      ▼
┌─────────────┐
│ primary_    │  ← 获取主图像句柄
│ image_      │
│ handle()    │
└─────────────┘
      │
      ▼
┌─────────────┐
│ LibHeif::   │  ← HEVC 解码
│ decode()    │     ColorSpace::Rgb(RgbChroma::Rgb)
└─────────────┘
      │
      ▼
┌─────────────┐
│ RGB Plane   │  ← 获取解码后的 RGB 数据
│ interleaved │
└─────────────┘
      │
      ▼
┌─────────────┐
│ RGB → RGBA  │  ← LiteMark 手动转换，添加 Alpha=255
│ 转换循环     │
└─────────────┘
      │
      ▼
   RgbaImage
```

### 4.2 关键代码解析

```rust
fn decode_heic_from_bytes(data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    // 1. 从字节创建 HEIF 上下文
    let ctx = HeifContext::read_from_bytes(data)?;
    let handle = ctx.primary_image_handle()?;

    // 2. 获取图像尺寸
    let width = handle.width();
    let height = handle.height();

    // 3. 解码为 RGB
    let image = libheif_rs::LibHeif::new()
        .decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;

    // 4. 获取 RGB 平面数据
    let planes = image.planes();
    let interleaved_plane = planes.interleaved.ok_or("No interleaved plane")?;

    // 5. RGB → RGBA 转换
    let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);
    let rgb_data = interleaved_plane.data;
    let stride = interleaved_plane.stride;

    for y in 0..height {
        let row_start = y as usize * stride;
        for x in 0..width {
            let pixel_start = row_start + (x as usize * 3);
            if pixel_start + 2 < rgb_data.len() {
                rgba_data.push(rgb_data[pixel_start]);     // R
                rgba_data.push(rgb_data[pixel_start + 1]); // G
                rgba_data.push(rgb_data[pixel_start + 2]); // B
                rgba_data.push(255);                        // A (不透明)
            }
        }
    }

    // 6. 创建 RgbaImage
    let rgba_image = RgbaImage::from_raw(width, height, rgba_data)
        .ok_or("Failed to create RGBA image from HEIC data")?;

    Ok(DynamicImage::ImageRgba8(rgba_image))
}
```

### 4.3 Stride（步幅）的概念

注意代码中的 `stride`：

```
RGB 平面内存布局（带 stride）：

理论布局（width=3, 无 padding）：
[R,G,B, R,G,B, R,G,B]  ← 每行 9 bytes

实际布局（stride=12）：
[R,G,B, R,G,B, R,G,B, X,X,X]  ← 每行 12 bytes
                 ↑
              padding（对齐用）

stride = 每行实际占用的字节数（≥ width * 3）
```

**为什么有 stride？** 许多图像库为了内存对齐（CPU 访问效率），会在每行末尾填充额外字节。解码时必须用 `stride` 而非 `width * channels` 来计算行偏移。

---

## 5. 跨平台限制

### 5.1 WASM 不支持 HEIC

```rust
#[cfg(target_arch = "wasm32")]
fn decode_heic_from_bytes(_data: &[u8]) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    Err("HEIC/HEIF decoding is not supported on WebAssembly".into())
}
```

WASM 不支持 HEIC 的原因：
1. `libheif` 依赖 C++ 原生库，无法编译到 WASM
2. WASM 沙箱无文件系统，而 libheif 内部有文件 I/O 假设
3. 体积考虑：HEVC 解码器代码量很大

### 5.2 条件编译

LiteMark 使用 Rust 的条件编译区分平台：

```rust
#[cfg(not(target_arch = "wasm32"))]
use libheif_rs::{ColorSpace, HeifContext, RgbChroma};

#[cfg(not(target_arch = "wasm32"))]
fn is_heic_format(data: &[u8]) -> bool { /* ... */ }

#[cfg(target_arch = "wasm32")]
fn is_heic_format(_data: &[u8]) -> bool {
    false  // WASM 直接返回 false，走普通解码路径
}
```

---

## 6. 色彩空间

### 6.1 解码时指定的色彩空间

```rust
ColorSpace::Rgb(RgbChroma::Rgb)
```

`libheif` 支持多种色彩空间：

| 色彩空间 | 说明 |
|---------|------|
| `Rgb(Rgb)` | 标准 RGB，8bit/通道 |
| `Rgb(Rgba)` | RGB + Alpha |
| `YCbCr` | 亮度 + 色度（视频常用） |

LiteMark 选择 `Rgb(Rgb)` 是因为：
1. 最简单，直接得到三通道数据
2. 照片通常没有透明通道（HEIC 虽然有支持，但 iPhone 照片不带 Alpha）
3. 转换到 RGBA 可以统一后续处理逻辑

---

## 小结

| 概念 | 要点 |
|------|------|
| HEIC | 基于 HEIF 的容器格式，内部用 HEVC 编码 |
| ISOBMFF | ISO 基础媒体文件格式，Box 嵌套结构 |
| ftyp | 文件类型 Box，用于魔数检测 |
| HEVC | 高效视频编码，帧内模式压缩单图 |
| Stride | 每行实际字节数，可能包含 padding |
| RGB→RGBA | LiteMark 统一转换为 RGBA 处理 |
| WASM 限制 | libheif 不支持 WebAssembly 编译 |

---

## 延伸阅读

- [01-image-formats.md](01-image-formats.md) — 图像格式概览与对比
- [02-pixel-and-color.md](02-pixel-and-color.md) — RGBA 像素与颜色模型
