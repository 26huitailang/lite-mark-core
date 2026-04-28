# LiteMark 图像处理知识 Wiki

> 专为图像处理新手打造的 LiteMark 项目技术知识库。从像素到字体渲染，从 EXIF 到图像合成，帮助你理解代码背后的原理。

---

## 快速导航

### 📚 推荐学习路径

**路径一：由浅入深（推荐新手）**

1. [02-像素、颜色模型与 Alpha 通道](02-pixel-and-color.md) — 最基础的概念，所有内容的起点
2. [03-EXIF 元数据与摄影参数](03-exif-metadata.md) — 理解 LiteMark 从哪里获取数据
3. [01-图像格式与编解码](01-image-formats.md) — 了解图像文件的存储方式
4. [07-HEIC/HEIF 格式深度解析](07-heic-decoding.md) — iPhone 照片的格式秘密
5. [04-矢量字体与文字渲染](04-font-rendering.md) — 文字如何变成像素
6. [05-图像合成：Alpha 混合与插值](05-image-compositing.md) — 水印是怎么"贴"上去的
7. [06-分辨率、缩放与响应式设计](06-resolution-scaling.md) — 为什么你的水印在不同照片上看起来不一样

**路径二：按需查阅**

| 你想了解... | 阅读 |
|------------|------|
| 图像文件是怎么存储的 | [01-图像格式](01-image-formats.md) |
| 颜色、透明度、混合 | [02-像素与颜色](02-pixel-and-color.md) |
| 照片中的参数是怎么读出来的 | [03-EXIF 元数据](03-exif-metadata.md) |
| 文字为什么能缩放不模糊 | [04-字体渲染](04-font-rendering.md) |
| Logo 缩放为什么平滑 | [05-图像合成](05-image-compositing.md) |
| 高分辨率图片文字太小怎么办 | [06-分辨率缩放](06-resolution-scaling.md) |
| iPhone 的 HEIC 照片怎么解码 | [07-HEIC 解码](07-heic-decoding.md) |

---

## 文档索引

### [01 - 图像格式与编解码](01-image-formats.md)
- 位图 vs 矢量图
- JPEG 有损压缩原理与质量参数
- PNG 无损压缩与透明度
- WebP 双模压缩
- 文件魔数（Magic Number）检测
- Rust `image` 库内存模型：`DynamicImage`、`RgbaImage`

### [02 - 像素、颜色模型与 Alpha 通道](02-pixel-and-color.md)
- 像素：数字图像的最小单位
- RGB 三原色叠加模型
- Hex 颜色表示法（`#RRGGBB` / `#RRGGBBAA`）
- Alpha 通道与透明度
- Alpha 混合数学公式
- 对比度与可读性设计

### [03 - EXIF 元数据与摄影参数](03-exif-metadata.md)
- EXIF 标准与 TIFF/IFD 结构
- 字节序（Big Endian / Little Endian）
- 有理数（Rational）格式
- 8 个核心字段：ISO、光圈、快门、焦距、相机、镜头、时间、作者
- 快门速度格式化逻辑
- 降级策略与错误处理

### [04 - 矢量字体与文字渲染](04-font-rendering.md)
- 位图字体 vs 矢量字体
- TrueType / OpenType 结构
- Glyph、Baseline、Advance 核心概念
- `ab_glyph` 渲染流程
- 栅格化与抗锯齿
- 字重（Font Weight）与字体嵌入

### [05 - 图像合成：Alpha 混合与插值](05-image-compositing.md)
- 画布扩展技术
- 线性渐变渲染
- 双线性插值（Bilinear Interpolation）
- 圆角矩形的数学实现
- 四种渲染模式的合成差异

### [06 - 分辨率、缩放与响应式设计](06-resolution-scaling.md)
- 分辨率与像素
- 短边（Short Edge）基准
- 比例系统设计
- 硬编码上限问题与改进方案
- 极端尺寸适配
- 性能与分辨率的权衡

### [07 - HEIC/HEIF 格式深度解析](07-heic-decoding.md)
- HEIC 与 HEVC 的关系
- ISOBMFF 容器与 Box 结构
- `ftyp` Box 与魔数检测
- `libheif-rs` 解码流程
- Stride（步幅）概念
- RGB → RGBA 转换
- WASM 平台限制

---

## 术语表

| 术语 | 英文 | 解释 |
|------|------|------|
| 像素 | Pixel | 数字图像的最小单位 |
| 分辨率 | Resolution | 图像的宽×高像素数 |
| 颜色通道 | Color Channel | RGBA 中的 R、G、B、A 各为一个通道 |
| Alpha 混合 | Alpha Blending | 前景色按透明度叠加到背景 |
| EXIF | Exchangeable Image File Format | 嵌入图像的元数据标准 |
| IFD | Image File Directory | EXIF/TIFF 中的数据目录 |
| 有理数 | Rational | `num/denom` 分数格式 |
| 矢量字体 | Vector Font | 数学曲线描述的字体，可任意缩放 |
| Glyph | Glyph | 字体中的字形单元 |
| 栅格化 | Rasterization | 矢量图形转为像素的过程 |
| 抗锯齿 | Anti-aliasing | 用灰度模拟平滑边缘 |
| 双线性插值 | Bilinear Interpolation | 4 像素加权平均的缩放算法 |
| 魔数 | Magic Number | 文件开头的类型标识字节 |
| Stride | Stride | 图像每行实际占用的字节数 |
| 比例系统 | Ratio System | 基于相对比例而非绝对像素的尺寸设计 |

---

## 与项目文档的关系

| Wiki 文档 | 对应的 LiteMark 代码 | 对应的项目文档 |
|-----------|---------------------|--------------|
| 02-像素与颜色 | `renderer/color.rs` `parse_color`<br>`renderer/draw.rs` `blend_pixel` | `docs/DESIGN_SYSTEM.md` |
| 03-EXIF | `exif.rs` | `examples/exif_extraction.md` |
| 01-图像格式 | `image_io.rs` | — |
| 07-HEIC | `image_io.rs` `decode_heic_from_bytes` | — |
| 04-字体渲染 | `renderer/text.rs` `render_text_simple` | `examples/chinese_font_guide.md` |
| 05-图像合成 | `renderer/draw.rs`, `renderer/logo.rs` | `litemark-core/ARCHITECTURE.md` |
| 06-分辨率 | `renderer/mod.rs` 尺寸计算 | `docs/resolution_scaling_analysis.md` |

---

*Wiki 版本: v1.0.0*
*最后更新: 2026-04-22*
