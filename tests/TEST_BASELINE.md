# LiteMark 测试规范与基线

本文档定义 LiteMark 测试体系，服务于**AI Agent 为主维护、人类仅提出需求和问题**的治理模式。

> **核心原则**：任何代码变更的正确性判断，必须能由自动化测试独立完成。如果某个变更需要人类"看一眼"才能确认，说明测试体系存在缺口。

---

## 一、愿景与治理模型

### 1.1 治理模型

```
人类                    Agent
  │                       │
  ├─ 提出需求 / Bug 报告 ──▶│
  │                       ├─ 分析影响范围
  │                       ├─ 修改代码
  │                       ├─ 运行测试
  │                       ├─ 自我验证
  │◀─ 接收结果 / 提新问题 ─┤
  │                       │
  └─ 仅需介入例外情况 ◀────┤（视觉审美、架构决策）
```

### 1.2 自治判定标准

Agent 能自治修改当且仅当：

1. **有测试覆盖**：变更影响的路径被测试捕获
2. **有精确断言**：测试失败时，agent 能从错误信息定位到具体代码
3. **无视觉依赖**：正确性不依赖"人眼判断"
4. **可回滚**：变更若导致问题，能通过测试快速识别并 revert

---

## 二、测试体系重构：从"视觉优先"到"契约优先"

传统图像处理项目以视觉回归为核心验证手段。在 AI 自治模式下，这不可行——agent 无法"看懂"图片。

### 2.1 三层验证体系

| 层级 | 名称 | 验证方式 | Agent 可理解？ | 权重 |
|------|------|---------|---------------|------|
| L1 | **契约层** | 精确数值断言（坐标、尺寸、像素值） | ✅ 完全理解 | **核心** |
| L2 | **属性层** | 属性测试 / 不变量检查（输入→输出的数学关系） | ✅ 完全理解 | **核心** |
| L3 | **视觉层** | 像素级参考图对比 | ❌ 不理解（只能机械比对） | **辅助** |

**原则**：L3 视觉回归只能作为 L1/L2 通过后的补充验证，绝不能成为主要判断依据。

### 2.2 契约定义（Agent 的行为准则）

以下规则必须以**精确断言**形式写入测试，成为 agent 的"法律"：

#### 渲染几何契约

```rust
// 契约 1：边框高度
let short_edge = image.width().min(image.height()) as f32;
let ratio = template.frame_height_ratio.clamp(0.05, 0.20);
let calculated = (short_edge * ratio) as u32;
let expected_frame_height = calculated.max(80);
assert_eq!(
    image.height(),
    original_height + expected_frame_height,
    "边框高度必须符合公式：short_edge * clamp(ratio, 0.05, 0.20)，且不低于 80px"
);

// 契约 2：内容区域非空
let border_pixel = image.get_pixel(original_width / 2, original_height + 5);
assert_ne!(
    border_pixel.0,
    [0, 0, 0, 0],
    "边框区域必须有可见内容，不能是全透明"
);

// 契约 3：原图区域不变（Overlay 模式除外）
if template.render_mode != RenderMode::Overlay {
    assert_eq!(
        image.width(),
        original_width,
        "非 Overlay 模式下原图宽度必须保持不变"
    );
}
```

#### 模板布局契约

```rust
// 契约 4：左列包含优先级变量
let left_items = template.get_left_column_items();
assert!(
    left_items.iter().any(|i| i.contains("Author") || i.contains("Camera")),
    "左列必须包含 Author 或 Camera 之一"
);

// 契约 5：字体大小比例约束
assert!(
    template.primary_font_ratio > 0.0 && template.primary_font_ratio <= 1.0,
    "主字体比例必须在 (0, 1] 范围内"
);
```

#### 数据处理契约

```rust
// 契约 6：缺失 EXIF 不 panic
let empty_exif = ExifData::new();
let result = renderer.render(..., &empty_exif.to_variables(), ...);
assert!(result.is_ok(), "空 EXIF 必须能正常渲染");

// 契约 7：编码后数据非空
let encoded = encode_image(&image, ImageFormat::Jpeg)?;
assert!(
    encoded.len() > 1024,
    "编码后的 JPEG 必须大于 1KB（空图检测）"
);
```

---

## 三、L1 契约层基线

### 3.1 目标

所有业务规则都有**非视觉的精确断言**，agent 能从失败信息直接定位代码问题。

### 3.2 基线标准

每个模块必须将其关键行为写成可独立测试的契约：

#### `draw.rs` — 像素操作契约

| 函数 | 契约 | 断言形式 |
|------|------|---------|
| `blend_pixel` | Alpha 混合符合标准 over 操作 | 输入 (bg, fg) → 精确输出 RGBA |
| `render_frame_background` | 目标区域内所有像素变为 bg_color | 遍历断言区域内每个像素 |
| `render_gradient_background` | Alpha 从 64 线性递增到 255 | 首行 a=64, 末行 a=255, 单调递增 |
| `render_minimal_line` | 仅目标 y 行变色 | 该行变色，上下行不变 |
| `render_vertical_line` | 仅目标 x 列、y 范围变色 | 列内变色，列外不变 |
| `render_rounded_rect` | 圆角外像素不变 | 四角像素断言 + 中心区域断言 |

#### `text.rs` — 字体渲染契约

| 函数 | 契约 | 断言形式 |
|------|------|---------|
| `parse_font_data` | 有效字体解析成功，无效返回 Err | 输入/输出表驱动测试 |
| `load_default_font` | 默认字体加载成功 | 加载后 `text_width("A") > 0` |
| `text_width` | 空串宽度为 0，非空串宽度 > 0 | 边界值断言 |
| `render_text_simple` | 文本渲染后画布有变化 | 渲染前后像素差异计数 > 0 |

#### `logo.rs` — Logo 处理契约

| 函数 | 契约 | 断言形式 |
|------|------|---------|
| `render_logo_from_bytes` | 有效 Logo → 目标区域像素变化 | 差异像素 > 0 |
| `render_logo_from_bytes` | 无效 Logo → 静默跳过不 panic | `is_ok()` + 像素无变化 |

#### `color.rs` — 颜色解析契约

| 函数 | 契约 | 断言形式 |
|------|------|---------|
| `parse_color` | #RGB/#RRGGBB/#RRGGBBAA 正确解析 | 输入/输出表驱动 |
| `parse_color` | 无效格式返回 Err | 非法输入断言 |

#### `layout.rs` — 模板系统契约

| 函数 | 契约 | 断言形式 |
|------|------|---------|
| `substitute_variables` | `{Key}` 被替换为对应值 | 输入/输出精确匹配 |
| `substitute_variables` | 缺失变量保留原样 | `"{Missing}"` → `"{Missing}"` |
| `substitute_variables` | 空变量替换为空字符串 | `"{Empty}"` → `""` |

#### `renderer/mod.rs` — 编排契约

| 场景 | 契约 | 断言形式 |
|------|------|---------|
| BottomFrame | 高度增加，原图顶部保留 | `height == old + frame` + 原图像素不变 |
| Overlay | 尺寸不变，原图被覆盖 | `width == old && height == old` |
| 空变量 | 不 panic，占位符保留或消失 | 渲染成功 + 输出可编码 |

### 3.3 断言质量标准（面向 Agent）

**不合格的断言（agent 无法定位问题）：**
```rust
assert!(result.is_ok());                    // 不知道哪里错了
assert!(image.height() > original_height);  // 不知道期望高度是多少
```

**合格的断言（agent 能直接修复）：**
```rust
assert_eq!(
    image.height(),
    600 + 80,
    "600px 短边 × 0.12 ratio = 72px, clamp 到最小值 80px，\n\
     期望总高度 = 600 + 80 = 680，实际 = {}"
);
// agent 看到：期望 680，实际 672 → 检查 frame_height 计算中的 max(80) 是否被遗漏
```

### 3.4 画布规范

- 单元测试画布控制在 **10×100** 像素
- 断言必须精确到像素值，不允许"大概正确"
- 测试函数名必须包含被测函数名：`test_blend_pixel_alpha_zero`

---

## 四、L2 属性层基线

### 4.1 目标

验证输入与输出之间的数学关系，不依赖具体像素值。

### 4.2 属性测试清单

| 属性 | 验证方式 | 示例 |
|------|---------|------|
| 幂等性 | 同一输入渲染两次，输出相同 | `render(a) == render(a)` |
| 单调性 | 图片越大，边框高度 ≥ 小图边框 | `frame(4K) >= frame(1080p)` |
| 边界约束 | 边框高度始终在 [80, 短边×20%] | 对任意输入 `80 <= frame <= short*0.2` |
| 无损性 | 非 Overlay 模式下原图像素不变 | 原图区域像素渲染前后一致 |
| 编码完整性 | 任意有效输入都能编码为 JPEG/PNG | `encode(render(any_valid_input)).is_ok()` |

### 4.3 实现建议

使用 `proptest` 或手写参数化测试：

```rust
#[test]
fn test_frame_height_bounds() {
    for width in [400, 800, 1920, 4000, 8000] {
        for height in [300, 600, 1080, 3000, 6000] {
            let short = width.min(height);
            let frame = calculate_frame_height(short, 0.12);
            assert!(frame >= 80, "边框高度不能低于 80px: {}x{} -> {}", width, height, frame);
            assert!(frame as f32 <= short as f32 * 0.20, "边框高度不能超过短边 20%");
        }
    }
}
```

---

## 五、L3 视觉层基线（辅助验证）

### 5.1 定位

视觉回归不是主要验证手段，而是**契约层通过后的防退化层**。agent 无法理解视觉输出，但可以在以下安全条件下自动更新参考图。

### 5.2 Agent 更新参考图的安全条件

Agent **只有在同时满足以下全部条件时**，才能执行 `UPDATE_REFS=1`：

1. ✅ **所有 L1 契约测试通过** — 几何、布局、像素操作正确
2. ✅ **所有 L2 属性测试通过** — 数学关系未被破坏
3. ✅ **代码变更是故意的** — PR 描述中明确说明"视觉输出变更原因"
4. ✅ **变更范围可控** — 仅影响预期的模板/模式，非全局

**如果 L1/L2 有任何失败，agent 禁止更新参考图，必须先修复代码。**

### 5.3 基线矩阵

| 优先级 | 组合 | 说明 |
|--------|------|------|
| P0 | 4 模板 × 1920×1080 × 无Logo | 核心基线 |
| P0 | 4 模板 × 1920×1080 × 有Logo | Logo 渲染防退化 |
| P0 | Classic × 1024×1024 | 正方形布局 |
| P0 | Classic × 1080×1920 | 竖屏布局 |
| P1 | Classic × 1920×1080 × 极简EXIF | 缺失字段处理 |
| P1 | Classic × 800×600 | 小图下限 |
| P2 | 其他组合 | 按需扩展 |

### 5.4 容差标准

```rust
const PIXEL_TOLERANCE: u8 = 2;         // 单通道
const DIFF_RATIO_TOLERANCE: f64 = 0.001; // 0.1%
```

---

## 六、任务分级：Agent 自治边界

### 6.1 完全自治（Agent 独立完成，无需人类审查）

| 任务 | 前提条件 |
|------|---------|
| 修复编译错误 | `cargo check` 通过 |
| 修复契约测试失败 | 失败信息足够定位问题 |
| 纯逻辑重构 | L1/L2 测试全部通过 |
| 性能优化 | Benchmark 不退化 |
| 文档同步 | 代码变更对应文档自动更新 |

### 6.2 受限自治（Agent 执行后需人类确认或补充输入）

| 任务 | 人类需提供 |
|------|-----------|
| 新增模板 | 设计稿或描述（agent 实现后人类审美观） |
| 调整默认样式 | 确认"好看" |
| 更新参考图 | 确认 L1/L2 通过后的视觉变更是预期的 |
| 引入新依赖 | 安全审查 |

### 6.3 禁止自治（必须人类主导）

| 任务 | 原因 |
|------|------|
| 架构重构 | 影响测试体系本身，agent 可能破坏验证能力 |
| 放宽容差/删除测试 | agent 可能"作弊"让测试通过 |
| 修改测试契约 | 契约是 agent 的法律，不能自我修改 |
| 发布版本 | 需要人类对整体质量负责 |

---

## 七、覆盖率基线

覆盖率是**必要非充分条件**。达到基线不表示无 bug，未达标说明存在已知盲区。

### 7.1 测量

```bash
# 推荐
cargo llvm-cov -p litemark-core --html --open
```

### 7.2 分模块要求

| 模块 | 行覆盖率基线 | 原因 |
|------|-------------|------|
| `color.rs` | ≥ 90% | 纯解析逻辑 |
| `layout.rs` | ≥ 85% | 模板系统 |
| `draw.rs` | ≥ 85% | 像素操作可精确测试 |
| `exif.rs` | ≥ 80% | 解析逻辑 |
| `text.rs` | ≥ 75% | 字体相关有外部依赖 |
| `logo.rs` | ≥ 75% | 图像解码有外部依赖 |
| `image_io.rs` | ≥ 60% | I/O 错误路径多 |
| `renderer/mod.rs` | ≥ 75% | 编排逻辑 |
| **整体** | **≥ 75%** | — |

### 7.3 豁免

标注 `// TEST-EXEMPT: <原因>` 可豁免：
- 平台条件编译
- 仅调试日志
- 明确不可达代码

---

## 八、实施路线图（面向自治）

### Phase 1：契约层建设（2-3 天，阻塞后续所有自治能力）

**目标**：建立 agent 可理解的判断依据。

- [ ] `draw.rs` 内建 `#[cfg(test)]`，覆盖所有几何绘制函数（精确像素断言）
- [ ] `text.rs` 内建 `#[cfg(test)]`，覆盖字体加载和字宽计算
- [ ] `logo.rs` 内建 `#[cfg(test)]`，覆盖 Logo 加载和失败处理
- [ ] `renderer/mod.rs` 增加编排契约测试（高度计算、原图保留验证）
- [ ] `pipeline_tests.rs` 所有 `is_ok()` 替换为精确属性断言

**达标标志**：任意渲染代码变更若破坏了边框高度/原图保留/编码完整性，测试能精确指出期望值和实际值。

### Phase 2：属性层建设（1-2 天）

- [ ] 增加 `frame_height` 边界属性测试（任意尺寸输入）
- [ ] 增加渲染幂等性测试
- [ ] 增加编码完整性属性测试

### Phase 3：视觉回归扩展（1 天）

- [ ] 扩展视觉回归矩阵（有 Logo、正方形、竖屏）
- [ ] 生成参考图基线
- [ ] 建立"L1/L2 通过后方可更新参考图"的自动化检查

### Phase 4：验收样本集（持续）

- [ ] 建立 `tests/fixtures/samples/` 和 README
- [ ] 版本发布前运行样本集并生成报告

---

## 九、Agent 操作手册（快速参考）

### 当你需要修改渲染代码时

1. **先运行契约测试**：`cargo test -p litemark-test-suite --test unit`
2. **运行属性测试**：`cargo test -p litemark-test-suite --test integration -- pipeline`
3. **修改代码**
4. **再次运行 L1/L2**
5. **如果 L1/L2 失败**：修复代码，禁止更新参考图
6. **如果 L1/L2 通过但视觉回归失败**：
   - 确认变更是故意的（如：调整了边框高度公式）
   - 运行 `UPDATE_REFS=1 cargo test -p litemark-test-suite --test integration -- visual`
   - 在 PR 中说明视觉变更原因
7. **如果全部通过**：提交

### 当你看到测试失败时

| 失败类型 | 你的行动 |
|---------|---------|
| `assert_eq!(expected, actual)` with numbers | 检查计算逻辑，修复公式 |
| `assert_eq!(pixel, Rgba([...]))` | 检查绘制逻辑，修复像素操作 |
| `visual regression: diff 12.5%` | **先检查 L1/L2 是否通过** → 若通过则可能是合法变更；若 L1/L2 也失败，先修 L1/L2 |
| `encode failed` | 检查输出图像尺寸或内存 |

### 禁止做的事

- ❌ 不要删除或跳过测试来让 CI 通过
- ❌ 不要在没有 L1/L2 通过的情况下更新参考图
- ❌ 不要放宽 `PIXEL_TOLERANCE` 或 `DIFF_RATIO_TOLERANCE`
- ❌ 不要修改 `TEST_BASELINE.md` 中的契约定义

---

## 十、相关文档

- `tests/README.md` — 操作指南（命令速查）
- `litemark-core/ARCHITECTURE.md` — 渲染引擎架构
- `AGENTS.md` — 项目整体规范
