# 矢量字体与文字渲染

> LiteMark 的水印是文字，文字的清晰渲染直接影响水印质量。理解字体渲染的原理，能帮助你调试字体相关问题（如中文显示、字重选择）。

---

## 1. 位图字体 vs 矢量字体

### 1.1 两种字体技术

```
位图字体（Bitmap Font）           矢量字体（Vector Font）
┌────────┬────────┐              ┌────────────────────────┐
│  A     │  预渲染  │              │                        │
│ ■■□    │  的像素  │              │     /‾‾‾‾‾‾\           │
│ ■ ■    │  矩阵    │              │    /        \          │
│ ■■■    │         │              │   │    A     │         │
│ ■ ■    │  每个字号 │              │   │   / \    │         │
│        │  一套图   │              │    \  ‾‾‾  /          │
└────────┴────────┘              │     \______/           │
                                  │                        │
  放大后锯齿严重                   │   由数学曲线描述轮廓     │
                                  │   任意缩放都清晰         │
                                  └────────────────────────┘
```

| 特性 | 位图字体 | 矢量字体 |
|------|---------|---------|
| 存储 | 像素矩阵 | 轮廓曲线（贝塞尔） |
| 缩放 | 固定大小，放大模糊 | 任意缩放清晰 |
| 渲染开销 | 低（直接贴图） | 高（需栅格化） |
| 文件大小 | 每个字号一套数据 | 一套数据覆盖所有字号 |
| 现代使用 | 嵌入式设备、游戏 | 操作系统、浏览器、LiteMark |

LiteMark 使用 **矢量字体**（思源黑体嵌入 + 用户自定义字体）。

---

## 2. TrueType / OpenType 字体格式

### 2.1 字体文件里存了什么

```
字体文件（.ttf / .otf）结构：
┌─────────────────────────────────────────┐
│  文件头 (Header)                         │
│  ├── 表数量、搜索范围等                    │
├─────────────────────────────────────────┤
│  表目录 (Table Directory)                │
│  ├── cmap: 字符 → Glyph ID 映射           │
│  ├── glyf: Glyph 轮廓数据                  │
│  ├── head: 全局字体信息                    │
│  ├── hhea: 水平布局参数                    │
│  ├── hmtx: 水平度量（advance width）       │
│  └── ... 其他表                           │
├─────────────────────────────────────────┤
│  各表数据                                │
└─────────────────────────────────────────┘
```

### 2.2 关键概念

#### Glyph（字形）

**Glyph** 是字体中一个可渲染的图形单元。注意：Glyph ≠ 字符。一个字符可能对应多个 Glyph（如连字 "fi"），一个 Glyph 也可能被多个字符共用。

```
字符 'A' → Glyph ID 36 → 轮廓数据

轮廓数据是一组点定义的路径：
      (100, 300)
           *        ← 曲线控制点
          / \
    (0,0)*   *(200,0)
        |\ /|
        | * |      ← (100, 150)
        |/ \|
        *   *
      (40,0) (160,0)
```

#### Baseline（基线）

**基线**是文字排假想的一条水平线，大部分字母"坐"在上面。

```
         x-height
        ┌────────┐
   ▲    │        │
   │    │   x    │
Ascent├─ ─ ─ ─ ─┤
   │   │   A    │
───┼───┼────────┤ ← Baseline（基线）
   │   │   g    │
Descent├── ─ ─ ─┤
   │   │   p    │
   ▼   │   y    │
        └────────┘
```

- **Ascent**：基线到最高点的距离
- **Descent**：基线到最低点的距离（通常为负值）
- **x-height**：小写字母 'x' 的高度

#### Advance（步进/字距）

**Advance** 是光标从当前 Glyph 移动到下一个 Glyph 的水平距离。

```
"AB" 的渲染：

┌─────────┬─────────┐
│    A    │    B    │
│         │         │
│    ▲    │    ▲    │
└────┼────┴────┼────┘
     │         │
   起点      起点 + A.advance
```

---

## 3. 字体渲染流程

### 3.1 从字符到像素的完整流程

```
字符 'A'
   │
   ▼
cmap 表查找
   │
   ▼
Glyph ID = 36
   │
   ▼
glyf 表读取轮廓数据
   │
   ▼
轮廓数据（贝塞尔曲线控制点）
   │
   ▼
┌─────────────────┐
│   栅格化 (Rasterize) │  ← 将矢量轮廓转为像素覆盖度
│   ab_glyph 内部完成   │
└─────────────────┘
   │
   ▼
每个像素的覆盖度值 v (0.0 ~ 1.0)
   │
   ▼
Alpha 混合到目标图像
```

### 3.2 栅格化与抗锯齿

栅格化是将连续曲线映射到离散像素网格的过程。核心问题：**边缘像素只有部分被覆盖**。

```
理想情况（矢量）        无抗锯齿              有抗锯齿
   ┌──┐                ┌──┐                 ┌──┐
  /    \               ██░░                 ▓▓░░
 │      │              ██░░                 ▓▓░░
  \    /               ░░██                 ░░▓▓
   └──┘                ░░██                 ░░▓▓

                        锯齿明显              边缘平滑
```

**抗锯齿（Anti-aliasing）** 的原理：用灰度（覆盖度 `v`）代替二值（全黑/全白）。`v = 0.5` 的像素渲染为 50% 亮度，视觉上形成平滑过渡。

---

## 4. LiteMark 中的字体渲染

### 4.1 ab_glyph 渲染代码解析

LiteMark 使用 `ab_glyph` 库进行字体渲染：

```rust
fn render_text_simple(
    &self,
    image: &mut RgbaImage,
    text: &str,
    x: i32,
    y: i32,
    font_size: u32,
    color: Rgba<u8>,
    weight: Option<&FontWeight>,
) {
    let font = self.select_font(weight);
    let scale = PxScale::from(font_size as f32);
    let scaled_font = font.as_scaled(scale);

    let mut glyph_x = x as f32;
    // 基线位置 = y + Ascent
    let baseline_y = y as f32 + scaled_font.ascent();

    for c in text.chars() {
        let glyph_id = font.glyph_id(c);  // 字符 → Glyph ID
        let glyph = Glyph {
            id: glyph_id,
            scale,
            position: Point {
                x: glyph_x,
                y: baseline_y,
            },
        };

        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // draw: 对每个像素调用闭包，v 是覆盖度 (0.0~1.0)
            outlined.draw(|px, py, v| {
                let px = px as i32 + bounds.min.x as i32;
                let py = py as i32 + bounds.min.y as i32;

                if px >= 0 && py >= 0
                    && px < image.width() as i32
                    && py < image.height() as i32
                    && v > 0.01
                {
                    let px_u32 = px as u32;
                    let py_u32 = py as u32;
                    let bg = image.get_pixel(px_u32, py_u32);

                    // Alpha 混合：v 就是字形的 Alpha
                    let r = ((color[0] as f32 * v) + (bg[0] as f32 * (1.0 - v))) as u8;
                    let g = ((color[1] as f32 * v) + (bg[1] as f32 * (1.0 - v))) as u8;
                    let b = ((color[2] as f32 * v) + (bg[2] as f32 * (1.0 - v))) as u8;

                    image.put_pixel(px_u32, py_u32, Rgba([r, g, b, 255]));
                }
            });
        }

        // 光标前进
        glyph_x += scaled_font.h_advance(glyph_id);
    }
}
```

### 4.2 关键步骤拆解

**Step 1: 创建缩放字体**

```rust
let scale = PxScale::from(font_size as f32);
let scaled_font = font.as_scaled(scale);
```

`PxScale` 将字体缩放到目标像素大小。字体文件中的轮廓是"单位坐标"（通常为 1000 或 2048 units per em），需要缩放到实际像素。

**Step 2: 获取 Glyph ID**

```rust
let glyph_id = font.glyph_id(c);
```

通过字体的 `cmap` 表，将 Unicode 字符映射到内部 Glyph ID。

**Step 3: 构建 Glyph**

```rust
let glyph = Glyph {
    id: glyph_id,
    scale,
    position: Point { x: glyph_x, y: baseline_y },
};
```

Glyph 结构包含：要渲染哪个字形、缩放比例、在画布上的位置。

**Step 4: 轮廓化与栅格化**

```rust
if let Some(outlined) = scaled_font.outline_glyph(glyph) {
    outlined.draw(|px, py, v| { /* ... */ });
}
```

`outline_glyph` 将矢量轮廓缩放并栅格化，`draw` 遍历所有被影响的像素，提供覆盖度 `v`。

**Step 5: Alpha 混合**

覆盖度 `v` 直接作为 Alpha 值与背景混合，形成抗锯齿边缘。

### 4.3 字重（Font Weight）

LiteMark 支持三种字重：

| 字重 | 用途 | 回退策略 |
|------|------|----------|
| **Bold** | 摄影师名、主标题 | 无 Bold 字体时回退 Regular |
| **Normal** | 相机型号、参数行 | 默认字重 |
| **Light** | 日期、极简风格的装饰文字 | 无 Light 时回退 Normal |

字重选择逻辑：

```rust
fn select_font(&self, weight: Option<&FontWeight>) -> &FontRef<'static> {
    match weight {
        Some(FontWeight::Bold) => self.fonts.bold.as_ref().unwrap_or(&self.fonts.regular),
        _ => &self.fonts.regular,
    }
}
```

**注意**：字重需要独立的字体文件支持。如果只有 Regular 字体文件，设置 Bold 也会回退到 Regular。

---

## 5. 字体嵌入

### 5.1 编译时嵌入

LiteMark 将思源黑体嵌入到二进制中：

```rust
fn load_default_font() -> Result<FontRef<'static>, CoreError> {
    let font_data = include_bytes!("../../assets/fonts/SourceHanSansCN-Regular.otf");
    FontRef::try_from_slice(font_data)?;
}
```

`include_bytes!` 在编译时将文件内容作为 `&'static [u8]` 嵌入到可执行文件中。运行时无需外部字体文件。

### 5.2 运行时自定义字体

用户可以提供自定义字体：

```rust
fn parse_font_data(data: &[u8]) -> Result<FontRef<'static>, CoreError> {
    let leaked: &'static [u8] = Box::leak(data.to_vec().into_boxed_slice());
    FontRef::try_from_slice(leaked)?;
}
```

这里用了 `Box::leak` 将堆内存转换为 `'static` 引用。因为 `FontRef` 需要引用字体数据的字节切片，而渲染器的生命周期是长期的，所以将字体数据"泄漏"到程序生命周期中。

---

## 6. 文字宽度计算

布局需要知道文字占多宽：

```rust
fn text_width(&self, text: &str, font_size: u32, weight: Option<&FontWeight>) -> f32 {
    let font = self.select_font(weight);
    let scale = PxScale::from(font_size as f32);
    let scaled_font = font.as_scaled(scale);

    text.chars()
        .map(|c| scaled_font.h_advance(font.glyph_id(c)))
        .sum()
}
```

对每个字符，累加其 `h_advance`（水平步进），得到总宽度。这用于：
- **右对齐**：计算文字右边缘位置
- **Overlay 模式**：计算半透明背景框的宽度

---

## 小结

| 概念 | 要点 |
|------|------|
| 矢量字体 | 数学曲线描述轮廓，任意缩放清晰 |
| Glyph | 字体的最小渲染单元 |
| Baseline | 文字排版基线，Ascent/Descent 定义上下边界 |
| Advance | 水平步进，决定字符间距 |
| 栅格化 | 矢量轮廓 → 像素覆盖度 |
| 抗锯齿 | 用灰度值模拟平滑边缘 |
| 字重 | Bold/Normal/Light，需独立字体文件 |
| 字体嵌入 | `include_bytes!` 编译时嵌入 |

---

## 延伸阅读

- [02-pixel-and-color.md](02-pixel-and-color.md) — Alpha 混合的数学原理
- [05-image-compositing.md](05-image-compositing.md) — 图像合成中的文字叠加
- [06-resolution-scaling.md](06-resolution-scaling.md) — 字号与分辨率的适配关系
