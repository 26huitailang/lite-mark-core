# LiteMark 开发指南

本文档为开发者提供 LiteMark 项目的开发环境配置、代码规范、贡献指南等信息。

## 快速开始

### 环境要求

- Rust 1.70+ (推荐使用 `rustup` 安装)
- Cargo (包含在 Rust 工具链中)
- Git

### 克隆和构建

```bash
# 克隆仓库
git clone https://github.com/26huitailang/lite-mark-core.git
cd lite-mark-core

# 构建项目
cargo build

# 运行测试
cargo test

# 运行示例
cargo run -- add -i test_images/800x600_landscape.jpg -t classic -o output.jpg --author "Developer"
```

## 项目结构

```
lite-mark-core/
├── src/
│   ├── main.rs              # CLI 入口
│   ├── lib.rs               # 库入口，导出公共模块
│   ├── exif_reader/         # EXIF 读取模块
│   │   └── mod.rs
│   ├── layout/              # 模板引擎
│   │   └── mod.rs
│   ├── renderer/            # 渲染引擎（核心）
│   │   └── mod.rs
│   └── io/                  # 文件 I/O
│       └── mod.rs
├── templates/               # JSON 模板文件
│   ├── classic.json
│   ├── modern.json
│   └── minimal.json
├── test_images/             # 测试图片
├── assets/                  # 资源文件
│   └── fonts/              # 字体文件
├── docs/                    # 文档
│   ├── ARCHITECTURE.md
│   └── DEVELOPMENT.md
├── .github/
│   └── workflows/          # CI/CD 配置
├── Cargo.toml              # 项目配置
└── README.md
```

## 核心概念

### 相框模式

LiteMark 使用"相框模式"而不是传统的"水印叠加"：

- **传统水印：** 在原图上叠加半透明文字（覆盖原图内容）
- **相框模式：** 在图片外添加边框区域，保持原图完整

```
传统模式：         相框模式：
┌─────────┐       ┌─────────┐
│ 原图    │       │  原图    │
│ [水印]  │       │         │
└─────────┘       ├─────────┤
                  │ Logo    │ ← 100px 底部相框
                  │ 参数    │
                  └─────────┘
```

### 变量系统

模板支持变量替换，变量来源：

1. **EXIF 数据：** ISO、光圈、快门等
2. **CLI 参数：** `--author` 等用户输入
3. **默认值：** 如果数据不可用，使用默认值

变量映射流程：

```rust
// EXIF 数据
ExifData { iso: Some(100), aperture: Some(2.8), ... }
   ↓
// 转换为字符串
{"ISO": "100", "Aperture": "f/2.8", ...}
   ↓
// 替换模板变量
"{Aperture} | ISO {ISO}" → "f/2.8 | ISO 100"
```

## 代码规范

### Rust 风格

遵循 Rust 官方风格指南：

```bash
# 自动格式化
cargo fmt

# 检查 lint
cargo clippy
```

### 命名约定

- **模块：** 小写，使用下划线（`exif_reader`）
- **结构体：** 大驼峰（`ExifData`, `WatermarkRenderer`）
- **函数：** 蛇形命名（`extract_exif_data`, `render_watermark`）
- **常量：** 大写下划线（`BOTTOM_FRAME_HEIGHT`）

### 错误处理

使用 `Result<T, Box<dyn std::error::Error>>` 处理可能失败的操作：

```rust
pub fn process_image(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let image = image::open(path)?;  // ? 操作符传播错误
    // ...
    Ok(())
}
```

### 文档注释

公共 API 必须有文档注释：

```rust
/// Renders a watermark frame to an image.
///
/// # Arguments
/// * `image` - The image to add frame to
/// * `template` - Template configuration
/// * `variables` - Variables for text substitution
///
/// # Returns
/// `Ok(())` on success, error otherwise
pub fn render_watermark(
    &self,
    image: &mut DynamicImage,
    template: &Template,
    variables: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // ...
}
```

## 添加新功能

### 1. 添加新的 CLI 命令

编辑 `src/main.rs`：

```rust
#[derive(Subcommand)]
enum Commands {
    Add { ... },
    Batch { ... },
    NewCommand {  // 添加新命令
        // 参数定义
    }
}
```

### 2. 添加新的模板变量

1. 在 `src/layout/mod.rs` 的 `substitute_variables` 中添加逻辑
2. 更新文档

### 3. 扩展渲染功能

1. 在 `src/renderer/mod.rs` 添加新的渲染方法
2. 在 `render_watermark` 中调用新方法
3. 添加单元测试

## 测试策略

### 单元测试

每个模块都应该有单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        // 测试逻辑
        assert_eq!(expected, actual);
    }
}
```

### 集成测试

在 `tests/` 目录添加集成测试，测试完整的流程。

### 测试图片

使用 `test_images/` 目录中的图片进行测试。确保测试图片：
- 多种分辨率
- 包含或不包含 EXIF 数据
- 不同格式（JPEG、PNG）

## 调试技巧

### 打印调试信息

```rust
println!("Debug: variable = {:?}", variable);
```

### 使用 Rust 调试器

```bash
# 安装调试器
rustup component add rust-gdb

# 运行调试
rust-gdb target/debug/litemark
```

### 性能分析

```bash
# 使用 perf（Linux）
perf record cargo run --release
perf report
```

## 贡献流程

1. **Fork 仓库**
2. **创建功能分支**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **编写代码和测试**
4. **确保测试通过**
   ```bash
   cargo test
   cargo clippy
   ```
5. **提交更改**
   ```bash
   git commit -m "feat: add amazing feature"
   ```
6. **推送到分支**
   ```bash
   git push origin feature/amazing-feature
   ```
7. **创建 Pull Request**

### Commit 消息规范

使用约定式提交：

- `feat:` 新功能
- `fix:` 修复 bug
- `docs:` 文档更新
- `refactor:` 代码重构
- `test:` 测试相关
- `chore:` 构建/工具相关

示例：

```
feat: add logo rendering support
fix: correct text positioning in frame mode
docs: update architecture documentation
```

## CI/CD

项目使用 GitHub Actions 进行自动化：

- **构建测试：** 每次 push 触发
- **Release 构建：** 发布 tag 时构建跨平台二进制
- **Lint 检查：** 自动运行 `cargo clippy`

查看 `.github/workflows/release.yml` 了解详情。

## 依赖管理

### 添加依赖

编辑 `Cargo.toml`：

```toml
[dependencies]
new-crate = "1.0"
```

### 更新依赖

```bash
cargo update
```

### 依赖选择原则

- 优先选择维护活跃的库
- 检查 LICENSE 兼容性
- 考虑依赖体积（特别是 iOS 打包）

## 常见问题

### 问题：字体渲染为方块

**原因：** 字体不支持该字符（如中文字符）

**解决：** 
1. 检查字体文件是否包含所需字符
2. 使用支持目标语言的字体

### 问题：内存占用过高

**原因：** 大图全加载到内存

**解决：**
1. 考虑流式处理
2. 限制同时处理的图片数量

### 问题：编译错误（rusttype）

**原因：** 版本冲突

**解决：**
```bash
cargo update rusttype
```

## 资源链接

- [Rust 官方文档](https://doc.rust-lang.org/)
- [rusttype 文档](https://docs.rs/rusttype/)
- [image crate 文档](https://docs.rs/image/)
- [项目 Issues](https://github.com/26huitailang/lite-mark-core/issues)

## 获取帮助

- 查看 [ARCHITECTURE.md](./ARCHITECTURE.md) 了解架构细节
- 提交 [Issue](https://github.com/26huitailang/lite-mark-core/issues)
- 参与 [Discussions](https://github.com/26huitailang/lite-mark-core/discussions)

