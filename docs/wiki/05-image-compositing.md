# 图像合成：Alpha 混合与插值

> 水印本质上是在原始照片上"叠加"新内容。LiteMark 用到的图像合成技术包括画布扩展、渐变渲染、双线性插值和圆角矩形绘制。

---

## 1. 画布扩展

### 1.1 什么是画布扩展

BottomFrame、GradientFrame、Minimal 三种模式都需要在照片**下方添加边框**。这涉及扩展图像的画布：

```
扩展前：                扩展后：
┌──────────────┐       ┌──────────────┐
│              │       │              │
│   原始照片    │  →    │   原始照片    │
│   800×600    │       │   800×600    │
│              │       │              │
└──────────────┘       ├──────────────┤
                       │   底部边框    │
                       │   800×100    │
                       └──────────────┘
                              ↓
                         总尺寸 800×700
```

### 1.2 LiteMark 的实现

```rust
let original_width = image.width();
let original_height = image.height();
let short_edge = original_width.min(original_height) as f32;

// 边框高度 = 短边 × 比例（默认 0.10）
let frame_height_ratio = template.frame_height_ratio.clamp(0.05, 0.20);
let calculated_frame_height = (short_edge * frame_height_ratio) as u32;
let bottom_frame_height = calculated_frame_height.max(80); // 最低 80px

// 新画布尺寸
let new_width = original_width;
let new_height = original_height + bottom_frame_height;

// 创建新图像
let mut frame_image = RgbaImage::new(new_width, new_height);

// 将原图复制到顶部
let original_rgba = image.to_rgba8();
for y in 0..original_height {
    for x in 0..original_width {
        frame_image.put_pixel(x, y, *original_rgba.get_pixel(x, y));
    }
}
```

**注意**：这里用逐像素复制而非直接内存拷贝，因为 `RgbaImage::new` 创建的是新分配的内存，需要将原图数据逐个写入。

---

## 2. 渐变渲染

### 2.1 线性渐变原理

GradientFrame 模式从照片底部向上产生一个白色渐变过渡：

```
渐变效果示意：

顶部（y=0）          中间               底部（y=height）
┌─────────┐        ┌─────────┐        ┌─────────┐
│ ░░░░░░░ │   →    │ ▒▒▒▒▒▒▒ │   →    │ ███████ │
│ ░░░░░░░ │        │ ▒▒▒▒▒▒▒ │        │ ███████ │
│ ░░░░░░░ │        │ ▒▒▒▒▒▒▒ │        │ ███████ │
└─────────┘        └─────────┘        └─────────┘

Alpha: 25% (64)    Alpha: 50% (160)    Alpha: 100% (255)
```

### 2.2 数学实现

```rust
fn render_gradient_background(
    &self,
    image: &mut RgbaImage,
    frame_y: u32,        // 边框起始 Y 坐标
    width: u32,
    height: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    for y in 0..height {
        // progress: 0.0（顶部）→ 1.0（底部）
        let progress = y as f32 / height as f32;

        // Alpha: 64（25%）→ 255（100%），线性插值
        let alpha = (64.0 + 191.0 * progress) as u8;
        let overlay = Rgba([255, 255, 255, alpha]);

        for x in 0..width {
            let original = *image.get_pixel(x, frame_y + y);
            let blended = Self::blend_pixel(original, overlay);
            image.put_pixel(x, frame_y + y, blended);
        }
    }
    Ok(())
}
```

**公式推导**：

```
目标：Alpha 从 64 线性增长到 255

alpha(y) = start + (end - start) × progress
         = 64 + (255 - 64) × (y / height)
         = 64 + 191 × progress
```

### 2.3 为什么用渐变

渐变过渡比硬边界更柔和，视觉上不会割裂照片与边框：

```
硬边界（BottomFrame）：          渐变过渡（GradientFrame）：
┌──────────────┐                ┌──────────────┐
│              │                │              │
│    照片      │                │    照片      │
│              │                │░░░░░░░░░░░░░░│ ← 柔和过渡
├──────────────┤                │░░░░░░░░░░░░░░│
│    边框      │                │░░░░░░░░░░░░░░│
└──────────────┘                └──────────────┘
    突兀的分割线                      自然的融合
```

---

## 3. 双线性插值（Bilinear Interpolation）

### 3.1 为什么需要插值

Logo 渲染时，原始 Logo 图片尺寸（如 400×200）与目标显示尺寸（如 100×50）不一致，需要**缩放**。直接取最近像素（Nearest Neighbor）会产生锯齿：

```
原图（4×4）：           最近邻缩放（2×）：       双线性缩放（2×）：
┌──┬──┬──┬──┐         ┌────┬────┬────┬────┐    ┌────┬────┬────┬────┐
│A │B │C │D │    →    │ A  │ A  │ B  │ B  │    │ A  │AB  │ BC │ C  │
├──┼──┼──┼──┤         ├────┼────┼────┼────┤    ├────┼────┼────┼────┤
│E │F │G │H │         │ A  │ A  │ B  │ B  │    │AE  │... │... │ CG │
├──┼──┼──┼──┤         ├────┼────┼────┼────┤    ├────┼────┼────┼────┤
│I │J │K │L │         │ E  │ E  │ F  │ F  │    │... │... │... │... │
├──┼──┼──┼──┤         ├────┼────┼────┼────┤    ├────┼────┼────┼────┤
│M │N │O │P │         │ E  │ E  │ F  │ F  │    │ I  │IJ  │ JK │ K  │
└──┴──┴──┴──┘         └────┴────┴────┴────┘    └────┴────┴────┴────┘

                         块状、锯齿               平滑、自然
```

### 3.2 双线性插值原理

双线性插值用目标像素周围的 **4 个源像素** 加权平均：

```
源图像坐标：              目标像素映射回源图像：

p00 ────── p10           p00 ───┬─── p10
 │          │              │    ●    │   ← 目标像素对应点
 │          │              │  (sx,sy)│
p01 ────── p11              ├───┼────┤
                           p01 ───┴─── p11

目标像素颜色 = p00×(1-fx)×(1-fy)
            + p10×fx×(1-fy)
            + p01×(1-fx)×fy
            + p11×fx×fy

其中 fx = sx 的小数部分, fy = sy 的小数部分
```

### 3.3 LiteMark 中的实现

```rust
for y in 0..scaled_h {
    for x in 0..scaled_w {
        // 目标像素 (x,y) 映射回源图像的浮点坐标
        let src_xf = (x as f32 / scaled_w as f32) * (logo_w - 1) as f32;
        let src_yf = (y as f32 / scaled_h as f32) * (logo_h - 1) as f32;

        // 取周围 4 个像素的整数坐标
        let src_x0 = src_xf as u32;
        let src_y0 = src_yf as u32;
        let src_x1 = (src_x0 + 1).min(logo_w - 1);
        let src_y1 = (src_y0 + 1).min(logo_h - 1);

        // 小数部分（权重）
        let fx = src_xf - src_x0 as f32;
        let fy = src_yf - src_y0 as f32;

        // 读取 4 个邻近像素
        let p00 = logo_rgba.get_pixel(src_x0, src_y0);
        let p10 = logo_rgba.get_pixel(src_x1, src_y0);
        let p01 = logo_rgba.get_pixel(src_x0, src_y1);
        let p11 = logo_rgba.get_pixel(src_x1, src_y1);

        // 对每个通道做双线性插值
        let sample = |idx: usize| {
            let v00 = p00[idx] as f32;
            let v10 = p10[idx] as f32;
            let v01 = p01[idx] as f32;
            let v11 = p11[idx] as f32;

            // 先在 X 方向插值
            let v0 = v00 * (1.0 - fx) + v10 * fx;
            let v1 = v01 * (1.0 - fx) + v11 * fx;

            // 再在 Y 方向插值
            (v0 * (1.0 - fy) + v1 * fy) as u8
        };

        let r = sample(0);
        let g = sample(1);
        let b = sample(2);
        let alpha = sample(3);

        // Alpha 混合到目标位置
        // ...
    }
}
```

### 3.4 插值 + Alpha 混合

Logo 缩放后还要与背景混合：

```
Step 1: 双线性插值得到 Logo 像素颜色 (r, g, b, alpha)
Step 2: 用 alpha 作为透明度与背景做 Alpha 混合
Step 3: 写入目标图像
```

---

## 4. 圆角矩形

### 4.1 Overlay 模式的背景

Overlay 模式在照片右下角绘制一个**圆角矩形**作为文字背景：

```
┌─────────────────────────────┐
│                             │
│         原始照片             │
│                   ┌───────┐ │
│                   │ 圆角  │ │  ← 圆角矩形背景
│                   │ 文字  │ │
│                   └───────┘ │
└─────────────────────────────┘
```

### 4.2 圆角的数学判断

圆角矩形 = 矩形主体 + 四个 1/4 圆角。判断一个像素是否在圆角矩形内：

```rust
fn render_rounded_rect(
    &self,
    image: &mut RgbaImage,
    x: u32, y: u32,           // 左上角
    width: u32, height: u32,  // 尺寸
    radius: u32,              // 圆角半径
    color: Rgba<u8>,
) {
    let r = radius.min(width / 2).min(height / 2);
    let r_sq = (r * r) as f32;

    for dy in 0..height {
        for dx in 0..width {
            let px = x + dx;
            let py = y + dy;

            let mut inside = true;

            // 判断是否在四个圆角区域
            if dx < r && dy < r {
                // 左上角：到圆角圆心的距离 ≤ r
                let cx = r - dx;
                let cy = r - dy;
                if (cx * cx + cy * cy) as f32 > r_sq {
                    inside = false;
                }
            } else if dx >= width - r && dy < r {
                // 右上角
                let cx = dx - (width - r);
                let cy = r - dy;
                if (cx * cx + cy * cy) as f32 > r_sq {
                    inside = false;
                }
            }
            // ... 左下角、右下角同理

            if inside {
                let bg = image.get_pixel(px, py);
                let blended = Self::blend_pixel(*bg, color);
                image.put_pixel(px, py, blended);
            }
        }
    }
}
```

### 4.3 圆角判断原理

```
圆角矩形展开：

        顶部边
    ┌────────────┐
   ╱              ╲   ← 左上圆角：圆心在 (r, r)
  │    主体区域     │
  │                │
   ╲              ╱   ← 左下圆角：圆心在 (r, h-r)
    └────────────┘

判断点 (dx, dy) 是否在左上圆角内：
- 如果 dx ≥ r 且 dy ≥ r：在矩形主体，一定在内
- 如果 dx < r 且 dy < r：在左上区域，计算到圆心 (r, r) 的距离
  - 距离 ≤ r：在内（圆弧上或内部）
  - 距离 > r：在外（圆弧外部，要裁掉）
```

---

## 5. 各渲染模式合成总结

| 模式 | 画布扩展 | 背景类型 | 核心合成技术 |
|------|---------|---------|-------------|
| BottomFrame | ✅ | 纯白填充 | 纯色覆盖 |
| GradientFrame | ✅ | 白色渐变 | Alpha 混合渐变层 |
| Minimal | ✅ | 透明 + 1px 线 | 细线绘制 |
| Overlay | ❌ | 圆角半透明矩形 | 圆角判断 + Alpha 混合 |

---

## 小结

| 概念 | 要点 |
|------|------|
| 画布扩展 | 创建更大的图像，原图复制到顶部 |
| 渐变 | 线性插值计算 Alpha，从透明到不透明 |
| 双线性插值 | 4 邻近像素加权平均，平滑缩放 |
| 圆角矩形 | 距离圆心判断，裁掉角外像素 |
| Alpha 混合 | 所有合成操作的最终步骤 |

---

## 延伸阅读

- [02-pixel-and-color.md](02-pixel-and-color.md) — Alpha 混合的数学公式
- [04-font-rendering.md](04-font-rendering.md) — 字体渲染的抗锯齿技术
- [06-resolution-scaling.md](06-resolution-scaling.md) — 分辨率与缩放的关系
