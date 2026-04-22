---
name: litemark-dev
description: LiteMark 项目开发速查。提供模板变量、make 命令、常见开发任务和调试技巧。当在 LiteMark 代码库中开发、调试或修改功能时使用。通用项目信息（构建命令、项目结构、开发规范）见项目根目录 AGENTS.md。
---

# LiteMark 开发技能

LiteMark 照片水印工具的**开发速查卡**。

> **通用项目信息**（技术栈、项目结构、完整构建命令、开发规范）见项目根目录 `AGENTS.md`。本 skill 只包含日常开发中高频使用的速查信息。

## Make 速查

```bash
make demo        # 生成所有模板演示图
make test        # 运行所有测试
make install     # 安装 CLI 到系统
```

## 模板变量

| 变量 | 说明 | 示例 |
|------|------|------|
| `{Author}` | 摄影师名 | "张三" |
| `{ISO}` | ISO 感光度 | "100" |
| `{Aperture}` | 光圈值 | "f/2.8" |
| `{Shutter}` | 快门速度 | "1/125" |
| `{Focal}` | 焦距 | "50mm" |
| `{Camera}` | 相机型号 | "Sony A7M4" |
| `{Lens}` | 镜头型号 | "FE 50mm F1.8" |
| `{DateTime}` | 拍摄时间 | "2025:01:15 14:30:00" |

## 模板开发

1. 在 `templates/` 创建 JSON 文件
2. 使用内置模板作为参考：`classic.json`, `compact.json`, `professional.json`, `overlay.json`
3. 测试：`cargo run -- templates`

## 字体配置

```bash
# 命令行指定
litemark add -i photo.jpg --font "/path/to/font.ttf"

# 环境变量
export LITEMARK_FONT="/path/to/font.ttf"
```

## 常见任务

### 添加新模板变量

1. 修改 `litemark-core/src/exif.rs` — 提取数据
2. 修改 `litemark-core/src/layout.rs` — 变量替换
3. 更新本文档变量表格
4. 更新 `AGENTS.md` 如有必要

### 调试渲染问题

```bash
# 启用详细日志
RUST_LOG=debug cargo run -- add -i test.jpg -o out.jpg
```

### 快速验证修改

```bash
# 增量检查（最快）
cargo check -p litemark-core

# 完整测试
cargo test --workspace --exclude litemark-wasm
```
