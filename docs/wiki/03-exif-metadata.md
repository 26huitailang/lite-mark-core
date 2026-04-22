# EXIF 元数据与摄影参数

> EXIF 是 LiteMark 的核心数据来源。理解 EXIF 的结构和字段含义，就能理解 LiteMark 如何从照片中提取拍摄参数。

---

## 1. EXIF 是什么

**EXIF（Exchangeable Image File Format）** 是一种嵌入在图像文件中的元数据标准，记录了照片的拍摄信息。

### 1.1 EXIF 存储在哪里

EXIF 数据嵌入在图像文件内部，通常位于文件的开头部分：

```
JPEG 文件结构：
┌─────────────┬─────────────┬─────────────┬─────────────┐
│  SOI 标记   │   APP1 段    │   图像数据   │   EOI 标记  │
│  (2 bytes)  │  (EXIF 在此) │  (压缩像素)  │  (2 bytes)  │
└─────────────┴─────────────┴─────────────┴─────────────┘
       ↑
   0xFFD8     0xFFE1 + "Exif\0\0"      0xFFD9
```

- **SOI** (Start Of Image): `0xFFD8`
- **APP1**: EXIF 数据存放的段，以 `0xFFE1` 开头，后跟 `"Exif\0\0"`
- **图像数据**: 实际的压缩像素
- **EOI** (End Of Image): `0xFFD9`

### 1.2 EXIF 的内部结构

EXIF 基于 **TIFF（Tagged Image File Format）** 格式，核心结构是 **IFD（Image File Directory）**：

```
TIFF Header (8 bytes)
┌────────────┬────────────┬────────────┐
│  字节序标识  │  魔数 42   │  IFD0 偏移  │
│  "II"或"MM" │  0x002A   │   4 bytes   │
└────────────┴────────────┴────────────┘

IFD0 (主图像目录)
┌────────────┬──────────────────────────┐
│  条目数量   │  条目1 │ 条目2 │ ... │ 条目N │
│  2 bytes   │  每个条目 12 bytes        │
└────────────┴──────────────────────────┘

每个 IFD 条目：
┌─────────┬─────────┬─────────┬─────────┐
│  Tag    │  Type   │ Count   │  Value  │
│ 2 bytes │ 2 bytes │ 4 bytes │ 4 bytes │
└─────────┴─────────┴─────────┴─────────┘
```

**Tag** 是字段标识符（如 `0x8827` = ISO），**Type** 是数据类型，**Value** 存放实际值或偏移地址。

---

## 2. 字节序（Endianness）

### 2.1 什么是字节序

字节序指多字节数据在内存中的存储顺序：

- **Little Endian (II)**：低位字节在前。如 `0x1234` 存储为 `34 12`
- **Big Endian (MM)**：高位字节在前。如 `0x1234` 存储为 `12 34`

### 2.2 为什么重要

不同相机品牌使用不同的字节序：
- 佳能 (Canon): Little Endian
- 尼康 (Nikon): Big Endian
- 索尼 (Sony): Little Endian

EXIF 解析器必须首先读取 TIFF Header 的字节序标识，然后按正确顺序解析后续数据。LiteMark 使用的 `kamadak-exif` 库会自动处理这一点。

---

## 3. 数据类型：有理数（Rational）

### 3.1 什么是有理数

EXIF 中许多摄影参数用**有理数（Rational）**存储：两个 32 位整数组成的分数 `num/denom`。

```
Rational 结构（8 bytes）：
┌─────────────────┬─────────────────┐
│   Numerator     │   Denominator   │
│    4 bytes      │    4 bytes      │
└─────────────────┴─────────────────┘
```

### 3.2 为什么用有理数

摄影参数通常是分数或小数，用有理数可以避免浮点精度丢失：

| 参数 | 存储值 | 实际值 |
|------|--------|--------|
| 光圈 | 14/5 | f/2.8 |
| 快门 | 1/125 | 1/125 秒 |
| 焦距 | 85/1 | 85mm |

LiteMark 中的解析代码：

```rust
fn extract_aperture(exif: &exif::Exif) -> Option<f64> {
    let field = exif.get_field(Tag::FNumber, In::PRIMARY)?;
    if let Value::Rational(rationals) = &field.value
        && let Some(rational) = rationals.first() {
            return Some(rational.num as f64 / rational.denom as f64);
        }
    None
}
```

这里 `rational.num` 是分子，`rational.denom` 是分母，相除得到实际的浮点值。

---

## 4. LiteMark 提取的 8 个 EXIF 字段

### 4.1 字段总览

| 字段 | EXIF Tag | Tag ID | 数据类型 | 示例 |
|------|----------|--------|----------|------|
| ISO | PhotographicSensitivity | 0x8827 | 整数 | `400` |
| 光圈 | FNumber | 0x829D | 有理数 | `f/2.8` |
| 快门 | ExposureTime | 0x829A | 有理数 | `1/125` |
| 焦距 | FocalLength | 0x920A | 有理数 | `50mm` |
| 相机 | Model | 0x0110 | ASCII | `Canon EOS R5` |
| 镜头 | LensModel | 0xA434 | ASCII | `RF 24-70mm F2.8` |
| 时间 | DateTimeOriginal | 0x9003 | ASCII | `2024:01:15 14:30:00` |
| 作者 | Artist | 0x013B | ASCII | `John Doe` |

### 4.2 ISO 感光度

ISO 表示相机传感器对光线的敏感程度。数值越高，感光越强，但噪点也越多。

```rust
fn extract_iso(exif: &exif::Exif) -> Option<u32> {
    let field = exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY)?;
    field.value.get_uint(0)
}
```

- `get_field(Tag::XXX, In::PRIMARY)` — 从主图像 IFD 中查找指定 Tag
- `get_uint(0)` — 获取第一个无符号整数值

### 4.3 光圈（F-Number）

光圈控制镜头进光孔径大小。数值越小，光圈越大，进光越多，背景虚化越强。

```
f/1.4  ← 光圈很大，背景虚化强
f/2.8
f/5.6
f/11
f/22  ← 光圈很小，景深大
```

LiteMark 显示格式：`f/{值}`，如 `f/2.8`。

### 4.4 快门速度

快门速度是传感器暴露在光线下的时间。

LiteMark 的格式化逻辑：

```rust
fn format_shutter_speed(exposure_time: f64) -> String {
    if exposure_time >= 1.0 {
        format!("{}s", exposure_time as u32)  // ≥1秒：显示 "2s", "5s"
    } else {
        let denominator = (1.0 / exposure_time).round() as u32;
        format!("1/{}", denominator)          // <1秒：显示 "1/125", "1/500"
    }
}
```

| EXIF 原始值 | 格式化结果 | 说明 |
|-------------|-----------|------|
| 0.001 | `1/1000` | 极快，冻结运动 |
| 0.008 | `1/125` | 日常手持安全速度 |
| 0.0166... | `1/60` | 手持临界值 |
| 1.0 | `1s` | 需要三脚架 |
| 2.5 | `2s` | 长曝光 |

### 4.5 焦距

焦距决定视角宽窄和透视效果。单位是毫米（mm）。

```rust
fn extract_focal_length(exif: &exif::Exif) -> Option<f64> {
    let field = exif.get_field(Tag::FocalLength, In::PRIMARY)?;
    if let Value::Rational(rationals) = &field.value
        && let Some(rational) = rationals.first() {
            return Some(rational.num as f64 / rational.denom as f64);
        }
    None
}
```

显示格式：`{值}mm`，如 `50mm`。

### 4.6 相机型号与镜头型号

这两个字段以 ASCII 字符串存储，直接读取即可：

```rust
fn extract_camera_model(exif: &exif::Exif) -> Option<String> {
    let field = exif.get_field(Tag::Model, In::PRIMARY)?;
    let value = field.display_value().to_string();
    Some(value.trim_matches('"').to_string())  // 去掉可能的引号
}
```

`display_value()` 会处理编码转换，返回人类可读的字符串。

### 4.7 拍摄时间

`DateTimeOriginal` 记录按下快门的时刻，格式固定为 `YYYY:MM:DD HH:MM:SS`。

### 4.8 作者

`Artist` 字段通常由相机设置或后期软件写入，记录摄影师名字。

---

## 5. EXIF 解析流程

LiteMark 的完整解析流程：

```
图像文件字节数据
       │
       ▼
┌──────────────┐
│ Cursor::new  │   ← 将字节数据包装为可读取的流
└──────────────┘
       │
       ▼
┌──────────────┐
│ Reader::read │   ← kamadak-exif 解析 TIFF/IFD 结构
│_from_container│
└──────────────┘
       │
       ▼
┌──────────────┐
│  遍历 IFD    │   ← 按 Tag 查找各字段
│  提取字段值   │
└──────────────┘
       │
       ▼
┌──────────────┐
│  格式化输出   │   ← 光圈加 "f/"、快门转分数、焦距加 "mm"
└──────────────┘
```

---

## 6. 错误处理与降级策略

### 6.1 无 EXIF 数据

```rust
let exif = match exifreader.read_from_container(&mut cursor) {
    Ok(exif) => exif,
    Err(_) => {
        // 没有 EXIF，返回空对象而不是报错
        return Ok(ExifData::new());
    }
};
```

### 6.2 部分字段缺失

某个字段缺失时，仅该字段为 `None`，其他字段正常：

```rust
let mut data = ExifData::new();
data.iso = extract_iso(&exif);        // Some(400) 或 None
data.aperture = extract_aperture(&exif); // Some(2.8) 或 None
// ... 其他字段
```

### 6.3 模板中的处理

模板变量未替换时会被跳过：

```rust
if substituted_text.contains('{') && substituted_text.contains('}') {
    continue;  // 跳过未替换的占位符
}
```

---

## 7. 常见问题

**Q: 为什么我的截图提取不到 EXIF？**

截图软件通常不会写入 EXIF 数据。这不是 bug，是正常行为。可以用 `--author` 参数手动指定信息。

**Q: 社交媒体下载的照片为什么缺少参数？**

Instagram、微信等平台会在上传时**剥离 EXIF 数据**，保护隐私但也丢失了拍摄信息。

**Q: 为什么老相机的照片缺少镜头信息？**

`LensModel` (0xA434) 是相对较新的 Tag，部分老相机固件不支持写入此字段。

---

## 小结

| 概念 | 要点 |
|------|------|
| EXIF | 嵌入图像的元数据标准，基于 TIFF 格式 |
| IFD | 图像文件目录，由 Tag-Value 条目组成 |
| 字节序 | Little Endian (II) / Big Endian (MM) |
| 有理数 | `num/denom` 分数格式，避免浮点精度问题 |
| 降级策略 | 无 EXIF 返回空对象，部分缺失仅影响对应字段 |

---

## 延伸阅读

- [01-image-formats.md](01-image-formats.md) — 了解不同图像格式对 EXIF 的支持差异
- [06-resolution-scaling.md](06-resolution-scaling.md) — 了解照片分辨率与水印布局的关系
