# LiteMark 技术 + 产品路线图 🚀

> **最近更新:** 2025-11-10  
> **当前版本:** v0.1.0  
> **下一版本:** v0.2.0 (准备发布中)

本文档为 LiteMark（轻量摄影参数水印工具）的完整路线图，包含产品目标、功能规划、技术架构、里程碑时间表、发布策略与风险缓解。

⸻

1) 项目定位与核心目标

产品名（临时）：LiteMark（照片参数水印工具）
定位：为轻度摄影爱好者与社媒用户提供“美观、隐私友好、易用”的照片参数水印（ISO/Shutter/Aperture/焦距/时间/作者等）。
价值主张：一次性买断或免费 + 高质量模板，避免订阅绑架，隐私优先（本地处理）。

关键指标（初期）：
	•	1 个月内有可用 CLI + iOS 原型
	•	首月 100+ 试用用户（由摄影群、微博/小红书推广）
	•	转化率（付费解锁）目标：5–10%（若有买断版）

⸻

2) 目标用户与使用场景
	•	手机摄影爱好者（iOS 优先）
	•	社交媒体发布者（Instagram / 小红书 / 微博）
	•	业余摄影师（展示拍摄参数、风格）
	•	希望批量加参数水印的用户（可选）

典型场景：
	•	发图到社媒，想展示拍摄参数与署名
	•	批量给一组照片添加统一参数水印（快速导出）
	•	本地保护版权/署名，不上传到云

⸻

3) 产品分层 — 功能优先级（MVP → v1 → v2）

### MVP（核心功能）✅ **已完成 100%**

**完成时间:** Week 1-2 (2025-10)

核心功能清单：
- ✅ CLI 工具：读取图片（JPEG/PNG/TIFF）、提取 EXIF、生成带相框的新图片
- ✅ **真实 EXIF 提取**：集成 kamadak-exif，支持 ISO、光圈、快门、焦距、相机、镜头、时间、作者
- ✅ 相框模式：底部相框显示参数和 logo（logo 居中，参数下方显示）
- ✅ 字体渲染：使用 rusttype 实现专业字体渲染，支持多语言
  - ✅ 内置 DejaVu Sans（英文）
  - ✅ 支持 `--font` 参数和 `LITEMARK_FONT` 环境变量（中文字体）
- ✅ Logo 支持：自动加载和缩放 logo 图片
  - ✅ 支持 `--logo` CLI 参数
  - ✅ 支持 `LITEMARK_LOGO` 环境变量
  - ✅ 优先级策略：CLI > ENV > Template
- ✅ 模板系统：JSON 配置，支持变量替换 ({ISO}, {Aperture}, {Shutter} 等)
- ✅ 批量处理：目录遍历，自动处理多张照片
- ✅ 高质量输出：保持原图分辨率
- ✅ 开源协议：MIT License
- ✅ 单元测试：EXIF、布局、渲染模块测试覆盖
- 🚧 CI/CD：GitHub Actions 配置中

### v0.2.0（完善与发布）🚧 **进行中 60%**

**目标完成时间:** Week 3-4 (2025-11)

优化任务清单：
- ✅ Logo 路径参数化（CLI 参数 + 环境变量）
- ✅ 完善 EXIF 提取（真实数据、错误处理、快门格式化）
- ✅ 文档完善（README、架构文档、使用指南）
- ✅ 中文字体配置指南
- 🚧 完善测试覆盖（renderer、layout 模块）
- 🚧 CI/CD 配置（GitHub Actions）
  - 🚧 自动化测试工作流
  - 🚧 跨平台编译（Linux/macOS/Windows）
  - 🚧 自动发布 Release
- 🚧 发布准备
  - 🚧 编写 CHANGELOG.md
  - 🚧 创建 GitHub Release v0.2.0
  - 🚧 发布二进制文件

### v1.0（iOS 原型 + UX）📅 **计划中**

**目标完成时间:** 1-2 个月

功能规划：
- iOS 原生 App（SwiftUI）调用 Rust core
  - 单张预览与批量队列
  - 实时预览（小图缩放后渲染）
- 自定义摄影师签名与 logo 导入
- 模板管理
  - 保存/删除/重命名模板
  - 模板参数调整（颜色、字体大小、位置）
- 导出分享
  - 保存到相册
  - 直接分享到社媒
- 一次性买断解锁（内购）
  - 批量处理
  - 高级模板
  - 自定义模板

### v2.0（跨平台 + 智能功能）📅 **计划中**

**目标完成时间:** 3-6 个月

功能规划：
- Web Demo（WASM）
  - 本地浏览器处理（无上传）
  - 拖拽上传、实时预览
  - 方便传播和体验
- Desktop GUI
  - macOS / Windows 原生应用
  - 或基于 Tauri/Electron
- 智能布局
  - 避开人脸/主体（基于 iOS Vision / wasm face detect）
  - 自动选择最佳水印位置
- 模板市场
  - 导入/分享自定义模板
  - 社区模板库
  - 志愿付费或模板包
- 企业功能（按需）
  - 批量 API
  - 自定义品牌模板
  - 批量授权

⸻

4) 技术选型（核心建议）

核心语言（推荐）：Rust
原因：高性能、内存安全、成熟的 WASM 支持、良好的图像处理生态（image, imageproc, resvg, rusttype/font-kit 等），方便编译成 native lib + wasm。

替代：Go（实现更快，但 wasm 支持/字体渲染相对弱）。
CLI 框架：clap（Rust）
EXIF 解析：rexiv2、exif（Rust crates）或使用 libexif via FFI（必要时）。
字体/排版：font-kit / fontdue / rusttype（注意中文字体处理）。
图像处理：image, imageproc, resize（高质量缩放）。
WASM 运行：wasm-bindgen / wasm-pack（Web），wasm for embedding（Wasmtime/Wasmer 可用于 native hosts if needed）。
iOS 集成：两条主路径
	•	推荐：Rust -> staticlib（.a） + C ABI -> Swift Bridging（高性能、稳定）
	•	备选：WASM 内嵌到 WKWebView 或 SwiftWasm（方便但可能性能、包体限制）

打包与分发：GitHub Releases + Homebrew Tap（macOS CLI）+ Scoop/Chocolatey（Windows）。iOS 通过 TestFlight -> App Store。

⸻

5) Core 架构设计（模块化）

core/
 ├─ cli/                # CLI entry (clap)
 ├─ exif_reader/        # 读取与标准化 EXIF 数据
 ├─ layout_engine/      # 模板解析与排版（支持参数占位）
 ├─ renderer/           # 图像绘制、字体、logo 合成
 ├─ io/                 # 图片解码/编码（HEIC 支持策略）
 ├─ wasm_bindings/      # wasm 绑定层
 └─ ffi/                 # C ABI 导出（供 iOS/macOS 使用）

核心职责（已实现）：
	•	exif_reader：EXIF 解析（占位符实现，待完善真实 EXIF 读取）。返回统一结构体（iso, aperture, shutter, focal, camera, lens, date_time, author）。
	•	layout：模板 JSON 解析、变量替换（{ISO}、{Aperture}、{Time}、{Author}等）。
	•	renderer：相框模式渲染、rusttype 字体渲染（支持中文），logo 加载和缩放，底部相框生成。
	•	io：图片加载/保存，批量处理目录遍历。
	
待实现：
	•	wasm_bindings：暴露 JS-friendly 函数：processImage(inputBlob, templateJSON) -> outputBlob。
	•	ffi：C ABI：process(path_in, path_out, template_json_cstr)（用于 iOS 集成）。

⸻

6) 模板系统（设计要点）
	•	模板使用 JSON/YAML 描述：布局（anchor）、字体大小规则（相对/绝对）、文字线条、阴影、背景遮罩（半透明矩形）、logo 路径、可选 QR 码（指向个人主页）。
	•	支持 “变量映射” 与 “占位格式化”（例如时间格式化）。
	•	对中文/英文做字体回退策略（中文需打包或提供字体下载提示）。
	•	提供 5-10 个内置模板（摄影感、极简、社媒封面、左下参数条、右下角小签名等）。

模板示例（当前实现）：

{
  "name": "ClassicParam",
  "anchor": "bottom-left",
  "padding": 0,
  "items": [
    {"type": "logo", "value": "path/to/logo.png"},
    {"type": "text", "value": "{Author}", "font_size": 20, "color": "#000000"},
    {"type": "text", "value": "{Aperture} | ISO {ISO} | {Shutter}", "font_size": 16, "color": "#000000"}
  ]
}

相框布局：
- 底部添加 100px 白色相框
- Logo 居中显示（相框上半部分）
- 摄影师姓名和参数文字（相框下半部分，居中显示）


⸻

7) iOS 集成方案（重点）

推荐实现：Rust 编译成 staticlib (.a) + C ABI + Swift bridging（通过 modulemap 或 C wrapper）。
优点：性能最好、内存控制、无 JS 层开销，便于直接在 SwiftUI 中调用并即时展示预览。

iOS UI 功能（MVP）：
	•	图片选择（PhotoKit），单张/多张选择
	•	实时预览（小图缩放后渲染）
	•	模板选择/自定义（文字、签名、位置）
	•	导出分享（保存到相册 / 直接分享到社媒）
	•	内购：一次性解锁（批量/高级模板）

注意点：
	•	HEIC 的本地支持（iOS 很友好，直接使用 UIImage/CGImage 解码并传到 core）
	•	字体：打包基础英文字体 + 提示用户安装或内置小体积中文字体（需注意授权）
	•	内购策略：一次性买断解锁“批量处理 + 自定义模板”。

⸻

8) WASM & Web 方案

目标：一个无需安装的演示页面（Web），用户拖图本地处理，体验核心功能，提升传播。

实现：
	•	用 wasm-pack + wasm-bindgen 将 core 编译为 wasm + JS wrapper。
	•	Web UI 只做演示与模板选择，所有处理在浏览器内（FileReader -> wasm -> createObjectURL 或 下载）。
	•	注意点：字体需通过 @font-face 加载（受 CORS）；中文字体体积问题（可使用 subset/woff2）；内存与大图限制需要流式或按分辨率处理（先缩略预览，导出时做 full-res）。

⸻

9) HEIC/RAW 支持策略
	•	iOS 端：使用系统解码（UIImage/CGImage），传 RGBA bitmap 到 core。
	•	CLI / Desktop：优先支持 JPEG/PNG。HEIC 支持通过 libheif（FFI）或调用系统工具（macOS 下用 CoreImage）。RAW（.CR2 等）可以后续扩展（复杂且大，先不做）。

⸻

10) 性能优化要点
	•	对大图做 tile/stream 渲染或先生成缩略预览。导出时逐步处理避免 OOM。
	•	使用高质量但有 SIMD 加速的缩放库（image crate + simd feature）。
	•	字体渲染缓存 glyph atlas，避免重复布局。
	•	并行处理批量（限制线程数，遵守移动端 CPU 限制）。

⸻

11) 测试、CI、发布
	•	单元测试：EXIF 解析、模板变量替换、渲染结果比对（可用小图片快照测试）。
	•	集成测试：CLI end-to-end（sample images）。
	•	CI：GitHub Actions → 构建 release binaries（macOS/Linux/Windows）、wasm artifacts、iOS XCFramework（通过 cargo-lipo / cbindgen + xcodebuild）。
	•	发布：GitHub Releases、Homebrew Tap、TestFlight -> App Store。

⸻

12) 开源 & 授权建议
	•	Core（CLI + renderer）开源：MIT / Apache-2.0（鼓励贡献、方便第三方集成）。
	•	UI（iOS App）可以闭源或双许可证：App 开源会降低商业化，建议 iOS App 作为官方闭源二进制（或带商业模板/内购）。
	•	模板可设为免费内置 + 商业模板包（可付费）。

⸻

13) 收费与变现策略（避免过度付费）

优先策略（以“良心工具”为定位）：
	•	免费版：基础模板、单张导出、署名功能。
	•	一次性买断（App 内购，非订阅）：批量处理、导出无压缩、定制模板、logo 导入。 建议定价区间：¥29–¥69（iOS 中国区可考虑 ¥29 起；国外 App Store 对应 $4.99–$9.99）。
	•	模板商店（后期）：志愿付费 / 模板包（一次性）。
	•	企业授权 / API（长期）：面对商家、第三方应用按量收费。

不要做订阅；明确一次性解锁的价值点（节省时间、批量效率、模板美观）。

⸻

14) 隐私与合规
	•	默认完全本地处理（Web 使用 wasm 在客户端，不上传服务器）。
	•	若提供云渲染/API（未来），需要明确告知并选 opt-in。
	•	日志仅收集匿名崩溃与使用统计（可选）。提供隐私政策并在 App 中显著告知。

⸻

15) 推广策略（起步）
	•	在摄影小圈子试用（微博摄影群、小红书、摄影论坛、微信摄影群）。
	•	做几个“教学帖”：展示如何用参数水印提升专业感（对比图）。
	•	提供 10 个精美模板免费（吸引用户试用）。
	•	在 GitHub 设置说明文档 & demo：paramark --help + demo images。
	•	上架 iOS 时利用 ASO（关键词：摄影参数、水印、EXIF、签名、批量）。

⸻

16) 风险清单与应对
	•	字体授权/中文字体体积：只打包开源字体或提示用户导入；可用 subset 分发。
	•	WASM 大图内存 & 浏览器限制：先做缩略预览、分块/降采样导出，限制单次处理大小并向用户提示。
	•	HEIC/RAW 兼容问题：iOS 原生处理优先；桌面后期支持 libheif。
	•	商业化难度：保持良心定位，先聚集用户口碑，再做小额买断与模板付费。
	•	开源被滥用（例如有人改包并收费）：选择宽松许可证并在 App 中做差异化（闭源模板/商店或官方签名）。

⸻

17) 里程碑时间线（建议，单人开发可调整）

## 📅 详细时间线

### ✅ Week 0 - 准备阶段（已完成）
- ✅ 创建 GitHub 仓库
- ✅ 确定 MIT License
- ✅ 项目初始化
- ✅ 准备测试图片集

### ✅ Week 1-2 - Core MVP（已完成 100%）
**完成时间:** 2025-10

- ✅ Rust 项目初始化（Cargo 配置）
- ✅ EXIF 解析模块
  - ✅ 集成 kamadak-exif
  - ✅ 支持 8 个核心字段
  - ✅ 快门速度自动格式化
- ✅ 模板引擎（JSON 解析、变量替换）
- ✅ CLI 工具（clap 框架）
- ✅ 相框模式渲染
  - ✅ 底部相框生成
  - ✅ Logo 居中显示
  - ✅ 参数文字渲染
- ✅ rusttype 字体渲染（支持多语言）
- ✅ 批量处理功能（walkdir）
- ✅ 单元测试（EXIF、布局、优先级）

### 🚧 Week 3 - 完善与发布（进行中 60%）
**目标完成:** 2025-11

- ✅ Logo 路径参数化
  - ✅ `--logo` CLI 参数
  - ✅ `LITEMARK_LOGO` 环境变量
  - ✅ 优先级策略实现
- ✅ 优化字体渲染和文本定位
- ✅ 完善模板系统（变量替换）
- ✅ 文档完善
  - ✅ README 更新
  - ✅ EXIF 提取指南
  - ✅ 中文字体配置指南
- 🚧 完善测试覆盖
- 🚧 配置 CI/CD（GitHub Actions）
- 🚧 发布 v0.2.0

### 📅 Week 4-6 - iOS 原型（计划中）
**预计开始:** 2025-11 下旬

- Rust -> staticlib 编译
  - C ABI 导出
  - Swift bridging header
  - XCFramework 打包
- iOS App 开发（SwiftUI）
  - 图片选择（PhotoKit）
  - 模板预览
  - 单张/批量处理
  - 导出分享
- TestFlight 内测
- 内购集成（一次性解锁）

### 📅 Month 2-3 - Web WASM（计划中）
**预计开始:** 2025-12

- wasm-bindgen 编译
- Web UI 开发
  - 拖拽上传
  - 实时预览
  - 本地处理
- 字体加载优化（subset/woff2）
- 性能优化（大图处理）

### 📅 Month 3-6 - 扩展与市场（计划中）
**预计开始:** 2026-01

- 模板商店
- 更多内置模板
- 智能布局（避开人脸）
- macOS/Windows GUI
- 用户反馈迭代

⸻

## ✅ 当前进度总结

### 已完成功能
- ✅ Core MVP（CLI 工具）
- ✅ 真实 EXIF 数据提取
- ✅ 相框模式渲染
- ✅ 字体渲染系统（rusttype）
- ✅ Logo 支持（参数化）
- ✅ 模板系统（JSON 配置）
- ✅ 批量处理
- ✅ 单元测试
- ✅ 文档体系

### 当前版本
- **发布版本:** v0.1.0
- **开发版本:** v0.2.0-dev（60% 完成）

### 代码统计
- **语言:** Rust (edition 2021)
- **依赖库:** 10 个核心依赖
- **模块:** 5 个（exif_reader, layout, renderer, io, main）
- **内置模板:** 7 个（classic, modern, minimal, dark, elegant, professional, compact）
- **测试覆盖:** 良好（EXIF、布局、优先级）

### 质量指标
- ✅ P0 级别 bug: 0 个
- ✅ 编译错误: 0 个
- ✅ 文档完整性: 优秀
- ✅ 单元测试: 良好
- 🚧 CI/CD: 配置中

## 📋 下一步行动（Week 3 剩余任务）

### 立即执行（本周内）

1. **完善测试覆盖** (2-3 小时)
   - [ ] renderer 模块测试
   - [ ] layout 模块更多测试
   - [ ] 端到端集成测试

2. **CI/CD 配置** (2-3 小时)
   - [ ] GitHub Actions 工作流
   - [ ] 跨平台编译（Linux/macOS/Windows）
   - [ ] 自动发布 Release

3. **发布 v0.2.0** (1-2 小时)
   - [ ] 编写 CHANGELOG.md
   - [ ] 创建 GitHub Release
   - [ ] 发布二进制文件
   - [ ] 在摄影社区推广

### 中期规划（下个月）

4. **iOS 原型开发**
   - [ ] Rust -> staticlib 编译脚本
   - [ ] Swift wrapper 和 bridging header
   - [ ] SwiftUI 界面开发
   - [ ] TestFlight 内测

5. **用户反馈收集**
   - [ ] 在摄影群/论坛发布
   - [ ] 收集使用反馈
   - [ ] 迭代优化

## 🎯 成功指标

### 技术指标
- ✅ CLI 工具可用性: 100%
- ✅ EXIF 提取准确性: 优秀
- 🚧 跨平台兼容性: 待测试
- 🚧 性能（批量处理）: 待优化

### 用户指标（v0.2.0 目标）
- 📊 GitHub Stars: 目标 50+
- 📊 首月试用用户: 目标 100+
- 📊 社区反馈: 收集并迭代

### 商业指标（v1.0 目标）
- 💰 iOS App 下载: 目标 1000+
- 💰 付费转化率: 目标 5-10%
- 💰 定价策略: ¥29-¥69 一次性买断

---

**最后更新:** 2025-11-10  
**维护者:** @26huitailang  
**项目仓库:** https://github.com/26huitailang/lite-mark-core