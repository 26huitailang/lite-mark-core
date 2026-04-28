# 开发全流程 Skill 化方案

> 本文档记录 LiteMark 项目开发流程的 Skill 化设计。实际流程已由 `.kimi/skills/dev-pipeline/SKILL.md` 落地。

## 设计演进

原方案计划通过 5 个自定义 Agent YAML 文件（`main.yaml` + 4 个子 Agent）实现开发流水线。经源码级可行性验证（见 [P0 验证](#p0-验证)），发现 Kimi CLI 的 `Agent` 工具参数 `subagent_type` 仅支持内置类型（`coder`/`explore`/`plan`），虽然自定义 Agent 的 `subagents` 配置会被注册到 `labor_market` 中，但维护一套独立的 Agent 配置会增加复杂度。

因此，最终方案演进为：**不创建自定义 Agent YAML，而是将流程封装为标准 Skill**，由默认 Agent 加载后，直接调用内置子 Agent 完成任务。

## 当前架构

```
主 Agent (Coordinator，使用默认 Agent + dev-pipeline Skill)
├── 需求分析 → Agent(subagent_type="explore")
├── 架构设计 → Agent(subagent_type="plan")
├── 代码实现 → Agent(subagent_type="coder")
└── 代码审核
    ├── 4a. 静态审查 → Agent(subagent_type="coder"，只读约束)
    └── 4b. 测试验证 → Agent(subagent_type="coder"，执行约束)
```

## 关键设计决策

### 1. 使用内置子 Agent，不维护自定义 YAML
- **原因**：`explore`、`plan`、`coder` 已完全覆盖四阶段需求，无需重复造轮子
- **好处**：启动方式简单（无需 `--agent-file`），与 Kimi CLI 官方演进兼容

### 2. 中间产物统一输出到 `.kimi/sessions/`
- **原因**：避免 `PLAN.md`、`IMPLEMENTATION.md` 等文件污染 Git 工作目录
- **措施**：`.gitignore` 已添加 `/.kimi/sessions`

### 3. 代码审核拆分为"只读审查" + "测试执行"
- **原因**：解决 reviewer 同时拥有 `Shell` 和读权限时的"隐性写权限"矛盾（`sed -i` 等命令可绕过 prompt 约束）
- **拆分后**：
  - `REVIEW_CODE.md`：由纯只读 `coder` 生成，专注设计、安全、兼容性
  - `REVIEW_TEST.md`：由执行型 `coder` 生成，专注 `cargo test` / `cargo clippy` 的客观数据

### 4. Rust 增量编译策略内嵌到 developer 阶段
- **原因**：Rust workspace 完整编译耗时较长，频繁全量测试会导致 token 消耗爆炸
- **策略**：`cargo check -p litemark-core` → `cargo test --workspace --exclude litemark-wasm` → `cargo check -p litemark-wasm --target wasm32-unknown-unknown`

### 5. 与现有 Skill 体系深度融合
- 需求分析阶段：参考 `req-bug-analyzer` 的结构化报告模板
- 开发实现阶段：遵循 `litemark-dev` 的项目构建规范
- 测试验证阶段：调用 `rust-test-runner` 的错误分析策略和脚本

## 使用方法

### 方式1：对话中直接加载 Skill

```
/skill dev-pipeline
```

然后说：
> "我要开发一个新功能：[具体需求]。请启动完整开发流程。"

### 方式2：默认行为（不加载 Skill）

由于 dev-pipeline 是 Skill 而非 Agent 配置，主 Agent 不会自动触发。建议在有明确开发需求时主动 `/skill dev-pipeline`。

## P0 验证

为验证自定义子 Agent 的调用可行性，检查了 Kimi CLI 源码（`v1.35.0`）中的关键路径：

- `kimi_cli/tools/agent/__init__.py`：`Agent` 工具的 `Params` 中 `subagent_type` 仅描述为 "built-in agent type"
- `kimi_cli/background/agent_runner.py`：`type_def = self._runtime.labor_market.require_builtin_type(self._subagent_type)`
- `kimi_cli/soul/agent.py`：自定义 Agent YAML 中的 `subagents` **确实会被注册到 `labor_market.add_builtin_type()` 中**

**结论**：技术上自定义 `subagents` 可以被 `Agent` 工具调用，但维护成本高于直接使用内置子 Agent。因此选择 Skill 化方案而非自定义 Agent YAML 方案。

## 文件清单

| 文件 | 说明 |
|------|------|
| `.kimi/skills/dev-pipeline/SKILL.md` | 核心 Skill，包含完整的工作流指导和 prompt 模板 |
| `.gitignore` | 已排除 `.kimi/sessions/` |
| `skills.md` | 本文档，架构决策记录 |
