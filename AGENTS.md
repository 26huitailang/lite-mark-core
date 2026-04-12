# LiteMark 项目指南

## 项目概述

LiteMark 是一个 Rust 照片水印处理工具，为照片添加参数边框（EXIF 数据）。支持 CLI、WASM 和潜在的 iOS 集成。

## 技术栈

- **语言**: Rust (Edition 2024, 1.70+)
- **图像处理**: `image` crate
- **字体渲染**: `rusttype` + 嵌入式 DejaVu Sans
- **EXIF 解析**: `kamadak-exif`
- **CLI**: `clap`
- **并行处理**: `rayon`
- **错误处理**: `anyhow` + `thiserror`

## 项目结构

```
lite-mark-core/                 # Workspace 根目录
├── litemark-core/              # 核心库（平台无关）
│   ├── src/exif.rs             # EXIF 数据提取
│   ├── src/layout.rs           # 模板引擎
│   ├── src/renderer.rs         # 水印渲染
│   └── src/image_io.rs         # 图像编解码
├── litemark-cli/               # 命令行工具
├── litemark-wasm/              # WASM 绑定
├── templates/                  # JSON 模板文件
├── assets/                     # 字体和 Logo 资源
└── Makefile                    # 常用任务
```

## 核心设计原则

1. **平台无关**: `litemark-core` 无文件系统依赖，纯内存 I/O
2. **无副作用**: 图像处理使用纯函数
3. **模板系统**: JSON 模板支持变量替换

## 快速开始

```bash
# 构建
cargo build --workspace

# 测试（排除 WASM）
cargo test --workspace --exclude litemark-wasm

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
