# LiteMark 深度改造计划：从 MVP 到发布标准

## 项目现状诊断

当前 LiteMark 是一个**功能可用的 MVP**，在字体渲染、布局引擎、错误处理和 API 契约方面存在大量技术债务。三个内置模板（Classic/Compact/Professional）视觉效果不佳的根因不是参数调不好，而是**渲染引擎的能力天花板太低**——无论怎么调 JSON 参数，都无法突破以下硬约束：

| 约束 | 现状 | 审美影响 |
|------|------|----------|
| 渲染模式 | 只有"底部加纯白框"一种 | 所有照片底部一刀切，割裂感强 |
| 布局算法 | 硬编码四列（左文/Logo/竖线/右文） | 右栏未右对齐，竖线像Word文档分隔线 |
| 字重系统 | 只有 SourceHanSansCN-Regular，weight 字段完全无效 | 所有文字一样粗细，无视觉层级 |
| 字体引擎 | rusttype（已停止维护） | 小字号模糊，无 hinting，内存泄漏 |
| 背景样式 | 纯色矩形 + 可选圆角 | 无渐变、无毛玻璃、无透明过渡 |
| Logo 缩放 | 最近邻插值 | 边缘锯齿明显 |
| 颜色系统 | 仅支持 6 位 hex，无 alpha | 无法实现半透明文字/背景 |
| 错误处理 | `Box<dyn Error>` 全局统一 | 调用方无法区分"字体缺失"和"解码失败" |

---

## 改造目标

1. **视觉多样性**：支持至少 4 种高品质水印风格（底部白框 / 渐变过渡 / 照片内嵌 / 极简线条）
2. **排版质量**：多字重字体 + 对齐控制 + 黄金比例层级 + 字距优化
3. **模板表达能力**：颜色含 alpha、背景支持渐变、布局可配置、条件渲染
4. **生产级稳定性**：结构化错误类型、视觉回归测试、性能基准
5. **API 兼容性**：尽可能保持 CLI / WASM 接口不变，改动集中在 Core 内部

---

## 阶段规划

### Phase 1: 基础重构（错误类型 + 字体引擎替换）

**目标**：消除 P0 技术债务，建立可维护的基石。

#### 1.1 定义结构化错误类型
- 使用 `thiserror` 为每个模块定义错误枚举
- `litemark-core::error::CoreError` 作为根错误类型
- 调用方可精确匹配 `ErrorKind::FontLoadFailed` / `ErrorKind::ExifParseFailed` 等

#### 1.2 替换 rusttype → ab_glyph
- `ab_glyph` 是 rusttype 的官方继任者，API 相似，维护活跃
- 解决 `Box::leak` 内存泄漏问题（`ab_glyph` 支持自有生命周期）
- 提升渲染质量（更好的曲线光栅化）
- 保留自定义字体注入能力

#### 1.3 引入多字重字体系统
- 添加 `SourceHanSansCN-Bold.otf` 到 `assets/fonts/`
- `WatermarkRenderer` 从单字体改为 `FontSet { regular: FontRef, bold: Option<FontRef> }`
- `render_text_simple` 接收 `weight` 参数并选择对应字体
- 若 bold 字体未加载，fallback 到 regular（不 panic）

#### 1.4 修复 Core 模块的文件系统依赖
- `create_builtin_templates()` 从文件读取改为 `include_str!` 编译时嵌入
- 确保 Core 真正实现"无文件系统依赖"的设计承诺

**Phase 1 验收标准**：
- `cargo test --workspace --exclude litemark-wasm` 全绿
- `cargo clippy` 无 warn
- 视觉报告对比：Bold 文字在 Professional 模板中可见加粗效果

---

### Phase 2: 布局引擎重构（从硬编码到配置驱动）

**目标**：打破"四列布局"的硬编码，让模板真正控制排版。

#### 2.1 引入布局区域（Region）概念
模板中每个 item 可指定所在区域：
```json
{
  "type": "text",
  "value": "{Author}",
  "region": "left-top",    // 新增：left-top | left-bottom | right-top | right-bottom | center
  "align": "left",         // 新增：left | center | right
  "font_size_ratio": 0.22,
  "weight": "bold",
  "color": "#1A1A1A"
}
```

#### 2.2 渲染模式（RenderMode）
模板新增顶层字段：
```json
{
  "name": "Overlay",
  "render_mode": "overlay",   // 新增：bottom-frame | gradient-frame | overlay | minimal
  "overlay": {
    "position": "bottom-right",
    "background": {
      "type": "rounded-rect",
      "color": "#00000080",   // 支持 alpha（最后两位 = 透明度）
      "radius": 12,
      "padding": 16
    }
  }
}
```

四种渲染模式说明：

| 模式 | 效果 | 适用场景 |
|------|------|----------|
| `bottom-frame` | 底部加纯色框（当前行为，保留） | 传统风格 |
| `gradient-frame` | 底部从透明渐变到白色 | 现代过渡风格，不割裂照片 |
| `overlay` | 在照片内部某角叠加半透明底 + 文字 | 不扩展画布，不裁切照片 |
| `minimal` | 底部细线 + 单行文字，无背景框 | 极简风格 |

#### 2.3 右对齐与居中对齐
- `render_text_simple` 增加 `align: HorizontalAlign` 参数
- 右对齐时根据文字宽度计算 `x = region_right - text_width`
- 居中对齐时 `x = region_center - text_width / 2`

#### 2.4 竖线分隔线优化
- 颜色从 `#C8C8C8` 改为 `#E8E8E8`（更淡）
- 支持 `"separator": { "style": "line" | "none" | "space" }`

#### 2.5 Logo 高质量缩放
- 替换最近邻缩放为双线性插值
- 消除 Logo 边缘锯齿

**Phase 2 验收标准**：
- 至少实现 `bottom-frame` + `gradient-frame` + `overlay` 三种模式
- Professional 模板使用 gradient-frame，视觉上底部与照片自然过渡
- 右栏文字在 Professional 模板中右对齐
- 视觉报告展示三种模式的对比

---

### Phase 3: 模板系统与视觉设计升级

**目标**：内置模板达到"可以直接发朋友圈"的品质。

#### 3.1 颜色系统支持 alpha
- `parse_color` 支持 `#RRGGBBAA` 格式（最后两位为 alpha）
- 背景、文字、分隔线均可设透明度
- 保持向后兼容（6 位 hex 默认 alpha=FF）

#### 3.2 背景样式扩展
```json
"background": {
  "type": "gradient",       // rect | gradient | none
  "direction": "top-to-bottom",
  "start_color": "#FFFFFF00",
  "end_color": "#FFFFFFFF",
  "start_opacity": 0.0,
  "end_opacity": 1.0
}
```

#### 3.3 间距节奏系统
引入基于 4px 网格的间距节奏：
- 小型间距：`padding_ratio * 0.5`
- 标准间距：`padding_ratio`
- 大型间距：`padding_ratio * 1.5`

模板可显式控制行间距：
```json
"line_spacing_ratio": 0.3   // 行高 = font_size * (1 + line_spacing_ratio)
```

#### 3.4 重新设计四个内置模板

**① Classic Frame（经典底框）**
- render_mode: `bottom-frame`
- 纯白底框，深灰文字
- 左：作者名（bold）+ 相机镜头（normal）
- 右：参数行（右对齐）
- 竖线分隔（细线 #E8E8E8）

**② Minimal Line（极简线条）**
- render_mode: `minimal`
- 无背景框，底部一条 1px 细线
- 单行文字：作者 · 相机 · 参数
- 所有文字同一字号，用字重和颜色区分层级

**③ Gradient Pro（渐变专业）**
- render_mode: `gradient-frame`
- 底部从透明渐变到 90% 白色
- 左：作者（bold, 大号）+ 日期（light, 小号, 浅灰）
- 右：相机（bold）+ 参数行（normal, 右对齐）
- 无竖线，用留白分隔

**④ Overlay Signature（内嵌签名）**
- render_mode: `overlay`
- 右下角圆角半透明黑底（`#00000080`）
- 白色文字，两行：作者（bold）+ 参数（normal）
- 不扩展画布，不裁切照片

**Phase 3 验收标准**：
- 四个模板在 800×600 / 1920×1080 / 6000×4000 下均视觉合格
- 用户主观评分：至少 3/4 模板"愿意使用"
- 视觉报告展示四个模板的 12 个用例

---

### Phase 4: 工程化与可维护性

**目标**：确保改造后的代码能长期稳定维护。

#### 4.1 视觉回归测试
- 集成 `image-compare` crate 进行像素级 diff
- 每个模板在固定尺寸（1920×1080）下生成参考图
- CI 中对比 PR 前后的差异，超过阈值则失败
- 参考图随版本更新手动刷新

#### 4.2 性能基准
- `criterion` 基准测试：单图渲染耗时、内存分配次数
- 目标：1920×1080 单图渲染 < 200ms（release 模式）

#### 4.3 文档更新
- `ARCHITECTURE.md` 更新为新架构
- `TEMPLATE_GUIDE.md` 更新为新模板语法
- 新增 `DESIGN_SYSTEM.md` 记录间距/颜色/字体规范

#### 4.4 WASM 兼容性验证
- `litemark-wasm` 随 Core API 变动同步更新
- `cargo check -p litemark-wasm --target wasm32-unknown-unknown` 通过

**Phase 4 验收标准**：
- CI 包含视觉回归测试步骤
- 基准测试可运行并输出报告
- WASM 编译通过
- 所有文档与代码一致

---

## 实施路径选项

### 选项 A：完整重构（Recommended）
按 Phase 1 → 2 → 3 → 4 顺序完整实施。预计 4-6 个迭代周期，每个 Phase 独立可交付。

**优点**：彻底解决问题，长期收益最大  
**风险**：改动量大，每个 Phase 之间可能存在依赖  
**适合**：确定要长期维护，愿意投入时间

### 选项 B：MVP 增强（先解决 80% 问题）
只做 Phase 1（错误类型+字体引擎） + Phase 3（模板重设计，但只支持 bottom-frame 模式） + 右对齐修复。

**优点**：改动可控，2-3 个迭代即可看到显著视觉提升  
**缺点**：布局引擎仍为硬编码，无法支持 overlay/gradient 等高级模式  
**适合**：希望快速看到效果，后续再考虑深度重构

### 选项 C：渐进式迭代（边用边改）
以"一个模板一个模板地改"为粒度，每个迭代只改一个模板，同时逐步替换底层引擎。

**优点**：每个迭代产出明确，可随时暂停  
**缺点**：技术债务清理不及时，代码可能暂时处于"新旧混杂"状态  
**适合**：时间不确定，需要保持可用性

---

## 我的建议

推荐 **选项 A 的 Phase 1+2+3**（跳过 Phase 4 的回归测试，先以人工视觉报告验收）。原因：

1. Phase 1 的 `thiserror` + `ab_glyph` 替换是**必须做的技术债务清理**，拖得越久成本越高
2. Phase 2 的 layout 重构是**解锁美观上限的关键**——没有区域/对齐/渲染模式的概念，模板怎么调都有限
3. Phase 3 的模板重设计是**用户可直接感知的产出**，四个高品质模板足以支撑首次发布
4. Phase 4 的视觉回归测试可以在有稳定模板后再补，不影响当前改造节奏

---

*计划生成时间：2026-04-17*  
*基于分支：dev*  
*当前测试状态：111 测试全绿*
