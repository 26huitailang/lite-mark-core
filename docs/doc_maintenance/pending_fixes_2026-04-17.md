# 文档整改清单

> 扫描时间：2026-04-17
> 扫描范围：22 个文档（排除 agent 过程文档）
> 已验证：6 个 High 优先级文档
> 状态：🟡 待处理

---

## 问题统计

| 严重程度 | 数量 | 状态 |
|---------|------|------|
| 🔴 Critical | 3 | 待处理 |
| 🟡 Warning | 13 | 待处理 |
| 🔵 Info | 14 | 待处理 |

---

## 🔴 Critical（严重）

### C1. README.md — CLI 命令名称错误
- **文档路径**：`README.md`
- **问题描述**：所有示例命令使用 `litemark`，但实际构建后的二进制文件名为 `litemark-cli`
- **影响**：用户复制 README 中的命令会直接报 "command not found"
- **建议修改**：将所有 `litemark` 改为 `litemark-cli`，或在 Cargo.toml 中将二进制名改为 `litemark`
- **状态**：🟡 待处理

### C2. ARCHITECTURE.md / litemark-core/README.md — 核心设计原则矛盾
- **文档路径**：`litemark-core/ARCHITECTURE.md`, `litemark-core/README.md`
- **问题描述**：文档宣称"平台无关、纯内存 API、无文件系统依赖"，但 `layout::create_builtin_templates()` 内部调用 `std::fs::read_to_string()` 读取模板文件
- **影响**：WASM 等无文件系统场景下该函数不可用，与项目核心架构承诺冲突
- **建议修改**：
  - 方案 A：将模板 JSON 通过 `include_str!` 编译时嵌入，消除运行时文件系统依赖
  - 方案 B：在文档中明确标注该函数为例外，并说明 WASM 下的替代方案
- **状态**：🟡 待处理

### C3. AGENTS.md — Rust 最低版本要求错误
- **文档路径**：`AGENTS.md`
- **问题描述**：文档写 `Rust (Edition 2024, 1.70+)`，但 Edition 2024 最低稳定版本为 **1.85.0**
- **影响**：使用 1.70 编译会直接报错
- **建议修改**：将 `1.70+` 改为 `1.85.0+`
- **状态**：🟡 待处理

---

## 🟡 Warning（警告）

### W1. README.md — `show-template` 子命令未文档化
- **文档路径**：`README.md`
- **问题描述**：`Usage` 部分只列出 `add`、`batch`、`templates` 三个子命令，遗漏了 `show-template`
- **建议修改**：增加 `litemark-cli show-template <template>` 说明
- **状态**：🟡 待处理

### W2. README.md — `LITEMARK_LOGO` 环境变量未提及
- **文档路径**：`README.md`
- **问题描述**：只提到 `LITEMARK_FONT` 环境变量，未提及 `LITEMARK_LOGO`
- **建议修改**：在 Logo 配置部分补充环境变量用法
- **状态**：🟡 待处理

### W3. README.md — HEIC/HEIF 支持未在功能列表中体现
- **文档路径**：`README.md`
- **问题描述**：Features 中未提及 HEIC/HEIF 支持，但代码已实现
- **建议修改**：增加 "🍎 HEIC/HEIF support - Native Apple image format decoding"
- **状态**：🟡 待处理

### W4. README.md — 输出格式限制未说明
- **文档路径**：`README.md`
- **问题描述**：CLI 无论输入格式如何，输出固定为 JPEG，文档未说明
- **建议修改**：添加 "当前 CLI 版本输出格式固定为 JPEG" 的说明
- **状态**：🟡 待处理

### W5. AGENTS.md — iOS 集成宣称支持但实际无代码
- **文档路径**：`AGENTS.md`
- **问题描述**：文档写"支持 CLI、WASM 和 iOS 集成"，但项目中无 iOS/Swift/ObjC 绑定代码
- **建议修改**：删除 iOS 声明，改为"计划支持"或仅列出已实现的 CLI + WASM
- **状态**：🟡 待处理

### W6. AGENTS.md — 测试命令在文档内部不一致
- **文档路径**：`AGENTS.md`
- **问题描述**：快速开始中写 `cargo test --workspace --exclude litemark-wasm`，但 Makefile 中 `make test` 未排除
- **建议修改**：统一两处命令，Makefile 也应添加 `--exclude litemark-wasm`
- **状态**：🟡 待处理

### W7. AGENTS.md — 项目结构描述不完整
- **文档路径**：`AGENTS.md`
- **问题描述**：遗漏了 `src/lib.rs`（库入口）和 `tests/`（测试套件），以及 `examples/`、`docs/` 等目录
- **建议修改**：补全项目结构树
- **状态**：🟡 待处理

### W8. ARCHITECTURE.md — Template 结构体字段描述不完整
- **文档路径**：`litemark-core/ARCHITECTURE.md`
- **问题描述**：示例中遗漏了 `padding`、`background`、`logo_size_ratio` 等字段
- **建议修改**：补全字段列表或明确标注为节选
- **状态**：🟡 待处理

### W9. ARCHITECTURE.md — "内置模板硬编码"描述错误
- **文档路径**：`litemark-core/ARCHITECTURE.md`
- **问题描述**：文档说"内置模板硬编码，无需外部文件"，但实际运行时读取 JSON 文件
- **建议修改**：改为"内置模板从 JSON 文件加载"（或配合 C2 方案改为编译时嵌入）
- **状态**：🟡 待处理

### W10. ARCHITECTURE.md — "渲染修改（in-place）"描述不准确
- **文档路径**：`litemark-core/ARCHITECTURE.md`
- **问题描述**：实际实现是创建新图像并替换原引用，并非真正的原地修改
- **建议修改**：改为"渲染生成新图像并替换原引用"
- **状态**：🟡 待处理

### W11. litemark-core/README.md — 编码格式支持范围描述不准确
- **文档路径**：`litemark-core/README.md`
- **问题描述**：说支持 JPEG、PNG、GIF、BMP、WebP、HEIC/HEIF，但编码端仅支持 JPEG、PNG、WebP
- **建议修改**：拆分为"解码输入"和"编码输出"分别说明
- **状态**：🟡 待处理

### W12. tests/README.md — 回归测试数据格式描述不匹配
- **文档路径**：`tests/README.md`
- **问题描述**：示例 JSON 包含 `input`（图片路径）字段，但实际数据结构为 `template` + `variables` 的文本替换级测试
- **建议修改**：更新文档中的回归测试 JSON 示例，与实际数据结构一致
- **状态**：🟡 待处理

### W13. tests/README.md — `visual_regression/` 目录不存在
- **文档路径**：`tests/README.md`
- **问题描述**：E2E 测试表格列出 `visual_regression/` 目录，但实际不存在
- **建议修改**：移除引用或补充实现
- **状态**：🟡 待处理

---

## 🔵 Info（建议优化）

### I1. README.md — Batch 处理高级参数未说明
- **建议**：补充 `--concurrency` 和 `--no-progress` 参数说明

### I2. README.md — `test_images/` 描述不够准确
- **建议**：改为 "Demo and example images"，避免与 `tests/` 混淆

### I3. AGENTS.md — 其他重要目录未提及
- **建议**：视需要补充 `examples/`、`docs/`、`output/` 等目录说明

### I4. ARCHITECTURE.md — v0.3.0 WASM 状态未更新
- **建议**：将 WASM 绑定层标记为已实现（`litemark-wasm/` 已存在）

### I5. ARCHITECTURE.md — 性能优化"原地修改"示例与实现不符
- **建议**：修改说明为"接口层面原地替换，内部目前涉及一次全图拷贝"

### I6. litemark-core/README.md — 示例代码缺少 `image` crate 依赖声明
- **建议**：在示例前增加 `use image::ImageFormat;` 注释或说明

### I7. litemark-core/README.md — `Template::from_json()` 错误类型未明确
- **建议**：在示例中标注返回类型为 `Result<Template, serde_json::Error>`

### I8. skills.md — 文件清单不完整
- **建议**：补充所有 6 个 Skill 的路径和说明

### I9. skills.md — 未提及 `doc-maintainer` Skill
- **建议**：在"与现有 Skill 体系深度融合"部分补充

### I10. skills.md — 协作关系列举不完整
- **建议**：补充 `litemark-release` 的协作说明

### I11. tests/README.md — `test_case` 宏未声明依赖
- **建议**：添加 `test-case` 到 Cargo.toml，或修改示例为手动循环

### I12. tests/README.md — `src/fixtures/` 和 `src/tools/` 为空但暗示有内容
- **建议**：调整目录结构树，标注为空或移除

### I13. tests/README.md — `fixtures/images/` 子目录均为空
- **建议**：补充说明当前为空或补充测试图片

### I14. tests/README.md — 代码示例格式瑕疵
- **建议**：删除 `cargo run` 行首多余空格

---

## 整改优先级建议

### 立即处理（P0）
1. C1 — README.md CLI 命令名（直接影响用户使用）
2. C3 — AGENTS.md Rust 版本号（影响编译）
3. W5 — AGENTS.md iOS 声明（虚假宣传）

### 尽快处理（P1）
4. C2 — 核心设计原则矛盾（架构一致性）
5. W1~W4 — README.md 功能遗漏
6. W11 — 编码格式说明错误

### 后续优化（P2）
7. W6~W10, W12~W13 — 其他文档不准确项
8. I1~I14 — Info 级优化项

---

*本清单由 doc-maintainer skill 自动生成，建议定期复查并更新状态。*
