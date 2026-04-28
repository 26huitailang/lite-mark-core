---
name: litemark-release
description: LiteMark 版本发布流程指南。涵盖版本号更新、CHANGELOG 维护、Git Tag 创建、GitHub Release 发布。当需要发布新版本或准备发布时使用。
---

# LiteMark 发布技能

LiteMark 版本发布流程快速参考。

## 发布检查清单

- [ ] 版本号已更新
- [ ] CHANGELOG.md 已更新
- [ ] 所有测试通过
- [ ] 文档已更新
- [ ] Git Tag 已创建
- [ ] GitHub Release 已发布

## 版本号位置

```toml
# Cargo.toml (根目录)
[workspace.package]
version = "0.2.0"

# litemark-core/Cargo.toml
[package]
version = "0.2.0"

# litemark-cli/Cargo.toml
[package]
version = "0.2.0"

# litemark-wasm/Cargo.toml
[package]
version = "0.2.0"
```

## 发布步骤

### 1. 准备发布

```bash
# 确保代码干净
git status

# 运行完整测试
cargo test --workspace --exclude litemark-wasm
cargo clippy --workspace --all-targets --exclude litemark-wasm

# 构建验证
cargo build --workspace --release
```

### 2. 更新版本

```bash
# 使用 cargo-set-version (需安装 cargo-edit)
cargo set-version 0.3.0

# 或手动更新所有 Cargo.toml
```

### 3. 更新 CHANGELOG.md

```markdown
## [0.3.0] - 2025-XX-XX

### Added
- 新功能描述

### Fixed
- 修复描述

### Changed
- 变更描述
```

### 4. 提交并打 Tag

```bash
git add .
git commit -m "chore: bump version to 0.3.0"
git tag v0.3.0
git push origin main --tags
```

### 5. GitHub Release

- 访问: https://github.com/26huitailang/lite-mark-core/releases
- 点击 "Draft a new release"
- 选择标签: `v0.3.0`
- 标题: `v0.3.0 - 版本描述`
- 内容: 复制 CHANGELOG 内容
- 上传构建产物（CI 会自动构建）

## CI/CD 说明

- `.github/workflows/test.yml` - PR 测试
- `.github/workflows/release.yml` - 发布构建

推送 tag 后，GitHub Actions 会自动构建跨平台二进制文件。
