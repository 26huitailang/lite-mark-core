# LiteMark 项目指南

## 项目概述

LiteMark 是一个 Rust 照片水印处理工具，为照片添加参数边框（EXIF 数据）。支持 CLI、WASM 和 Desktop 集成。

## 信息分层与文档导航

本项目采用三层信息架构，agent 应按层级定位所需信息：

| 层级 | 文档/Skill | 职责 | 何时查阅 |
|------|-----------|------|---------|
| **L1 权威指南** | `AGENTS.md`（本文档） | 项目概述、技术栈、项目结构、构建命令、开发规范、CI/CD | **首次接触项目**或需要通用项目信息时 |
| **L2 任务技能** | `.kimi/skills/litemark-dev` | 开发速查：模板变量、常见任务、调试技巧 | 日常开发时快速参考 |
| **L2 任务技能** | `.kimi/skills/litemark-release` | 版本发布：版本号、CHANGELOG、Git Tag | 发布新版本时 |
| **L2 任务技能** | `.kimi/skills/dev-pipeline` | 完整开发流水线：需求→设计→实现→审核 | 新功能完整开发流程 |
| **L2 任务技能** | `.kimi/skills/rust-test-runner` | 测试运行与错误分析 | `cargo test` 相关任务 |
| **L2 任务技能** | `.kimi/skills/req-bug-analyzer` | 只读分析与结构化报告 | 需求分析、Bug 排查 |
| **L2 任务技能** | `.kimi/skills/doc-maintainer` | 文档一致性检查 | 检查文档与代码是否同步 |
| **L3 技术文档** | `README.md` | 终端用户功能介绍和安装指南 | 了解项目能做什么 |
| **L3 技术文档** | `litemark-core/README.md` | 库使用者 API 示例 | 集成 Core 库 |
| **L3 技术文档** | `litemark-core/ARCHITECTURE.md` | 内部架构和设计决策 | 深入理解实现细节 |

**原则**：
- `AGENTS.md` 是**单源真理**，通用项目信息以本文档为准
- Skills 不重复 `AGENTS.md` 内容，而是引用并补充**任务特有**信息
- 修改项目-wide 信息时，优先更新 `AGENTS.md`

## 技术栈

- **语言**: Rust (Edition 2024, MSRV ~1.85+，建议使用 stable 最新版)
- **图像处理**: `image` + `libheif-rs` (HEIC，原生平台 only)
- **字体渲染**: `ab_glyph` + 嵌入式字体
- **EXIF 解析**: `kamadak-exif`
- **CLI**: `clap`
- **并行处理**: `rayon` (原生平台 only)

## 项目结构

```
lite-mark-core/                 # Workspace 根目录
├── litemark-core/              # 核心库（平台无关，纯内存 API）
│   ├── src/error.rs            # 错误类型定义
│   ├── src/exif.rs             # EXIF 数据提取
│   ├── src/image_io.rs         # 图像编解码
│   ├── src/layout.rs           # 模板引擎
│   ├── src/lib.rs              # 库入口
│   └── src/renderer/           # 水印渲染模块
│       ├── mod.rs              # 渲染器入口
│       ├── color.rs            # 颜色处理
│       ├── draw.rs             # 绘制逻辑
│       ├── logo.rs             # Logo 渲染
│       └── text.rs             # 文字渲染
├── litemark-cli/               # 命令行工具
│   ├── src/main.rs             # clap CLI 定义与命令分发
│   ├── src/commands.rs         # 单图处理、模板列表、模板详情
│   ├── src/batch.rs            # 并行批处理（rayon + indicatif）
│   └── src/utils.rs            # 模板/字体/Logo 加载辅助函数
├── litemark-wasm/              # WASM 绑定（cdylib）
├── tests/                      # 测试套件（包名: litemark-test-suite）
│   ├── src/unit/               # 单元测试
│   ├── src/integration/        # 集成测试（含视觉回归）
│   ├── src/e2e/                # 端到端测试
│   └── src/bin/                # 可执行工具（generate-report, health-check 等）
├── templates/                  # JSON 模板文件
└── assets/                     # 字体和 Logo 资源
```

详细架构见 `litemark-core/ARCHITECTURE.md`。

## 核心设计原则

1. **平台无关**: `litemark-core` 无文件系统依赖，纯内存 I/O。**不允许在 Core 中引入 `std::fs` 依赖**
2. **无副作用**: 图像处理使用纯函数
3. **模板系统**: JSON 模板支持变量替换
4. **错误处理**: `CoreError`（`thiserror`）已定义，但公共 API 仍返回 `Box<dyn std::error::Error>`，尚未全面迁移

## 渲染管线

```
图像字节 → decode_image → 提取 EXIF → 加载模板 → 变量替换
    → 创建 WatermarkRenderer → render_watermark_with_logo_bytes → encode_image → 输出字节
```

- `WatermarkRenderer` 持有 `FontSet`（regular + optional bold），渲染时直接修改 `DynamicImage`
- 自定义字体数据通过 `Box::leak` 泄漏到 `'static` 以简化 `ab_glyph` 的 `FontRef` 生命周期管理
- 支持四种 `RenderMode`：`BottomFrame` / `GradientFrame` / `Overlay` / `Minimal`

## 平台兼容性

- `litemark-wasm` 使用条件编译：`#[cfg(target_arch = "wasm32")]` / `#[cfg(not(target_arch = "wasm32"))]`
- `libheif-rs` 和 `rayon` 仅在非 WASM 目标可用
- HEIC 支持仅限原生构建，WASM 不可用

## 快速开始

```bash
# 构建
cargo build --workspace
cargo build --workspace --release

# 测试（推荐，排除 WASM 以加快速度）
cargo test --workspace --exclude litemark-wasm

# 或运行完整测试套件（与 make test 行为一致）
cargo test --workspace

# 测试分层（tests 包名：litemark-test-suite）
cargo test -p litemark-test-suite --test unit -- --test-threads=8
cargo test -p litemark-test-suite --test integration
cargo test -p litemark-test-suite --test e2e

# 更新视觉回归基准图
UPDATE_REFS=1 cargo test -p litemark-test-suite --test integration -- visual

# 测试工具二进制
cargo run -p litemark-test-suite --bin generate-report      # HTML 视觉报告
cargo run -p litemark-test-suite --bin health-check

# 代码检查
cargo clippy --workspace --all-targets --exclude litemark-wasm
cargo check -p litemark-wasm --target wasm32-unknown-unknown

cargo fmt
```

## 开发规范

- **格式化**: `cargo fmt`
- **命名规范**: 
  - 模块/函数: `snake_case`
  - 结构体: `PascalCase`
  - 常量: `SCREAMING_SNAKE_CASE`
- **文档语言**: 中文
- **提交格式**: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`

## 常用命令

```bash
make demo        # 生成所有模板演示图
make test        # 运行所有测试
make install     # 安装 CLI 到系统
```

## 测试规范

测试分层（`tests/` 包名 `litemark-test-suite`）：

- **单元测试** (`src/unit/`) — 快速、独立，对纯像素/几何函数做精确断言，画布控制在 10×10 ~ 50×50
- **集成测试** (`src/integration/`) — 多模块协作、完整 pipeline、视觉回归测试。视觉回归对比 `fixtures/expected/` 参考图，容差：单通道差异 ≤2，差异像素比例 ≤0.1%
- **E2E 测试** (`src/e2e/`) — CLI 命令测试，CI 中在 Ubuntu 和 macOS 双平台运行
- **真实照片样本** — 用于 `generate-report` 定期人工审查，不加入自动化回归

测试命名规范：`test_<函数名>_<场景>`

## CI/CD

- **测试工作流** (`.github/workflows/test.yml`): Push/PR 时运行 clippy + 测试 + WASM 检查
- **测试套件工作流** (`.github/workflows/test-suite.yml`): 单元 + 集成 + E2E + 视觉报告 + 健康检查
- **发布工作流** (`.github/workflows/release.yml`): Tag 推送时构建多平台二进制（macOS/Linux/Windows）

## 安全

- 本地处理，不上传云端
- 字体文件编译时嵌入
- Core 无 unsafe 代码
