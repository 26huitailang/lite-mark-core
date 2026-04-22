---
name: rust-test-runner
description: 运行 Rust 测试并分析错误，提供修复建议。当用户要求运行测试、检查测试结果、修复测试失败或执行 cargo test / cargo check / clippy 相关任务时使用。
---

# Rust 测试运行器

运行 Rust 测试，提供错误分析与修复建议。

## 工作流

### 1. 运行测试

根据项目结构执行测试：

```bash
# Default: run all workspace tests (excluding wasm if needed)
cargo test --workspace

# If litemark-wasm exists and causes issues
cargo test --workspace --exclude litemark-wasm

# Run specific package
cargo test -p <package-name>

# Run with output
cargo test --workspace -- --nocapture
```

### 2. 分析错误

解析测试输出并分类错误：

**编译错误：**
- 语法错误（缺少 `,`、`;`、括号不匹配）
- 类型不匹配
- 缺少导入
- 重复定义
- 缺少 derive 宏

**测试失败：**
- 断言失败
- panic
- 超时

**警告（可选修复）：**
- 未使用的导入/变量
- 已弃用代码

### 3. 提供修复建议

对每个错误：
1. 显示错误位置（file:line:column）
2. 解释根本原因
3. 提供具体修复（代码片段）
4. 应用前请求确认

### 4. 应用修复

用户确认后：
1. 使用 StrReplaceFile 应用修复
2. 重新运行测试验证
3. 报告结果

## 常见错误模式与修复

### 常见编译错误

**`expected ',', found '+'`**
- 原因：在 `println!` 中使用 `+` 拼接字符串
- 修复：使用 `format!()` 或 `,` 分隔符
```rust
// 错误
println!("\n" + &"=".repeat(50));
// 修复
println!("\n{}", "=".repeat(50));
```

**`name 'X' is defined multiple times`**
- 原因：重复导入
- 修复：删除重复导入

**`cannot find derive macro 'Deserialize'`**
- 原因：缺少 serde feature 或依赖
- 修复：检查 Cargo.toml 中是否有带 `derive` feature 的 `serde`

### 测试特定问题

**测试超时：** 添加 `--timeout` 或优化测试
**断言失败：** 显示预期值与实际值
**缺少测试数据：** 检查 fixtures 目录

## 脚本

使用 `scripts/parse_test_output.py` 解析 cargo test 输出：

```bash
cargo test --workspace 2>&1 | python3 .kimi/skills/rust-test-runner/scripts/parse_test_output.py
```

提取内容：
- 错误位置
- 错误信息
- 建议修复

## 最佳实践

1. **先检查再测试**：运行完整测试前先执行 `cargo check` 获取更快反馈
2. **增量修复**：一次修复一类错误
3. **修复后验证**：每批修复后重新运行测试
4. **也运行 Clippy**：执行 `cargo clippy` 获取额外建议

## 交互示例

用户："运行测试"

→ 运行 `cargo test --workspace`
→ 解析输出，查找错误
→ 报告："发现 3 个编译错误："
  1. `tests/src/bin/health_check.rs:121` - println! 中使用了 `+` 拼接字符串
  2. `tests/src/integration/pipeline_tests.rs:364` - 重复导入 `RgbImage`
  3. `litemark-core/tests/integration_test.rs:7` - 缺少 `Deserialize` derive
→ 提供修复
→ 询问："是否应用这些修复？"
→ 应用并重新运行
