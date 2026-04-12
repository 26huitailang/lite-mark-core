---
name: rust-test-runner
description: Run Rust unit tests, report errors with analysis, and provide fix suggestions. Use when user asks to run tests, check test results, fix test failures, or any cargo test related tasks. Works for cargo test, cargo check, clippy, and other Rust testing workflows.
---

# Rust Test Runner

Run Rust tests and provide intelligent error analysis with fix suggestions.

## Workflow

### 1. Run Tests

Execute tests based on project structure:

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

### 2. Analyze Errors

Parse test output and categorize errors:

**Compilation Errors:**
- Syntax errors (missing `,`, `;`, unmatched braces)
- Type mismatches
- Missing imports
- Duplicate definitions
- Missing derive macros

**Test Failures:**
- Assertion failures
- Panics
- Timeouts

**Warnings (optional fix):**
- Unused imports/variables
- Deprecated code

### 3. Provide Fix Suggestions

For each error:
1. Show error location (file:line:column)
2. Explain the root cause
3. Provide concrete fix (code snippet)
4. Ask for confirmation before applying

### 4. Apply Fixes

After user confirms:
1. Apply fixes using StrReplaceFile
2. Re-run tests to verify
3. Report results

## Error Patterns & Fixes

### Common Compilation Errors

**`expected ',', found '+'`**
- Cause: Using `+` for string concatenation in `println!`
- Fix: Use `format!()` or `,` separator
```rust
// Wrong
println!("\n" + &"=".repeat(50));
// Fix
println!("\n{}", "=".repeat(50));
```

**`name 'X' is defined multiple times`**
- Cause: Duplicate imports
- Fix: Remove duplicate import

**`cannot find derive macro 'Deserialize'`**
- Cause: Missing serde feature or dependency
- Fix: Check Cargo.toml for `serde` with `derive` feature

### Test-Specific Issues

**Test timeouts:** Add `--timeout` or optimize test
**Assertion failures:** Show expected vs actual
**Missing test data:** Check fixtures directory

## Scripts

Use `scripts/parse_test_output.py` to parse cargo test output:

```bash
cargo test --workspace 2>&1 | python3 .kimi/skills/rust-test-runner/scripts/parse_test_output.py
```

This extracts:
- Error locations
- Error messages
- Suggested fixes

## Best Practices

1. **Always check first**: Run `cargo check` before full test suite for faster feedback
2. **Incremental fixes**: Fix one error category at a time
3. **Verify after fix**: Re-run tests after each fix batch
4. **Clippy too**: Run `cargo clippy` for additional suggestions

## Example Interaction

User: "运行测试"

→ Run `cargo test --workspace`
→ Parse output, find errors
→ Report: "发现 3 个编译错误："
  1. `tests/src/bin/health_check.rs:121` - println! 中使用了 `+` 拼接字符串
  2. `tests/src/integration/pipeline_tests.rs:364` - 重复导入 `RgbImage`
  3. `litemark-core/tests/integration_test.rs:7` - 缺少 `Deserialize` derive
→ Provide fixes
→ Ask: "是否应用这些修复？"
→ Apply and re-run
