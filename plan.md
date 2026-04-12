# LiteMark 路线图

**当前版本:** v0.2.0  
**定位:** 轻量摄影参数水印工具

---

## 已完成 ✅

- Core/CLI/WASM 三层架构
- 真实 EXIF 提取 (8个字段)
- 模板系统 (JSON配置)
- 批量并发处理
- 自定义字体/Logo
- 完整单元测试

---

## 进行中 🚧

- [ ] CI/CD 优化
- [ ] 更多内置模板
- [ ] 性能基准测试

---

## 计划 📅

### v1.0 - iOS App (1-2月)
- SwiftUI 界面
- Rust Core 集成
- TestFlight 内测
- 一次性买断内购

### v2.0 - Web & Desktop (3-6月)
- WASM Web Demo
- macOS/Windows GUI
- 模板市场
- 智能布局

---

## 技术栈

- **Core:** Rust, image, rusttype, kamadak-exif
- **CLI:** clap, rayon, indicatif
- **WASM:** wasm-bindgen
- **iOS:** SwiftUI + Rust staticlib

---

## 参考

- [开发指南](docs/DEVELOPMENT.md)
- [架构文档](docs/ARCHITECTURE.md)
- [历史文档](archive/)
