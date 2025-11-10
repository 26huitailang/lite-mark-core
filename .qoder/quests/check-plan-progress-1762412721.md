# LiteMark 项目进度分析与后续规划

## 📊 当前项目状态概览

**分析时间**: 2025年当前  
**项目阶段**: Week 3 末期 → 准备进入 Week 4  
**整体完成度**: MVP 已完成 ✅，进入跨平台扩展阶段

---

## ✅ 已完成功能清单（Week 1-3）

### 核心功能模块（100% 完成）

#### 1. EXIF 数据提取模块
- ✅ 集成 kamadak-exif 库实现真实 EXIF 解析
- ✅ 支持 8 个核心字段提取：ISO、光圈、快门、焦距、相机型号、镜头型号、拍摄时间、作者
- ✅ 智能快门速度格式化（1/125、2s 等）
- ✅ 完善错误处理机制（无 EXIF 数据、部分字段缺失等）
- ✅ 支持 JPEG、PNG、TIFF 格式

#### 2. 图像渲染引擎
- ✅ 底部相框模式渲染
- ✅ rusttype 专业字体渲染（支持中英文配置）
- ✅ Logo 自动加载和缩放
- ✅ 高质量图像输出（保持原分辨率）

#### 3. 模板系统
- ✅ JSON 配置解析
- ✅ 变量替换功能（{ISO}、{Aperture}、{Author} 等）
- ✅ 7 个内置模板（classic、modern、minimal、elegant、professional、dark、compact）
- ✅ 模板查看和列表命令

#### 4. CLI 工具
- ✅ clap v4.4 框架集成
- ✅ 单张图片处理（add 命令）
- ✅ 批量目录处理（batch 命令）
- ✅ 模板管理命令（templates、show-template）
- ✅ 自定义作者、字体、Logo 参数支持

#### 5. 质量保障
- ✅ 核心模块单元测试（exif_reader、layout 基础测试）
- ✅ 代码文档完善（中文注释）
- ✅ CI/CD 流程配置（GitHub Actions）
- ✅ 跨平台构建支持（macOS Intel/ARM、Linux GNU/musl、Windows）

#### 6. 文档体系
- ✅ README.md（项目说明、快速开始）
- ✅ ARCHITECTURE.md（架构设计文档）
- ✅ examples/exif_extraction.md（EXIF 提取指南）
- ✅ examples/chinese_font_guide.md（中文字体配置指南）
- ✅ CHANGELOG_v0.2.0.md（版本更新日志）

---

## 🚧 未完成功能清单

### Week 3 剩余任务（P1 优先级）

#### 1. Logo 路径参数化
**当前状态**: Logo 路径硬编码在模板中  
**待实现内容**:
- 添加 `--logo` CLI 参数
- 支持 `LITEMARK_LOGO` 环境变量
- 模板中 logo 路径支持相对路径和绝对路径

#### 2. 测试覆盖完善
**当前状态**: 仅有 exif_reader 模块完整测试  
**待实现内容**:
- renderer 模块单元测试（字体渲染、相框生成）
- layout 模块完整测试（变量替换边界情况）
- io 模块测试（批量处理、文件操作）
- 端到端集成测试（完整流程验证）

#### 3. 发布准备
**当前状态**: CI/CD 已配置但未发布  
**待实现内容**:
- 完善 CHANGELOG.md（规范化版本日志）
- 创建 v0.2.0 Release
- 发布预编译二进制文件
- 编写发布公告

---

## 📅 路线图对照分析

### Week 1-2：Core MVP（✅ 100% 完成）
| 原计划功能           | 完成状态 | 备注                  |
| -------------------- | -------- | --------------------- |
| CLI 工具 + EXIF 解析 | ✅        | kamadak-exif 集成完成 |
| 相框模式渲染         | ✅        | rusttype 字体渲染     |
| Logo 支持            | ✅        | 自动缩放和居中        |
| 模板系统             | ✅        | JSON 配置 + 变量替换  |
| 批量处理             | ✅        | walkdir 实现目录遍历  |
| 单元测试             | ✅        | exif_reader 完整覆盖  |

### Week 3：完善 + 发布（⚠️ 60% 完成）
| 原计划功能   | 完成状态 | 剩余工作              |
| ------------ | -------- | --------------------- |
| 优化字体渲染 | ✅        | 已完成                |
| 完善模板系统 | ✅        | 已完成                |
| CI/CD 配置   | ✅        | GitHub Actions 已配置 |
| 完善文档     | ✅        | 已完成                |
| Logo 参数化  | ❌        | 待实现                |
| 完善测试覆盖 | 🚧        | 部分完成              |
| 发布 v0.2.0  | ❌        | 待执行                |

### Week 4-6：iOS 原型（❌ 0% 未开始）
| 原计划功能            | 当前状态 | 依赖条件          |
| --------------------- | -------- | ----------------- |
| Rust → staticlib 编译 | ❌        | Week 3 完成后启动 |
| Swift Bridging 封装   | ❌        | 需要 FFI 模块     |
| iOS App UI 实现       | ❌        | SwiftUI 开发      |
| 图片选择与预览        | ❌        | PhotoKit 集成     |
| TestFlight 内测       | ❌        | App 实现后        |

### Month 2-3：Web WASM（❌ 0% 未开始）
| 原计划功能        | 当前状态 | 技术准备            |
| ----------------- | -------- | ------------------- |
| wasm-bindgen 绑定 | ❌        | 需设计 WASM API     |
| Web Demo 页面     | ❌        | HTML/JS 界面        |
| 字体加载优化      | ❌        | @font-face + subset |
| 浏览器内处理      | ❌        | FileReader API      |

---

## 🎯 立即可执行的后续任务清单

### 阶段一：完成 Week 3 剩余工作（1-2 天）

#### 任务 1：Logo 路径参数化（1-2 小时）
**优先级**: P1  
**目标**: 用户可通过命令行或环境变量指定 Logo 路径

**实现步骤**:
1. 修改 CLI 参数
   - 在 `src/main.rs` 的 `AddArgs` 和 `BatchArgs` 中添加 `--logo` 参数
   - 类型为 `Option<PathBuf>`

2. 环境变量支持
   - 检查 `LITEMARK_LOGO` 环境变量
   - 优先级：CLI 参数 > 环境变量 > 模板默认值

3. 模板处理逻辑
   - 在 renderer 模块中，如果用户指定了 logo 路径，覆盖模板配置
   - 保持向后兼容性（模板内 logo 路径仍然有效）

**验收标准**:
- 命令 `litemark add -i input.jpg --logo my_logo.png` 正常工作
- 设置环境变量 `export LITEMARK_LOGO=logo.png` 后生效
- 不指定 logo 时使用模板默认值

---

#### 任务 2：完善测试覆盖（2-3 小时）
**优先级**: P1  
**目标**: 核心模块测试覆盖率达到 80% 以上

**测试模块**:

1. **renderer 模块测试** (1 小时)
   - 相框生成测试（尺寸计算、颜色正确性）
   - 文本定位测试（居中、对齐）
   - Logo 缩放测试（比例保持、位置正确）

2. **layout 模块测试** (0.5 小时)
   - 变量替换边界情况（缺失变量、特殊字符）
   - 模板加载测试（内置模板、自定义路径）
   - JSON 解析错误处理

3. **io 模块测试** (0.5 小时)
   - 批量处理测试（多文件、嵌套目录）
   - 文件过滤测试（格式筛选）
   - 错误处理测试（权限、空间不足）

4. **集成测试** (1 小时)
   - 端到端流程测试（add 命令完整流程）
   - 批量处理完整流程
   - 不同模板测试

**验收标准**:
- `cargo test` 通过率 100%
- 代码覆盖率 > 80%（可用 tarpaulin 检测）
- 所有边界情况都有测试用例

---

#### 任务 3：发布 v0.2.0（1-2 小时）
**优先级**: P1  
**目标**: 正式发布首个稳定版本

**执行步骤**:

1. **完善 CHANGELOG.md** (30 分钟)
   - 规范化版本日志格式
   - 列出所有新增功能
   - 记录 Breaking Changes（如有）
   - 添加升级指南

2. **创建 Git Tag** (5 分钟)
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0: MVP with real EXIF extraction"
   git push origin v0.2.0
   ```

3. **触发 CI/CD 构建** (自动，等待 10-15 分钟)
   - GitHub Actions 自动构建 5 个平台二进制
   - 生成 SHA256 校验和
   - 创建 GitHub Release

4. **发布验证** (30 分钟)
   - 下载各平台二进制文件测试
   - 验证功能完整性
   - 检查文档链接正确性

5. **编写发布公告** (30 分钟)
   - 发布到 GitHub Discussions
   - 准备社交媒体宣传文案（微博、小红书）
   - 在摄影社群分享

**验收标准**:
- GitHub Release 页面存在 v0.2.0
- 5 个平台二进制文件可下载
- 所有下载链接和文档链接正常
- 发布公告完成

---

### 阶段二：iOS 原型开发准备（Week 4，3-5 天）

#### 任务 4：设计 FFI 接口层（1 天）
**优先级**: P0（iOS 开发前置）  
**目标**: 定义 Rust 与 Swift 的交互接口

**设计内容**:

1. **C ABI 函数签名设计**
   - `process_image_with_frame()`: 单张图片处理
   - `batch_process_images()`: 批量处理
   - `get_exif_data()`: 仅提取 EXIF
   - `list_templates()`: 获取模板列表
   - `error_handling()`: 错误信息传递

2. **数据结构定义**
   - C 兼容的 struct（`ExifData`, `ProcessOptions`）
   - 内存管理策略（所有权、释放）
   - 字符串编码处理（UTF-8）

3. **创建 ffi 模块**
   - 在 `src/ffi/mod.rs` 实现 C ABI 导出
   - 使用 `cbindgen` 生成 C header 文件
   - 编写内存安全封装

**验收标准**:
- 生成的 `.h` 文件可被 Swift 导入
- FFI 函数可从 C 代码调用
- 内存安全测试通过（无泄漏）

---

#### 任务 5：Rust → staticlib 构建配置（0.5 天）
**优先级**: P0  
**目标**: 将 Rust core 编译为 iOS 可用的静态库

**实现步骤**:

1. **配置 Cargo.toml**
   ```toml
   [lib]
   name = "litemark_core"
   crate-type = ["staticlib", "cdylib"]
   ```

2. **添加 iOS 目标**
   - `aarch64-apple-ios`（真机 ARM64）
   - `x86_64-apple-ios`（模拟器 x86_64）
   - `aarch64-apple-ios-sim`（模拟器 ARM64，M1 Mac）

3. **编写构建脚本**
   - `scripts/build_ios.sh`: 构建所有 iOS 目标
   - `scripts/create_xcframework.sh`: 合并为 XCFramework

4. **集成 cbindgen**
   - 自动生成 `litemark_core.h`
   - 配置 cbindgen.toml

**验收标准**:
- 执行 `./scripts/build_ios.sh` 成功生成 `.a` 文件
- 生成的 XCFramework 可被 Xcode 导入
- 所有架构构建成功

---

#### 任务 6：Swift Bridging 示例（1 天）
**优先级**: P1  
**目标**: 提供 Swift 调用 Rust 的示例代码

**实现内容**:

1. **创建 Swift Package**
   - 封装 FFI 调用
   - 提供 Swift-friendly API
   - 错误处理封装

2. **核心 API 设计**
   ```swift
   class LiteMarkCore {
       func processImage(inputPath: String, 
                         template: String, 
                         outputPath: String) throws
       func extractExifData(imagePath: String) throws -> ExifData
       func listTemplates() -> [String]
   }
   ```

3. **编写示例 App**
   - 单页面 SwiftUI 应用
   - 图片选择器
   - 调用 Rust 处理
   - 显示结果

**验收标准**:
- Swift Package 可被 Xcode 项目导入
- 示例 App 可在模拟器运行
- 图片处理功能正常

---

#### 任务 7：iOS App UI 设计（2 天）
**优先级**: P1  
**目标**: 实现 iOS 应用基础界面

**功能模块**:

1. **主界面**
   - 图片选择按钮（单张/多张）
   - 模板选择器（横向滚动卡片）
   - 预览区域
   - 处理按钮

2. **模板选择页**
   - 模板缩略图展示
   - 模板详情（参数说明）
   - 选中状态

3. **设置页**
   - 自定义作者名称
   - Logo 导入
   - 字体选择
   - 关于页面

4. **导出分享**
   - 保存到相册
   - 分享到社交媒体
   - AirDrop

**技术栈**:
- SwiftUI（界面）
- PhotoKit（相册访问）
- Combine（数据流）

**验收标准**:
- 界面流畅，交互自然
- 图片选择和预览正常
- 模板切换实时生效
- 导出功能正常

---

### 阶段三：Web WASM 版本（Month 2-3，选做）

#### 任务 8：WASM 绑定层开发（2-3 天）
**优先级**: P2（可延后）  
**目标**: 将 core 编译为浏览器可用的 WASM 模块

**实现步骤**:

1. **添加 wasm-bindgen 依赖**
2. **创建 JS 友好 API**
   - `processImageBlob(imageBlob, templateJSON)`: 处理图片 Blob
   - `extractExifFromBlob(imageBlob)`: 提取 EXIF
   - 异步 API 设计

3. **字体处理策略**
   - 使用 @font-face 加载字体
   - subset 压缩中文字体
   - 字体回退机制

4. **内存管理**
   - 限制单次处理图片大小
   - 分块处理大图
   - 手动 GC 触发

**验收标准**:
- `wasm-pack build` 成功
- 生成的 `.wasm` 文件可在浏览器加载
- 示例 HTML 页面正常工作

---

#### 任务 9：Web Demo 页面（1-2 天）
**优先级**: P2  
**目标**: 提供在线演示页面

**功能设计**:
- 拖拽上传图片
- 模板选择
- 实时预览（缩略图）
- 导出高质量图片

**技术选型**:
- 原生 JS 或 React/Vue
- 托管在 GitHub Pages

---

## 🎨 产品优化建议（中长期）

### 功能增强

#### 1. 智能布局避脸
**价值**: 提升用户体验，避免水印遮挡人物  
**实现方案**:
- iOS: 使用 Vision 框架检测人脸
- Web: 使用 TensorFlow.js face-api
- 根据人脸位置动态调整水印位置

#### 2. 模板商店
**价值**: 增加用户粘性，可选的变现渠道  
**实现方案**:
- 设计模板分享协议
- 构建模板社区平台
- 支持用户上传自定义模板

#### 3. HEIC/RAW 支持
**价值**: 覆盖更多专业摄影师需求  
**实现方案**:
- iOS: 使用系统解码 API
- Desktop: 集成 libheif、libraw

### 商业化策略

#### 免费版功能
- 基础模板（3-5 个）
- 单张处理
- 基本 EXIF 字段

#### 付费解锁（一次性买断 ¥29-69）
- 批量处理
- 高级模板包
- Logo 导入
- 无水印导出
- 优先技术支持

#### 企业授权（按需定价）
- API 接口调用
- 批量授权
- 定制化开发

---

## 🚨 风险与应对

### 技术风险

| 风险项               | 影响程度 | 应对策略                      |
| -------------------- | -------- | ----------------------------- |
| iOS FFI 集成复杂度高 | 高       | 提前技术验证，参考成熟案例    |
| WASM 大图内存限制    | 中       | 实现分块处理和降采样          |
| 中文字体体积过大     | 中       | 提供字体下载指南，使用 subset |
| HEIC 格式支持困难    | 低       | iOS 优先，Desktop 后期支持    |

### 商业风险

| 风险项         | 影响程度 | 应对策略                     |
| -------------- | -------- | ---------------------------- |
| 用户付费意愿低 | 中       | 保持良心定价，提供高价值功能 |
| 开源被滥用     | 低       | 官方 App 差异化，品牌建设    |
| 市场竞争激烈   | 中       | 强调隐私保护和本地处理       |

---

## 📊 时间线总览

```
当前时间点
    ↓
Week 3 剩余（1-2 天）
├── Logo 参数化
├── 测试完善
└── 发布 v0.2.0
    ↓
Week 4（3-5 天）
├── FFI 接口设计
├── iOS 静态库构建
└── Swift Bridging 示例
    ↓
Week 5-6（1-2 周）
├── iOS App UI 开发
├── TestFlight 内测
└── 收集用户反馈
    ↓
Month 2-3（选做）
├── WASM 版本开发
├── Web Demo 页面
└── 推广与优化
```

---

## 🎯 关键决策点

### 决策 1：是否立即开始 iOS 开发？
**建议**: 是  
**理由**: 
- Week 3 剩余工作量小（1-2 天）
- iOS 是核心目标平台，用户价值最高
- 技术栈成熟，风险可控

### 决策 2：WASM 版本优先级？
**建议**: 延后到 Month 2  
**理由**:
- iOS 原型更重要
- WASM 主要用于演示和传播
- 可在 iOS 稳定后再做

### 决策 3：何时开始收费模式？
**建议**: TestFlight 内测阶段即引入  
**理由**:
- 早期验证付费意愿
- 收集定价反馈
- 建立付费用户群

---

## 📝 立即行动清单（本周）

### 今天
- [ ] 实现 Logo 参数化（--logo 和 LITEMARK_LOGO）
- [ ] 为 renderer 模块添加单元测试

### 明天
- [ ] 完善 layout 和 io 模块测试
- [ ] 编写集成测试
- [ ] 完善 CHANGELOG.md

### 后天
- [ ] 创建 v0.2.0 标签并触发构建
- [ ] 验证发布产物
- [ ] 编写发布公告

### 本周末
- [ ] 开始设计 FFI 接口
- [ ] 编写 iOS 构建脚本
- [ ] 研究 XCFramework 集成

---

## 🎓 学习资源

### iOS FFI 集成
- [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
- [Building and Deploying a Rust library on iOS](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html)

### WASM 开发
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)

### SwiftUI
- [Apple SwiftUI Tutorials](https://developer.apple.com/tutorials/swiftui)
- [Hacking with Swift](https://www.hackingwithswift.com/quick-start/swiftui)

---

## 结论与建议

### 当前状态评估
- **MVP 质量**: 优秀 ✅（核心功能完整，代码质量高）
- **文档完善度**: 优秀 ✅（用户文档和开发文档齐全）
- **发布就绪度**: 良好 🚧（需完成 Week 3 剩余任务）

### 核心建议
1. **短期（本周）**: 专注完成 Week 3 剩余任务，发布 v0.2.0
2. **中期（下周开始）**: 全力推进 iOS 原型开发
3. **长期（2-3 个月后）**: 根据 iOS 反馈决定 WASM 和其他平台优先级

### 成功关键因素
- 保持 MVP 简洁，避免功能蔓延
- 及早发布，快速迭代
- 重视用户反馈，持续优化
- 坚持隐私优先和良心定价

---

**建议下一步行动**: 立即开始实现 Logo 参数化功能,这是 Week 3 最后的阻塞项,预计 1-2 小时即可完成。

---

## 附录：Logo 路径参数化功能详细设计

### 功能概述

为 LiteMark 添加 Logo 路径参数化能力，允许用户通过命令行参数或环境变量指定 Logo 文件路径，覆盖模板中的默认 Logo 配置。

### 设计目标

#### 核心目标
- 用户可通过 `--logo` CLI 参数指定 Logo 路径
- 支持 `LITEMARK_LOGO` 环境变量配置
- 保持与现有模板系统的向后兼容性
- Logo 路径支持绝对路径和相对路径

#### 优先级策略

参数优先级从高到低：
1. CLI 参数 `--logo`（最高优先级）
2. 环境变量 `LITEMARK_LOGO`
3. 模板中定义的 Logo 路径
4. 无 Logo（跳过 Logo 渲染）

### 用户交互设计

#### 命令行接口

**单张图片处理**
```bash
# 使用 CLI 参数指定 Logo
litemark add -i photo.jpg -t classic -o output.jpg --logo my_logo.png

# 指定绝对路径
litemark add -i photo.jpg -o output.jpg --logo /path/to/brand_logo.png

# 不指定 Logo（使用模板默认或环境变量）
litemark add -i photo.jpg -o output.jpg
```

**批量处理**
```bash
# 批量处理使用统一 Logo
litemark batch -i photos/ -t classic -o output/ --logo company_logo.png
```

#### 环境变量配置

**设置环境变量**
```bash
# Linux/macOS - 临时设置
export LITEMARK_LOGO="/Users/username/logos/my_logo.png"
litemark add -i photo.jpg -o output.jpg

# Linux/macOS - 永久设置（添加到 ~/.bashrc 或 ~/.zshrc）
echo 'export LITEMARK_LOGO="/path/to/default_logo.png"' >> ~/.zshrc

# Windows PowerShell
$env:LITEMARK_LOGO="C:\logos\my_logo.png"
litemark add -i photo.jpg -o output.jpg

# Windows CMD
set LITEMARK_LOGO=C:\logos\my_logo.png
litemark add -i photo.jpg -o output.jpg
```

### 系统架构设计

#### 模块职责划分

**main.rs（CLI 层）**
- 职责：接收和验证 Logo 参数
- 功能：
  - 解析 `--logo` CLI 参数
  - 读取 `LITEMARK_LOGO` 环境变量
  - 应用优先级策略
  - 将最终 Logo 路径传递给渲染层

**renderer/mod.rs（渲染层）**
- 职责：接收 Logo 路径并渲染
- 功能：
  - 接收外部传入的 Logo 路径参数
  - 覆盖模板中的 Logo 配置
  - 执行 Logo 图像加载和渲染

**layout/mod.rs（模板层）**
- 职责：保持现有模板结构不变
- 功能：
  - 模板中 Logo 路径作为默认值保留
  - 不修改现有模板结构

#### 数据流设计

```
用户输入（CLI 参数 / 环境变量）
        ↓
    main.rs
  （参数收集）
        ↓
   优先级判断
  CLI > ENV > Template
        ↓
  最终 Logo 路径
        ↓
process_single_image()
        ↓
 WatermarkRenderer
 .render_watermark()
        ↓
  render_frame_content()
   （Logo 路径覆盖）
        ↓
    render_logo()
   （加载并渲染）
```

### 核心实现设计

#### 1. CLI 参数定义

**位置**：`src/main.rs`

**修改内容**：在 `Commands` 枚举的 `Add` 和 `Batch` 分支中添加 `logo` 字段

**数据结构**：
- 字段名：`logo`
- 类型：`Option<String>`
- 参数名：`--logo`
- 说明：Logo file path (overrides template and environment variable)

**预期效果**：
- 用户可通过 `--logo <path>` 指定 Logo
- 参数可选，不指定时为 `None`
- clap 自动处理参数解析和验证

#### 2. Logo 路径解析逻辑

**位置**：`src/main.rs` 的 `process_single_image()` 和 `process_single_image_in_batch()` 函数

**逻辑流程**：

```
开始
  ↓
检查 CLI 参数 logo
  ↓
是否存在？
  ├─ 是 → 使用 CLI 参数
  └─ 否 → 继续
       ↓
  检查环境变量 LITEMARK_LOGO
       ↓
  是否存在？
    ├─ 是 → 使用环境变量
    └─ 否 → 使用模板默认值或无 Logo
         ↓
      最终 Logo 路径
```

**优先级实现**：
```
let final_logo_path: Option<String> = match (cli_logo, env_logo) {
    (Some(cli), _) => Some(cli),           // CLI 参数最高优先级
    (None, Some(env)) => Some(env),        // 环境变量次优先级
    (None, None) => None,                  // 无外部指定，使用模板默认
}
```

#### 3. 渲染器接口扩展

**位置**：`src/renderer/mod.rs`

**方法签名修改**：

原有签名：
```
pub fn render_watermark(
    &self,
    image: &mut DynamicImage,
    template: &Template,
    variables: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>>
```

新增参数设计：
```
pub fn render_watermark(
    &self,
    image: &mut DynamicImage,
    template: &Template,
    variables: &HashMap<String, String>,
    logo_override: Option<&str>,  // 新增：外部 Logo 路径覆盖
) -> Result<(), Box<dyn std::error::Error>>
```

**内部处理逻辑**：

在 `render_frame_content()` 方法中：

```
1. 检查 logo_override 参数
   ↓
2. 如果 logo_override 存在
   - 使用 logo_override 作为 Logo 路径
   - 忽略模板中的 Logo 配置
   ↓
3. 如果 logo_override 为 None
   - 从模板 items 中查找 Logo 类型项
   - 使用模板中的 Logo 路径
   ↓
4. 如果两者都不存在
   - 跳过 Logo 渲染
```

#### 4. Logo 路径验证

**验证时机**：在调用 `render_logo()` 之前

**验证内容**：
- 文件是否存在
- 文件扩展名是否为支持的图像格式（png, jpg, jpeg, svg 等）
- 文件是否可读

**错误处理策略**：

| 错误类型          | 处理方式         | 用户提示                                              |
| ----------------- | ---------------- | ----------------------------------------------------- |
| Logo 文件不存在   | 警告并跳过 Logo  | "⚠️  Logo file not found: {path}, skipping logo"       |
| Logo 文件无法读取 | 警告并跳过 Logo  | "⚠️  Cannot read logo file: {path}, skipping logo"     |
| Logo 格式不支持   | 警告并跳过 Logo  | "⚠️  Logo format not supported: {path}, skipping logo" |
| Logo 加载失败     | 警告并使用占位符 | "⚠️  Failed to load logo, using placeholder"           |

**设计原则**：
- Logo 错误不应导致整个水印处理失败
- 给出明确的警告信息帮助用户排查问题
- 优雅降级（跳过 Logo 或使用占位符）

### 模板兼容性设计

#### 现有模板行为

**ClassicParam 模板**：
- 当前：Logo 项的 value 为空字符串
- 参数化后：
  - 如果用户指定 `--logo`，使用用户指定的路径
  - 如果未指定，value 为空时跳过 Logo 渲染

**自定义模板**：
- 模板中指定了 Logo 路径：作为默认值
- 用户可通过 `--logo` 覆盖

#### 向后兼容性保证

**兼容性原则**：
- 不修改现有模板 JSON 结构
- 现有模板继续正常工作
- 新参数为可选，不影响现有用户

**兼容性测试场景**：

| 场景 | 模板 Logo | CLI --logo | 环境变量 | 最终结果      |
| ---- | --------- | ---------- | -------- | ------------- |
| 1    | 有路径    | 未指定     | 未设置   | 使用模板路径  |
| 2    | 有路径    | 指定       | 未设置   | 使用 CLI 参数 |
| 3    | 有路径    | 未指定     | 已设置   | 使用环境变量  |
| 4    | 空字符串  | 未指定     | 未设置   | 跳过 Logo     |
| 5    | 空字符串  | 指定       | 未设置   | 使用 CLI 参数 |
| 6    | 空字符串  | 未指定     | 已设置   | 使用环境变量  |

### 用户体验设计

#### 命令行帮助信息

**add 命令帮助**：
```
Add watermark to a single image

Usage: litemark add [OPTIONS] -i <input> -o <output>

Options:
  -i, --input <input>        Input image path
  -t, --template <template>  Template name or path [default: classic]
  -o, --output <output>      Output image path
      --author <author>      Author name (overrides EXIF data)
      --font <font>          Custom font file path
      --logo <logo>          Logo file path (overrides template and env)
  -h, --help                 Print help
```

**batch 命令帮助**：
```
Batch process images in a directory

Usage: litemark batch [OPTIONS] -i <input-dir> -o <output-dir>

Options:
  -i, --input-dir <input-dir>    Input directory path
  -t, --template <template>      Template name or path [default: classic]
  -o, --output-dir <output-dir>  Output directory path
      --author <author>          Author name (overrides EXIF data)
      --font <font>              Custom font file path
      --logo <logo>              Logo file path (overrides template and env)
  -h, --help                     Print help
```

#### 日志输出设计

**成功场景日志**：
```
Processing image: photo.jpg
Loaded image: 4032x3024
Extracted EXIF data: ExifData { iso: Some("100"), ... }
Using template: ClassicParam
Using custom logo: brand_logo.png          ← 新增日志
Using default embedded font
Saved watermarked image: output.jpg
```

**环境变量日志**：
```
Processing image: photo.jpg
Loaded image: 4032x3024
Extracted EXIF data: ExifData { ... }
Using template: ClassicParam
Using logo from environment: /path/to/logo.png  ← 新增日志
Saved watermarked image: output.jpg
```

**无 Logo 场景日志**：
```
Processing image: photo.jpg
Loaded image: 4032x3024
Extracted EXIF data: ExifData { ... }
Using template: Minimal
No logo specified, skipping logo rendering      ← 新增日志
Saved watermarked image: output.jpg
```

### 错误处理设计

#### 错误分类与处理策略

**非致命错误（警告）**：
- Logo 文件不存在
- Logo 文件格式不支持
- Logo 加载失败

处理方式：输出警告，跳过 Logo 渲染，继续处理水印

**致命错误（中止）**：
- 输入图片不存在或无法读取
- 输出路径无写入权限
- 模板格式错误

处理方式：输出错误信息，终止当前图片处理

#### 错误信息示例

**Logo 文件不存在**：
```
Processing image: photo.jpg
⚠️  Warning: Logo file not found: 'missing_logo.png'
    Skipping logo rendering and continuing...
Saved watermarked image: output.jpg
```

**Logo 格式不支持**：
```
Processing image: photo.jpg
⚠️  Warning: Logo format not supported: 'logo.bmp'
    Supported formats: PNG, JPEG, GIF, WebP
    Skipping logo rendering and continuing...
Saved watermarked image: output.jpg
```

### 实现步骤详解

#### 步骤 1：修改 CLI 参数定义（10 分钟）

**文件**：`src/main.rs`

**修改位置**：`Commands` 枚举

**修改内容**：
在 `Add` 和 `Batch` 两个分支中添加 `logo` 字段

**字段定义**：
- 名称：`logo`
- 类型：`Option<String>`
- 属性标注：`#[arg(long)]`
- 注释：`/// Logo file path (overrides template and environment variable)`

**验证方式**：
运行 `cargo run -- add --help` 检查帮助信息中是否出现 `--logo` 参数

#### 步骤 2：实现 Logo 路径解析逻辑（15 分钟）

**文件**：`src/main.rs`

**修改函数**：
- `process_single_image()`
- `process_single_image_in_batch()`

**新增参数**：
在两个函数签名中添加 `logo: Option<&str>` 参数

**解析逻辑**：
```
在函数内部添加：

1. 读取环境变量
   let env_logo = std::env::var("LITEMARK_LOGO").ok();

2. 应用优先级策略
   let final_logo: Option<String> = match (logo, env_logo) {
       (Some(cli), _) => {
           println!("Using custom logo: {}", cli);
           Some(cli.to_string())
       },
       (None, Some(env)) => {
           println!("Using logo from environment: {}", env);
           Some(env)
       },
       (None, None) => None,
   };

3. 传递给渲染器
   renderer.render_watermark(
       &mut image, 
       &template, 
       &variables, 
       final_logo.as_deref()
   )?;
```

**验证方式**：
- 添加日志输出验证逻辑正确性
- 确保参数正确传递到渲染层

#### 步骤 3：修改渲染器接口（20 分钟）

**文件**：`src/renderer/mod.rs`

**修改方法**：`render_watermark()`

**参数调整**：
添加 `logo_override: Option<&str>` 参数到方法签名

**调用链更新**：
将 `logo_override` 传递给 `render_frame_content()` 方法

**render_frame_content() 修改**：

```
在函数签名中添加参数：
logo_override: Option<&str>

在 Logo 处理部分：

// 原有逻辑：从模板中提取 Logo 路径
let mut logo_path: Option<String> = None;
for item in template.items.iter() {
    match item.item_type {
        ItemType::Logo => {
            logo_path = Some(item.value.clone());
        }
        ...
    }
}

// 新增逻辑：应用 logo_override
let final_logo_path = match logo_override {
    Some(override_path) if !override_path.is_empty() => {
        Some(override_path.to_string())
    },
    _ => logo_path,  // 使用模板中的路径或 None
};

// 使用 final_logo_path 进行渲染
if let Some(ref logo_path) = final_logo_path {
    if !logo_path.is_empty() {
        let logo_y = frame_y + height / 2 - logo_size / 2;
        self.render_logo(...)?;
    } else {
        println!("Logo path is empty, skipping logo rendering");
    }
}
```

**验证方式**：
编译检查，确保所有调用点都更新了参数

#### 步骤 4：更新所有调用点（10 分钟）

**需要更新的位置**：

1. `src/main.rs` 中的 `process_single_image()`
2. `src/main.rs` 中的 `process_single_image_in_batch()`
3. 确保 `main()` 函数正确传递 `logo` 参数

**更新内容**：
- `Commands::Add` 分支：传递 `logo.as_deref()`
- `Commands::Batch` 分支：传递 `logo.as_deref()`

**验证方式**：
运行 `cargo build` 确保无编译错误

#### 步骤 5：增强错误处理和日志（15 分钟）

**文件**：`src/renderer/mod.rs`

**修改方法**：`render_logo()`

**错误处理增强**：

```
在 render_logo() 方法开始处添加验证：

// 1. 检查文件是否存在
let logo_path_obj = std::path::Path::new(logo_path);
if !logo_path_obj.exists() {
    println!("⚠️  Warning: Logo file not found: '{}'", logo_path);
    println!("    Skipping logo rendering and continuing...");
    return Ok(());  // 不渲染 Logo，但继续处理
}

// 2. 检查文件扩展名
if let Some(ext) = logo_path_obj.extension() {
    let ext_str = ext.to_string_lossy().to_lowercase();
    let supported = ["png", "jpg", "jpeg", "gif", "webp"];
    if !supported.contains(&ext_str.as_str()) {
        println!("⚠️  Warning: Logo format may not be supported: '{}'", ext_str);
        println!("    Supported formats: PNG, JPEG, GIF, WebP");
    }
}

// 3. 尝试加载图片（现有逻辑已有处理）
if let Ok(logo_img) = image::open(logo_path) {
    // 正常渲染逻辑
    ...
} else {
    println!("⚠️  Warning: Failed to load logo: '{}'", logo_path);
    println!("    Skipping logo rendering and continuing...");
    return Ok(());
}
```

**日志增强**：
在成功加载 Logo 时添加日志：
```
println!("✓ Logo loaded successfully: {}", logo_path);
```

**验证方式**：
- 测试不存在的 Logo 文件
- 测试不支持的格式
- 验证日志输出正确

#### 步骤 6：编写测试用例（20 分钟）

**文件**：`src/renderer/mod.rs` 或新建 `tests/logo_override_test.rs`

**测试场景**：

1. **测试 CLI 参数优先级**
   - 设置环境变量
   - 传入 CLI 参数
   - 验证使用 CLI 参数

2. **测试环境变量生效**
   - 不传 CLI 参数
   - 设置环境变量
   - 验证使用环境变量

3. **测试模板默认值**
   - 不传 CLI 参数
   - 不设置环境变量
   - 模板有 Logo 路径
   - 验证使用模板路径

4. **测试无 Logo 场景**
   - 所有来源都无 Logo
   - 验证跳过 Logo 渲染

5. **测试 Logo 文件不存在**
   - 传入不存在的文件路径
   - 验证输出警告
   - 验证继续处理水印

**测试代码结构**：
```
#[cfg(test)]
mod logo_override_tests {
    use super::*;

    #[test]
    fn test_cli_logo_overrides_env() {
        // 设置环境变量
        std::env::set_var("LITEMARK_LOGO", "env_logo.png");
        
        // 模拟 CLI 参数
        let cli_logo = Some("cli_logo.png");
        
        // 应用优先级逻辑
        let env_logo = std::env::var("LITEMARK_LOGO").ok();
        let final_logo = match (cli_logo, env_logo) {
            (Some(cli), _) => Some(cli.to_string()),
            (None, Some(env)) => Some(env),
            _ => None,
        };
        
        // 验证结果
        assert_eq!(final_logo, Some("cli_logo.png".to_string()));
        
        // 清理环境变量
        std::env::remove_var("LITEMARK_LOGO");
    }
    
    #[test]
    fn test_env_logo_when_no_cli() {
        std::env::set_var("LITEMARK_LOGO", "env_logo.png");
        let cli_logo: Option<&str> = None;
        
        let env_logo = std::env::var("LITEMARK_LOGO").ok();
        let final_logo = match (cli_logo, env_logo) {
            (Some(cli), _) => Some(cli.to_string()),
            (None, Some(env)) => Some(env),
            _ => None,
        };
        
        assert_eq!(final_logo, Some("env_logo.png".to_string()));
        std::env::remove_var("LITEMARK_LOGO");
    }
    
    #[test]
    fn test_no_logo_when_all_none() {
        std::env::remove_var("LITEMARK_LOGO");
        let cli_logo: Option<&str> = None;
        
        let env_logo = std::env::var("LITEMARK_LOGO").ok();
        let final_logo = match (cli_logo, env_logo) {
            (Some(cli), _) => Some(cli.to_string()),
            (None, Some(env)) => Some(env),
            _ => None,
        };
        
        assert_eq!(final_logo, None);
    }
}
```

**验证方式**：
运行 `cargo test logo_override` 确保所有测试通过

#### 步骤 7：集成测试（10 分钟）

**测试命令**：

```bash
# 1. 准备测试资源
cp test_images/800x600_landscape.jpg test_photo.jpg
cp path/to/test_logo.png test_logo.png

# 2. 测试 CLI 参数
cargo run -- add -i test_photo.jpg -t classic -o output_cli.jpg --logo test_logo.png

# 3. 测试环境变量
export LITEMARK_LOGO="test_logo.png"
cargo run -- add -i test_photo.jpg -t classic -o output_env.jpg
unset LITEMARK_LOGO

# 4. 测试不存在的 Logo（应输出警告但成功完成）
cargo run -- add -i test_photo.jpg -t classic -o output_nologo.jpg --logo missing.png

# 5. 测试批量处理
mkdir -p test_batch_output
cargo run -- batch -i test_images -t classic -o test_batch_output --logo test_logo.png

# 6. 验证输出图片
open output_cli.jpg  # macOS
# 或
xdg-open output_cli.jpg  # Linux
```

**验证要点**：
- CLI 参数指定的 Logo 正确渲染
- 环境变量指定的 Logo 正确渲染
- 不存在的 Logo 输出警告但不崩溃
- 批量处理所有图片使用统一 Logo
- 日志输出清晰准确

#### 步骤 8：文档更新（10 分钟）

**需要更新的文档**：

1. **README.md**
   - 在 Usage 部分添加 `--logo` 参数示例
   - 在 Features 部分提及 Logo 参数化
   - 添加环境变量配置说明

2. **examples/basic_usage.md**（如果存在）
   - 添加 Logo 自定义示例
   - 展示不同 Logo 的效果对比

3. **CHANGELOG_v0.2.0.md**
   - 在新功能部分添加 Logo 参数化条目

**文档内容示例**（README.md）：

```markdown
### Logo Customization

You can specify a custom logo for your watermarks:

**Using command-line parameter:**
```bash
litemark add -i photo.jpg -o output.jpg --logo my_logo.png
```

**Using environment variable:**
```bash
export LITEMARK_LOGO="/path/to/default_logo.png"
litemark add -i photo.jpg -o output.jpg
```

**Priority order:**
1. `--logo` CLI parameter (highest priority)
2. `LITEMARK_LOGO` environment variable
3. Logo path defined in template
4. No logo (skip logo rendering)

**Supported logo formats:**
- PNG (recommended for transparency)
- JPEG
- GIF
- WebP
```

**验证方式**：
检查文档格式正确，示例可执行

### 质量保证检查清单

#### 功能测试

- [ ] CLI 参数 `--logo` 正确解析
- [ ] 环境变量 `LITEMARK_LOGO` 正确读取
- [ ] 优先级策略正确执行（CLI > ENV > Template）
- [ ] Logo 文件存在时正确渲染
- [ ] Logo 文件不存在时输出警告并跳过
- [ ] Logo 路径为空时跳过渲染
- [ ] 绝对路径和相对路径都支持
- [ ] 批量处理使用统一 Logo
- [ ] 单张处理 Logo 参数正常

#### 兼容性测试

- [ ] 现有模板（ClassicParam、Modern、Minimal）正常工作
- [ ] 不使用 `--logo` 参数时行为不变
- [ ] 模板中有 Logo 路径时作为默认值
- [ ] 模板中无 Logo 路径时不崩溃

#### 错误处理测试

- [ ] 不存在的 Logo 文件：警告并继续
- [ ] 无读权限的 Logo 文件：警告并继续
- [ ] 损坏的 Logo 文件：警告并继续
- [ ] 不支持的 Logo 格式：警告并尝试加载

#### 用户体验测试

- [ ] 帮助信息清晰准确
- [ ] 日志输出信息完整
- [ ] 警告信息易于理解
- [ ] 成功场景有明确反馈

#### 代码质量检查

- [ ] 无编译警告
- [ ] 所有测试通过
- [ ] 代码符合 Rust 规范（运行 `cargo clippy`）
- [ ] 代码格式化正确（运行 `cargo fmt`）
- [ ] 文档注释完整

#### 性能检查

- [ ] Logo 加载不影响整体性能
- [ ] 批量处理时 Logo 加载优化（考虑缓存）
- [ ] 内存使用无明显增长

### 预期成果

#### 用户价值

1. **灵活性提升**
   - 用户可轻松切换不同 Logo
   - 批量处理时统一品牌标识
   - 不同项目使用不同 Logo

2. **易用性增强**
   - 命令行参数直观
   - 环境变量方便默认配置
   - 向后兼容无学习成本

3. **可靠性保证**
   - Logo 问题不影响水印处理
   - 清晰的错误提示
   - 优雅降级

#### 技术收益

1. **架构优化**
   - 参数传递链路清晰
   - 职责分离明确
   - 扩展性良好

2. **代码质量**
   - 测试覆盖全面
   - 错误处理健壮
   - 文档完善

3. **可维护性**
   - 逻辑简洁
   - 易于调试
   - 便于后续扩展

### 后续优化方向

#### 短期优化（可选）

1. **Logo 缓存机制**
   - 批量处理时缓存已加载的 Logo
   - 避免重复加载相同文件
   - 提升批量处理性能

2. **Logo 格式自动转换**
   - 支持更多图像格式
   - 自动处理颜色空间
   - 透明度优化

3. **Logo 位置微调**
   - 允许用户指定 Logo 偏移量
   - 支持 Logo 缩放比例调整
   - 更灵活的布局控制

#### 长期规划

1. **Logo 库管理**
   - 内置常用品牌 Logo
   - 用户 Logo 库管理
   - Logo 预设配置

2. **Logo 特效**
   - 阴影效果
   - 边框效果
   - 滤镜效果

3. **智能 Logo 适配**
   - 根据背景自动调整 Logo 颜色
   - 自动选择最佳 Logo 位置
   - Logo 大小智能缩放

### 实施时间表

| 步骤              | 预计时间 | 累计时间 |
| ----------------- | -------- | -------- |
| 1. CLI 参数定义   | 10 分钟  | 10 分钟  |
| 2. Logo 路径解析  | 15 分钟  | 25 分钟  |
| 3. 渲染器接口扩展 | 20 分钟  | 45 分钟  |
| 4. 更新调用点     | 10 分钟  | 55 分钟  |
| 5. 错误处理和日志 | 15 分钟  | 70 分钟  |
| 6. 编写测试用例   | 20 分钟  | 90 分钟  |
| 7. 集成测试       | 10 分钟  | 100 分钟 |
| 8. 文档更新       | 10 分钟  | 110 分钟 |

**总计**：约 2 小时（包含测试和文档）

### 风险与应对

| 风险                       | 概率 | 影响 | 应对措施                     |
| -------------------------- | ---- | ---- | ---------------------------- |
| 参数传递链路复杂导致遗漏   | 低   | 中   | 仔细检查所有调用点，编译验证 |
| Logo 格式兼容性问题        | 中   | 低   | 增强错误处理，优雅降级       |
| 环境变量在不同 OS 表现不一 | 低   | 低   | 文档说明不同平台配置方法     |
| 测试覆盖不全面             | 低   | 中   | 按测试清单逐项验证           |

### 成功标准

#### 必须满足（Must Have）

- ✅ `--logo` CLI 参数正常工作
- ✅ `LITEMARK_LOGO` 环境变量正常工作
- ✅ 优先级策略正确执行
- ✅ 向后兼容现有模板
- ✅ Logo 错误不导致处理失败
- ✅ 所有测试通过
- ✅ 文档更新完整

#### 应该满足（Should Have）

- ✅ 日志输出清晰
- ✅ 错误提示友好
- ✅ 帮助信息完善
- ✅ 测试覆盖全面

#### 可以满足（Could Have）

- Logo 缓存优化
- Logo 格式自动转换
- 更多 Logo 特效

---

**设计完成时间**：已完成
**设计评审状态**：待用户确认
**下一步行动**：用户确认后立即开始实现
