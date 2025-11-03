# LiteMark 模板参数调整指南

## 📐 模板结构说明

每个 JSON 模板由以下几个部分组成：

```json
{
  "name": "模板名称",
  "anchor": "位置锚点",
  "padding": 0,
  "frame_height_ratio": 0.10,
  "logo_size_ratio": 0.35,
  "primary_font_ratio": 0.20,
  "secondary_font_ratio": 0.14,
  "padding_ratio": 0.10,
  "items": [...],
  "background": {...}
}
```

---

## 🎯 核心参数详解

### 1️⃣ **布局位置 (anchor)**

控制水印在图片中的位置：

| 值               | 效果   | 适用场景                 |
| ---------------- | ------ | ------------------------ |
| `"bottom-left"`  | 左下角 | 传统水印位置，不遮挡主体 |
| `"bottom-right"` | 右下角 | 签名风格，适合简约设计   |
| `"top-left"`     | 左上角 | 杂志风格                 |
| `"top-right"`    | 右上角 | 现代感强                 |
| `"center"`       | 居中   | 版权声明                 |

**调整示例：**
```json
"anchor": "bottom-left"  // 改为你想要的位置
```

---

### 2️⃣ **相框高度 (frame_height_ratio)**

控制整个水印区域的高度，是基于**图片短边**的比例。

| 值     | 效果         | 适用场景           |
| ------ | ------------ | ------------------ |
| `0.06` | 6% - 极简    | 不想水印太明显     |
| `0.08` | 8% - 紧凑    | 简约风格           |
| `0.10` | 10% - 标准   | 平衡美观与信息量   |
| `0.12` | 12% - 专业   | 信息丰富，专业展示 |
| `0.15` | 15% - 大尺寸 | 需要展示很多信息   |

**实际效果计算：**
- 假设照片短边为 3000px
- `frame_height_ratio: 0.10` → 相框高度 = 300px
- `frame_height_ratio: 0.12` → 相框高度 = 360px

**调整建议：**
```json
// 想要水印小一点？减小这个值
"frame_height_ratio": 0.08

// 想要水印大一点？增加这个值
"frame_height_ratio": 0.12
```

---

### 3️⃣ **字体大小比例**

所有字体大小都基于**相框高度**的比例：

#### **primary_font_ratio** - 主字体比例
- 默认：`0.20`（相框高度的 20%）
- 范围：`0.15` - `0.30`
- 用途：摄影师名称、相机型号等主要信息

#### **secondary_font_ratio** - 次字体比例
- 默认：`0.14`（相框高度的 14%）
- 范围：`0.12` - `0.20`
- 用途：参数信息、镜头型号等次要信息

**实际字号计算示例：**
```
相框高度 = 300px
primary_font_ratio = 0.20
→ 主字体大小 = 300 × 0.20 = 60px

secondary_font_ratio = 0.14
→ 次字体大小 = 300 × 0.14 = 42px
```

**调整示例：**
```json
// 想要字体更大更醒目？
"primary_font_ratio": 0.25,
"secondary_font_ratio": 0.18

// 想要字体更小更低调？
"primary_font_ratio": 0.16,
"secondary_font_ratio": 0.12
```

---

### 4️⃣ **内边距 (padding_ratio)**

控制文本/Logo 之间的间距，也是基于**相框高度**的比例。

| 值     | 效果     |
| ------ | -------- |
| `0.08` | 紧凑布局 |
| `0.10` | 标准间距 |
| `0.12` | 宽松布局 |
| `0.15` | 超宽松   |

**调整示例：**
```json
// 想要元素之间距离更近？
"padding_ratio": 0.08

// 想要更宽松的布局？
"padding_ratio": 0.15
```

---

### 5️⃣ **Logo 大小 (logo_size_ratio)**

Logo 尺寸基于**相框高度**的比例。

| 值     | 效果      |
| ------ | --------- |
| `0.25` | 小 Logo   |
| `0.35` | 标准 Logo |
| `0.45` | 大 Logo   |
| `0.60` | 超大 Logo |

**调整示例：**
```json
"logo_size_ratio": 0.40  // Logo 占相框高度的 40%
```

---

## 🎨 文本项配置 (items)

每个文本项包含以下参数：

```json
{
  "type": "text",
  "value": "{Author}",
  "font_size": 20,           // 固定像素大小（不推荐）
  "font_size_ratio": 0.20,   // 推荐：使用比例自适应
  "weight": "bold",          // 字重：normal / bold / light
  "color": "#1A1A1A"         // 颜色：十六进制色值
}
```

### **可用变量**

| 变量         | 说明       | 示例              |
| ------------ | ---------- | ----------------- |
| `{Author}`   | 摄影师名称 | "John Doe"        |
| `{Camera}`   | 相机型号   | "Canon EOS R5"    |
| `{Lens}`     | 镜头型号   | "RF 24-70mm F2.8" |
| `{Focal}`    | 焦距       | "50mm"            |
| `{Aperture}` | 光圈       | "f/2.8"           |
| `{Shutter}`  | 快门速度   | "1/250s"          |
| `{ISO}`      | ISO 感光度 | "400"             |

### **颜色选择指南**

#### 深色背景用浅色文字：
```json
"color": "#FFFFFF"  // 纯白
"color": "#F0F0F0"  // 浅灰白
"color": "#E0E0E0"  // 更深的浅灰
"color": "#CCCCCC"  // 中灰（次要信息）
```

#### 浅色背景用深色文字：
```json
"color": "#1A1A1A"  // 深黑（比纯黑柔和）
"color": "#2C2C2C"  // 深灰黑
"color": "#4A4A4A"  // 中深灰
"color": "#6A6A6A"  // 中灰（次要信息）
```

### **字重选择**

```json
"weight": "bold"    // 粗体 - 用于主要信息（摄影师名、相机型号）
"weight": "normal"  // 常规 - 用于次要信息（参数）
"weight": "light"   // 细体 - 用于极简风格
```

---

## 🖼️ 背景配置 (background)

```json
"background": {
  "type": "rect",        // rect（矩形）或 circle（圆形）
  "opacity": 0.3,        // 透明度：0.0 - 1.0
  "radius": 8,           // 圆角半径（像素）
  "color": "#000000"     // 背景颜色
}
```

### **透明度效果**

| 值    | 效果                 |
| ----- | -------------------- |
| `0.1` | 非常淡，几乎看不见   |
| `0.2` | 淡背景，适合浅色遮罩 |
| `0.3` | 标准透明度           |
| `0.4` | 明显背景             |
| `0.5` | 半透明               |
| `0.8` | 接近不透明           |

### **不需要背景？**
```json
"background": null  // 设为 null 即可去除背景
```

---

## 🎯 实用调整场景

### 场景 1️⃣：字体太小看不清

**问题：** 生成的水印文字太小
**解决方案：**
```json
// 方案 A：增大相框高度
"frame_height_ratio": 0.12,  // 从 0.10 改为 0.12

// 方案 B：增大字体比例
"primary_font_ratio": 0.24,    // 从 0.20 改为 0.24
"secondary_font_ratio": 0.18,  // 从 0.14 改为 0.18
```

### 场景 2️⃣：水印太占地方

**问题：** 水印相框太高，遮挡照片
**解决方案：**
```json
// 减小相框高度
"frame_height_ratio": 0.06,  // 改为最小值

// 减少内容项
"items": [
  {
    "type": "text",
    "value": "{Author} • {Aperture} • ISO {ISO}",  // 合并为一行
    "font_size_ratio": 0.28,
    "weight": "normal",
    "color": "#1A1A1A"
  }
]
```

### 场景 3️⃣：文字对比度不够

**问题：** 浅色照片上白字看不清 / 深色照片上黑字看不清

**解决方案 A - 添加半透明背景：**
```json
"background": {
  "type": "rect",
  "opacity": 0.3,      // 调整透明度
  "radius": 8,
  "color": "#000000"   // 深色照片用黑底，浅色照片用白底 #FFFFFF
}
```

**解决方案 B - 调整文字颜色：**
```json
// 深色照片用浅色文字
"color": "#FFFFFF"

// 浅色照片用深色文字
"color": "#1A1A1A"
```

### 场景 4️⃣：想要更专业的排版

**解决方案：** 使用渐变颜色层次
```json
"items": [
  {
    "type": "text",
    "value": "{Author}",
    "font_size_ratio": 0.22,
    "weight": "bold",
    "color": "#1A1A1A"  // 最深色 - 主要信息
  },
  {
    "type": "text",
    "value": "{Camera} • {Lens}",
    "font_size_ratio": 0.16,
    "weight": "normal",
    "color": "#4A4A4A"  // 中深色 - 次要信息
  },
  {
    "type": "text",
    "value": "{Focal} • {Aperture} • {Shutter} • ISO {ISO}",
    "font_size_ratio": 0.14,
    "weight": "normal",
    "color": "#6A6A6A"  // 浅灰色 - 参数信息
  }
]
```

---

## 🔧 调整流程建议

1. **先调整相框大小**
   - 从 `frame_height_ratio` 开始，确定整体大小

2. **再调整字体大小**
   - 调整 `primary_font_ratio` 和 `secondary_font_ratio`

3. **优化间距**
   - 调整 `padding_ratio` 让布局更舒适

4. **调整颜色对比度**
   - 根据照片风格选择合适的文字颜色

5. **添加背景（可选）**
   - 如果对比度不够，添加半透明背景

---

## 📊 快速对比表

| 模板             | 相框高度 | 主字体 | 次字体 | 内边距 | 适用场景 |
| ---------------- | -------- | ------ | ------ | ------ | -------- |
| **Compact**      | 6%       | 0.28   | 0.18   | 0.08   | 极简风格 |
| **Elegant**      | 8%       | 0.24   | 0.16   | 0.15   | 简约优雅 |
| **Classic**      | 10%      | 0.20   | 0.14   | 0.10   | 标准平衡 |
| **Professional** | 12%      | 0.22   | 0.16   | 0.12   | 专业展示 |

---

## 💡 使用技巧

### ✅ 推荐做法
- 使用 `*_ratio` 比例参数，而不是固定 `font_size`
- 深色照片用浅色文字 + 深色半透明背景
- 浅色照片用深色文字 + 浅色半透明背景
- 主要信息用 `bold`，次要信息用 `normal`

### ❌ 避免做法
- 不要让 `frame_height_ratio` 超过 0.20（太大）
- 不要用纯黑 `#000000`，用 `#1A1A1A` 更柔和
- 不要让所有文字都用同一颜色，缺少层次感
- 不要在一个模板里放太多信息（超过 4 行）

---

## 🎨 示例：从零创建自定义模板

假设你想要一个**右下角、紧凑型、深色文字**的水印：

```json
{
  "name": "MyCustom",
  "anchor": "bottom-right",
  "padding": 0,
  "frame_height_ratio": 0.08,
  "logo_size_ratio": 0.35,
  "primary_font_ratio": 0.24,
  "secondary_font_ratio": 0.16,
  "padding_ratio": 0.12,
  "items": [
    {
      "type": "text",
      "value": "{Author}",
      "font_size_ratio": 0.24,
      "weight": "bold",
      "color": "#2C2C2C"
    },
    {
      "type": "text",
      "value": "{Aperture} • {Shutter} • ISO {ISO}",
      "font_size_ratio": 0.16,
      "weight": "normal",
      "color": "#5C5C5C"
    }
  ],
  "background": null
}
```

保存为 `templates/mycustom.json`，然后使用：
```bash
litemark add -i photo.jpg -t mycustom -o output.jpg
```

---

## 🚀 测试建议

1. 先用小图测试（避免等待时间）
2. 准备深色和浅色两种照片测试对比度
3. 每次只调整一个参数，观察效果
4. 记录满意的参数组合

---

需要更多帮助？查看 `templates/` 目录下的示例模板！
