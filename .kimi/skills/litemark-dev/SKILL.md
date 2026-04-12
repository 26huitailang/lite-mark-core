# LiteMark 开发技能

LiteMark 照片水印工具的开发者快速参考。

## 构建命令

```bash
# 完整构建
cargo build --workspace

# 发布构建
cargo build --workspace --release

# 运行测试
cargo test --workspace --exclude litemark-wasm

# 代码检查
cargo clippy --workspace --all-targets --exclude litemark-wasm

# 格式化
cargo fmt

# WASM 检查
cargo check -p litemark-wasm --target wasm32-unknown-unknown
```

## 快速演示

```bash
# 单图处理
make demo

# 批量处理测试
make test
```

## 项目结构

```
litemark-core/      # 纯内存 API（平台无关）
litemark-cli/       # CLI 客户端
litemark-wasm/      # WASM 绑定
templates/          # JSON 模板
test_images/        # 测试图片
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
2. 使用内置模板作为参考：`classic.json`, `compact.json`
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

1. 修改 `litemark-core/src/exif.rs` - 提取数据
2. 修改 `litemark-core/src/layout.rs` - 变量替换
3. 更新本文档变量表格

### 调试渲染问题

```bash
# 启用详细日志
RUST_LOG=debug cargo run -- add -i test.jpg -o out.jpg
```
