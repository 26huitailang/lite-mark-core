# LiteMark 测试套件待办事项

> 目标：测试套件在本地和 CI 上全部通过

## 当前状态

- ✅ `unit` 测试已通过 (54/54)
- ❌ `integration` 测试存在编译错误
- ❌ `e2e` 测试待验证

## 待办列表

### 1. 修复 integration 测试编译错误 (regression_tests.rs 类型不匹配) 🔴

**问题描述**:
```
regression_tests.rs:240-243  类型不匹配
  - Aperture 期望 u32，但传入 f64 (1.4, 22.0)
  - Focal 期望 u32，但传入 f64 (24.0, 200.0)
```

**修复建议**:
- 检查 `ExifData` 结构体的字段类型定义
- 修改测试用例中的类型，或调整结构体定义

---

### 2. 运行本地完整测试套件并确保全部通过 ⏳

```bash
# 运行所有测试
cargo test -p litemark-test-suite

# 运行特定测试集
cargo test -p litemark-test-suite --test unit
cargo test -p litemark-test-suite --test integration
cargo test -p litemark-test-suite --test e2e
```

---

### 3. 确保 CI 工作流程测试通过 ⏳

- [ ] 检查 `.github/workflows/test-suite.yml` 配置
- [ ] 确保 CI 环境与本地一致
- [ ] 修复 CI 特有的问题（如果有）

---

### 4. 清理测试警告 (unused variables/imports) ⏳

**当前警告**:
- `template_tests.rs:86` unused variable: `name`
- 其他未使用导入

```bash
# 自动修复部分警告
cargo fix --test integration
cargo fix --test e2e
```

---

## 完成标准

- [ ] `cargo test -p litemark-test-suite` 本地全部通过
- [ ] GitHub Actions CI 测试通过
- [ ] 无编译错误和警告（或警告被合理处理）

## 备注

- 上次提交：`c26bed6` fix: 修复测试套件编译错误和运行时溢出
- 主要修复了 unit 测试，integration 测试待修复
