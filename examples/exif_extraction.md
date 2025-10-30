# EXIF 数据提取示例

LiteMark 使用 `kamadak-exif` 库自动从照片中提取拍摄参数。

## 支持的 EXIF 字段

| 字段 | EXIF 标签 | 格式示例 | 说明 |
|------|-----------|----------|------|
| ISO | PhotographicSensitivity | `100`, `400`, `3200` | ISO 感光度 |
| 光圈 | FNumber | `f/2.8`, `f/5.6` | 光圈值 |
| 快门 | ExposureTime | `1/125`, `1/500`, `2s` | 曝光时间 |
| 焦距 | FocalLength | `50mm`, `85mm` | 焦距 |
| 相机 | Model | `Canon EOS R5` | 相机型号 |
| 镜头 | LensModel | `RF 24-70mm F2.8` | 镜头型号 |
| 时间 | DateTimeOriginal | `2024-01-15 14:30:00` | 拍摄时间 |
| 作者 | Artist | `Photographer Name` | 摄影师 |

## 基本用法

### 1. 使用真实照片的 EXIF 数据

```bash
# 从照片中自动提取 EXIF 参数
litemark add -i my_photo.jpg -t classic -o watermarked.jpg
```

输出将显示:
```
Extracting EXIF data from: my_photo.jpg
Successfully extracted EXIF data:
  ISO: Some(400)
  Aperture: Some(2.8)
  Shutter: Some("1/125")
  Focal: Some(50.0)
  Camera: Some("Canon EOS R5")
  ...
```

### 2. 覆盖作者信息

即使照片中有作者信息，也可以通过命令行参数覆盖：

```bash
litemark add -i photo.jpg -t classic -o output.jpg --author "张三"
```

### 3. 处理无 EXIF 数据的图片

对于截图、编辑后的图片或其他没有 EXIF 数据的图片：

```bash
litemark add -i screenshot.png -t classic -o output.png
```

LiteMark 会优雅地处理这种情况：
```
Warning: No EXIF data found in image: screenshot.png
  Error: ...
  This image may not contain EXIF data or format is unsupported
  Returning empty EXIF data
```

水印中的参数字段将为空或显示占位符。

## 支持的图片格式

| 格式 | 扩展名 | EXIF 支持 | 说明 |
|------|--------|-----------|------|
| JPEG | `.jpg`, `.jpeg` | ✅ 完全支持 | 最常见的格式 |
| PNG | `.png` | ✅ 支持 | 部分相机支持 PNG EXIF |
| TIFF | `.tif`, `.tiff` | ✅ 支持 | 专业相机格式 |
| HEIF/HEIC | `.heic`, `.heif` | 🚧 计划中 | iOS 默认格式 |
| RAW | `.cr2`, `.nef`, etc. | 🚧 计划中 | 相机原始格式 |

## 快门速度格式化

LiteMark 自动将 EXIF 中的曝光时间格式化为可读形式：

| EXIF 原始值 | 显示格式 |
|-------------|----------|
| 0.001 | `1/1000` |
| 0.008 | `1/125` |
| 0.0166... | `1/60` |
| 1.0 | `1s` |
| 2.5 | `2s` |

## 常见问题

### Q: 为什么我的照片提取不到 EXIF 数据？

**可能原因：**
1. 图片是截图或经过编辑（某些编辑软件会删除 EXIF）
2. 图片格式不支持 EXIF（如某些 PNG）
3. 照片在上传社交媒体时被平台删除了 EXIF

**解决方案：**
- 使用相机直出的原始 JPEG 文件
- 检查图片编辑软件的"保留元数据"选项
- 使用 `--author` 等参数手动指定信息

### Q: 为什么某些字段是空的？

**原因：**
不是所有相机都会记录所有 EXIF 字段。例如：
- 部分手机不记录镜头型号
- 老相机可能不记录作者信息
- 某些情况下焦距可能缺失

**行为：**
LiteMark 会提取所有可用的字段，缺失的字段在模板中会被跳过或显示为空。

### Q: 如何查看照片的原始 EXIF 数据？

使用命令行工具查看：

```bash
# macOS
mdls my_photo.jpg | grep kMDItem

# Linux (需要安装 exiftool)
exiftool my_photo.jpg

# 或使用 LiteMark（查看日志输出）
litemark add -i photo.jpg -t classic -o output.jpg
```

## 技术细节

### EXIF 解析流程

1. **打开图片文件** → `std::fs::File::open()`
2. **创建缓冲读取器** → `BufReader::new()`
3. **解析 EXIF 容器** → `exif::Reader::read_from_container()`
4. **提取各个字段** → `exif.get_field(Tag::XXX, In::PRIMARY)`
5. **格式化输出** → 应用特定格式规则

### 错误处理策略

| 错误类型 | 处理方式 |
|----------|----------|
| 文件不存在 | 返回错误，终止处理 |
| 无法读取文件 | 返回错误，终止处理 |
| 无 EXIF 数据 | 警告，返回空 ExifData |
| 部分字段缺失 | 仅该字段为 None，其他正常 |
| EXIF 数据损坏 | 尽可能解析，返回部分数据 |

## 示例输出

### 包含完整 EXIF 的照片

```
Extracting EXIF data from: DSC_1234.jpg
Successfully extracted EXIF data:
  ISO: Some(400)
  Aperture: Some(2.8)
  Shutter: Some("1/125")
  Focal: Some(85.0)
  Camera: Some("Nikon D850")
  Lens: Some("AF-S NIKKOR 85mm f/1.4G")
  DateTime: Some("2024-01-15 14:30:00")
  Author: Some("John Doe")
```

### 部分 EXIF 缺失的照片

```
Extracting EXIF data from: phone_photo.jpg
Successfully extracted EXIF data:
  ISO: Some(100)
  Aperture: Some(1.8)
  Shutter: Some("1/60")
  Focal: Some(4.2)
  Camera: Some("iPhone 14 Pro")
  Lens: None
  DateTime: Some("2024-01-15 10:00:00")
  Author: None
```

### 无 EXIF 数据的图片

```
Warning: No EXIF data found in image: screenshot.png
  Error: ...
  This image may not contain EXIF data or format is unsupported
  Returning empty EXIF data
Successfully extracted EXIF data:
  ISO: None
  Aperture: None
  Shutter: None
  Focal: None
  Camera: None
  Lens: None
  DateTime: None
  Author: None
```

## 进阶用法

### 批量处理时查看 EXIF 信息

```bash
litemark batch -i ./photos/ -t classic -o ./output/
```

每张照片的 EXIF 提取信息都会显示在控制台中。

### 结合自定义作者名

```bash
# 即使照片有作者信息，也会被覆盖
litemark add -i photo.jpg -o out.jpg --author "我的签名"
```

## 相关资源

- [kamadak-exif 文档](https://docs.rs/kamadak-exif/)
- [EXIF 标准规范](https://www.exif.org/)
- [LiteMark 架构文档](../docs/ARCHITECTURE.md)
