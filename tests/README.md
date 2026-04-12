# LiteMark 测试套件

LiteMark 的综合性测试套件，遵循测试金字塔原则，包含单元测试、集成测试和端到端测试。

## 快速开始

```bash
# 运行完整测试套件
cargo test -p litemark-test-suite

# 运行特定层级测试
cargo test -p litemark-test-suite --test unit
cargo test -p litemark-test-suite --test integration
cargo test -p litemark-test-suite --test e2e

# 生成测试图片
 cargo run -p litemark-test-suite --bin generate-test-images

# 生成视觉报告
cargo run -p litemark-test-suite --bin generate-report

# 健康检查
cargo run -p litemark-test-suite --bin health-check
```

## 目录结构

```
tests/
├── src/
│   ├── unit/           # 单元测试
│   ├── integration/    # 集成测试
│   ├── e2e/            # 端到端测试
│   ├── fixtures/       # 测试数据定义
│   ├── tools/          # 测试工具
│   └── bin/            # 可执行工具
├── fixtures/           # 测试数据文件
│   ├── images/         # 测试图片
│   ├── templates/      # 测试模板
│   └── expected/       # 预期输出
└── assets/             # 报告资源文件
```

## 测试层级

### 单元测试 (`src/unit/`)

快速、独立的测试，覆盖单个函数/模块：

| 模块 | 测试文件 | 覆盖范围 |
|-----|---------|---------|
| exif | `exif_tests.rs` | EXIF 提取和格式化 |
| layout | `layout_tests.rs` | 模板系统和变量替换 |
| renderer | `renderer_tests.rs` | 水印渲染引擎 |
| image_io | `image_io_tests.rs` | 图像编解码 |

运行时间: < 10 秒

### 集成测试 (`src/integration/`)

测试多模块协作和完整处理流程：

| 测试文件 | 覆盖范围 |
|---------|---------|
| `pipeline_tests.rs` | 完整处理管道 |
| `template_tests.rs` | 模板组合测试 |
| `regression_tests.rs` | 回归测试套件 |

运行时间: < 60 秒

### 端到端测试 (`src/e2e/`)

测试完整用户场景和 CLI：

| 测试文件 | 覆盖范围 |
|---------|---------|
| `cli_tests.rs` | CLI 命令测试 |
| `visual_regression/` | 视觉回归测试 |

运行时间: < 5 分钟

## 添加新测试

### 添加单元测试

1. 在 `src/unit/<模块>_tests.rs` 添加测试函数
2. 命名规范: `test_<函数名>_<场景>`

```rust
#[test]
fn test_exif_extract_from_empty_data() {
    let result = exif::extract_from_bytes(&[]);
    assert!(result.is_ok());
    let data = result.unwrap();
    assert!(data.iso.is_none());
}
```

### 添加集成测试

1. 在 `src/integration/<测试>.rs` 添加测试
2. 使用 `test_case` 宏进行参数化测试

```rust
#[test_case(1920, 1080, "classic")]
#[test_case(800, 600, "compact")]
fn test_pipeline_dimensions(width: u32, height: u32, template: &str) {
    // 测试代码
}
```

### 添加回归测试

在 `fixtures/regressions.jsonl` 添加一行：

```json
{"id": "R004", "description": "问题描述", "input": "fixtures/images/jpeg/test.jpg", "template": "classic", "expected": {"success": true}}
```

## 测试数据

### 测试图片

测试图片存储在 `fixtures/images/`，按类型分类：

- `jpeg/`: JPEG 格式测试图片
- `png/`: PNG 格式测试图片
- `heic/`: HEIC 格式测试图片（可选）
- `edge_cases/`: 边界情况图片
- `exif_variants/`: 不同 EXIF 组合

### 生成测试图片

```bash
cargo run -p litemark-test-suite --bin generate-test-images
```

## 视觉报告

E2E 测试会生成 HTML 视觉报告：

```bash
cargo run -p litemark-test-suite --bin generate-report
```

报告位置: `target/test-reports/latest/index.html`

报告包含：
- 输入/输出图片对比
- 模板参数展示
- EXIF 数据
- 处理时间统计

## 设计理念

借鉴 SQLite 测试理念：

1. **回归测试优先**: 每个功能、边界条件都有回归测试
2. **测试即文档**: 测试代码清晰展示 API 用法
3. **自我验证**: 健康检查工具验证测试套件完整性
4. **平台全覆盖**: 多平台、多配置组合测试
