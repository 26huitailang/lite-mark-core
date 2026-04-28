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

- **语言**: Rust (Edition 2024, 1.85+，建议使用 stable 最新版)
- **图像处理**: `image` + `libheif-rs` (HEIC)
- **字体渲染**: `ab_glyph` + 嵌入式字体
- **EXIF 解析**: `kamadak-exif`
- **CLI**: `clap`
- **并行处理**: `rayon`

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
├── litemark-wasm/              # WASM 绑定
├── tests/                      # 测试套件（包名: litemark-test-suite）
├── templates/                  # JSON 模板文件
└── assets/                     # 字体和 Logo 资源
```

详细架构见 `litemark-core/ARCHITECTURE.md`。

## 核心设计原则

1. **平台无关**: `litemark-core` 无文件系统依赖，纯内存 I/O
2. **无副作用**: 图像处理使用纯函数
3. **模板系统**: JSON 模板支持变量替换

## 快速开始

```bash
# 构建
cargo build --workspace

# 测试（推荐，排除 WASM 以加快速度）
cargo test --workspace --exclude litemark-wasm

# 或运行完整测试套件（与 make test 行为一致）
cargo test --workspace

# 发布构建
cargo build --workspace --release

# 代码检查
cargo clippy --workspace --all-targets --exclude litemark-wasm
cargo check -p litemark-wasm --target wasm32-unknown-unknown
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

## CI/CD

- **测试工作流**: Push/PR 时运行 clippy + 测试 + WASM 检查
- **发布工作流**: Tag 推送时构建多平台二进制（macOS/Linux/Windows）

## 安全

- 本地处理，不上传云端
- 字体文件编译时嵌入
- Core 无 unsafe 代码
