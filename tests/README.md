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
│   ├── integration/    # 集成测试（含视觉回归测试）
│   ├── e2e/            # 端到端测试
│   ├── fixtures/       # 测试数据定义（空，待补充）
│   ├── tools/          # 测试工具（空，待补充）
│   └── bin/            # 可执行工具
├── fixtures/           # 测试数据文件
│   ├── images/         # 测试图片（子目录为空，需运行 generate-test-images 生成）
│   ├── templates/      # 测试模板（空，待补充）
│   ├── expected/       # 预期输出（视觉回归基准图）
│   └── regressions.jsonl  # 回归测试用例定义
└── assets/             # 报告资源文件
```

## 测试分层与渲染测试规范

### 核心原则

**合成图片负责自动化回归，真实照片负责验收审查。**

不要依赖收集大量真实照片作为自动化渲染测试的主要手段。真实照片体积大、EXIF 不可控、参考图维护困难，且"好看"无法自动化断言。合成图片确定性高、可复现、适合 CI。

### 第一层：单元测试 (`src/unit/`)

快速、独立的测试，覆盖单个函数/模块。**对纯像素/几何函数做精确断言，不依赖视觉。**

| 模块 | 测试文件 | 覆盖范围 |
|-----|---------|---------|
| exif | `exif_tests.rs` | EXIF 提取和格式化 |
| layout | `layout_tests.rs` | 模板系统和变量替换 |
| renderer | `renderer_tests.rs` | 水印渲染引擎（创建、pipeline、编码） |
| image_io | `image_io_tests.rs` | 图像编解码 |

**渲染相关的单元测试规范：**

- 画布尺寸控制在 10×10 ~ 50×50，断言每个像素的 RGBA 值
- 必须覆盖 `draw.rs` 中的纯函数：`blend_pixel`、边框绘制、圆角矩形、垂直线
- 必须覆盖 `text.rs` 中的 `text_width` 和 `render_text_simple`
- 必须覆盖 `logo.rs` 中的 bilinear 采样和失败静默处理

运行时间: < 10 秒

### 第二层：集成测试 (`src/integration/`)

测试多模块协作和完整处理流程。

| 测试文件 | 覆盖范围 |
|---------|---------|
| `pipeline_tests.rs` | 完整处理管道 |
| `template_tests.rs` | 模板组合测试 |
| `regression_tests.rs` | 回归测试套件 |
| `visual_regression_tests.rs` | 视觉回归测试 |

**视觉回归测试规范：**

视觉回归测试通过像素级对比捕获跨模块集成的意外变更。参考图存储在 `fixtures/expected/`。

**覆盖矩阵（当前 → 目标）：**

| 维度 | 当前覆盖 | 目标覆盖 |
|------|---------|---------|
| 尺寸 | 1920×1080 | + 800×600, 1024×1024, 6000×4000 |
| 宽高比 | 16:9 横屏 | + 1:1, 9:16 竖屏, 3:1 全景 |
| Logo | 无 | 有 / 无 两种分支 |
| EXIF 组合 | 全字段 | 全字段 / 仅作者 / 空 |
| 模板 | 4 个内置 | 保持 |

**容差设置：**

```rust
/// 单个颜色通道允许的最大差异（应对抗锯齿跨平台差异）
const PIXEL_TOLERANCE: u8 = 2;
/// 允许差异像素占总像素的最大比例
const DIFF_RATIO_TOLERANCE: f64 = 0.001; // 0.1%
```

**参考图更新流程：**

1. 如果某次合法变更导致差异超过容差，先用 `UPDATE_REFS=1` 生成新参考图：
   ```bash
   UPDATE_REFS=1 cargo test -p litemark-test-suite --test integration -- visual
   ```
2. 在 PR 中说明变更原因，并单独提交参考图变更（与代码变更分开 commit）
3. 禁止在 CI 中自动更新参考图

**集成测试断言规范：**

- ❌ 避免仅使用 `assert!(result.is_ok())`
- ✅ 使用精确断言：
  ```rust
  let expected_frame = (600f32 * template.frame_height_ratio).clamp(0.05, 0.20) as u32;
  assert_eq!(image.height(), 600 + expected_frame.max(80));
  
  let bottom_pixel = image.get_pixel(100, 610);
  assert_ne!(bottom_pixel.0, [0, 0, 0, 0]); // 边框区域有内容
  ```

运行时间: < 60 秒

### 第三层：端到端测试 (`src/e2e/`)

测试完整用户场景和 CLI。

| 测试文件 | 覆盖范围 |
|---------|---------|
| `cli_tests.rs` | CLI 命令测试 |

运行时间: < 5 分钟

### 第四层：验收样本集（真实照片）

**目的**：发现合成图测不出的边缘 case（真实相机 EXIF 差异、特殊色彩分布等）。

**使用方式**：

1. **精选样本**：在 `fixtures/samples/` 存放 ~20 张真实照片，覆盖：
   - 不同相机品牌（Canon/Nikon/Sony/Fuji 的 EXIF 字段差异）
   - 不同宽高比（3:2, 4:3, 1:1, 16:9, 全景）
   - 不同分辨率（手机小图、全画幅大图）
   - 极端情况：无 EXIF、无镜头信息、超长作者名
   - 特殊场景：夜景（高 ISO）、黑白照片

2. **定期人工审查**：每月或版本发布前运行：
   ```bash
   cargo run -p litemark-test-suite --bin generate-report
   open target/test-reports/latest/index.html
   ```

3. **发现 bug 后转化**：如果某张真实照片触发 bug，提取其关键属性（尺寸、EXIF 组合）转化为合成图的自动化测试 case，而不是把整张真图加入回归。

**注意**：真实照片样本需在 `fixtures/samples/README.md` 中注明来源和授权信息。

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

测试图片存储在 `fixtures/images/`，按类型分类（子目录目前为空，可通过以下命令生成）：

- `jpeg/`: JPEG 格式测试图片
- `png/`: PNG 格式测试图片
- `heic/`: HEIC 格式测试图片（可选）
- `edge_cases/`: 边界情况图片
- `exif_variants/`: 不同 EXIF 组合

```bash
cargo run -p litemark-test-suite --bin generate-test-images
```

## 视觉报告

生成 HTML 视觉报告：

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
