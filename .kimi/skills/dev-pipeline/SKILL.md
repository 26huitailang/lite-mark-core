---
name: dev-pipeline
description: |
  指导主 Agent 通过协调 explore/plan/coder 内置子 Agent，完成"需求分析→架构设计→代码实现→代码审核"的工业化开发流水线。
  统一中间产物输出到 .kimi/sessions/，支持循环修复，与 req-bug-analyzer、rust-test-runner 等现有 Skill 协同。
---

# dev-pipeline: 开发全流程流水线

## 何时使用

当用户提出涉及多步骤、需要编码实现的开发需求时，如：
- "我要开发一个新功能：..."
- "请帮我实现..."
- "启动完整开发流程"

## 核心设计

**不使用自定义 Agent YAML**，而是直接调用 Kimi CLI 内置子 Agent：

| 阶段         | 子 Agent  | 类型 | 职责                      |
| ------------ | --------- | ---- | ------------------------- |
| 1. 需求分析  | `explore` | 内置 | 只读代码探索与需求理解    |
| 2. 架构设计  | `plan`    | 内置 | 只读方案规划              |
| 3. 代码实现  | `coder`   | 内置 | 编码、测试、文档          |
| 4a. 代码审查 | `coder`   | 内置 | **只读**代码质量审查      |
| 4b. 测试验证 | `coder`   | 内置 | 运行 lint/test 等检查工具 |

## 会话目录

每次启动流水线时，创建一个时间戳目录：

```bash
mkdir -p .kimi/sessions/YYYYMMDD-HHMMSS/
```

所有中间产物必须写入该目录：

- `REQUIREMENT.md` — 需求分析报告
- `PLAN.md` — 实现方案
- `IMPLEMENTATION.md` — 实现记录
- `REVIEW_CODE.md` — 代码审查报告
- `REVIEW_TEST.md` — 测试验证报告

## 标准工作流

### 阶段1: 需求分析

调用 `Agent` 工具：

```python
Agent(
    subagent_type="explore",
    description="需求分析与代码定位",
    prompt=f"""请分析以下需求，并深度探索代码库定位改动点。

需求：{user_requirement}

要求：
1. 阅读 AGENTS.md 和 litemark-core/ARCHITECTURE.md 了解项目规范
2. 使用 req-bug-analyzer skill 的方法论进行结构化分析
3. 输出结构化报告到 `{session_dir}/REQUIREMENT.md`

报告中必须包含：
- 需求摘要（核心功能、涉及模块、技术约束）
- 代码定位结果（文件路径、相关度、说明）
- 风险评估（破坏性变更、依赖第三方、数据库迁移等）
- 建议下一步（调用 plan Agent 进行架构设计）

重要：你只能读取代码，禁止修改任何文件。
"""
)
```

### 阶段2: 架构设计

调用 `Agent` 工具：

```python
Agent(
    subagent_type="plan",
    description="架构设计与方案规划",
    prompt=f"""请基于需求分析报告制定实现方案。

需求报告路径：`{session_dir}/REQUIREMENT.md`

要求：
1. 读取上述报告
2. 设计技术方案（数据模型、接口契约、算法、错误处理）
3. 按原子任务拆分实现步骤，估算复杂度
4. 将方案写入 `{session_dir}/PLAN.md`

PLAN.md 必须包含：
- 技术决策与理由
- 实现步骤（按优先级，标注复杂度与依赖）
- 文件变更清单（操作、文件路径、说明）
- 验收标准

注意：你不要调用 EnterPlanMode 或 ExitPlanMode，只负责输出 PLAN.md。
"""
)
```

### 阶段2.5: 方案审批（主 Agent 执行）

由主 Agent 直接执行：
1. `ReadFile` 读取 `PLAN.md`
2. `EnterPlanMode` 进入计划模式
3. `ExitPlanMode` 提交方案给用户审批
4. **等待用户明确确认后再继续**

### 阶段3: 代码实现

调用 `Agent` 工具：

```python
Agent(
    subagent_type="coder",
    description="代码实现与测试",
    prompt=f"""请将架构方案转化为高质量代码。

方案路径：`{session_dir}/PLAN.md`

实现规范：
1. **Rust 增量编译策略**（参考 AGENTS.md 或 litemark-dev skill）：
   - 修改后先运行 `cargo check -p litemark-core` 快速验证
   - 通过后再运行 `cargo test --workspace --exclude litemark-wasm`
   - 若涉及 WASM，额外运行 `cargo check -p litemark-wasm --target wasm32-unknown-unknown`
   - 长时命令使用 `run_in_background=true`

2. 遵循项目代码风格（见 AGENTS.md）
3. 新功能必须有测试覆盖
4. 错误处理必须完整
5. 公共 API 添加文档

过程记录：
- 将完整实现记录写入 `{session_dir}/IMPLEMENTATION.md`
- 包含：执行步骤、变更清单、技术决策、遇到的问题、验证结果

每完成一个子任务使用 SetTodoList 更新进度。
"""
)
```

### 阶段4: 代码审核

**拆分为两个并行的子任务**，可后台运行：

#### 4a. 静态代码审查（只读）

调用 `Agent` 工具：

```python
Agent(
    subagent_type="coder",
    description="静态代码审查",
    prompt=f"""请严格审查代码质量，但**禁止修改任何代码**。

审查范围：
- 读取 `{session_dir}/IMPLEMENTATION.md` 中的变更清单
- 使用 `git diff` 获取实际变更作为补充
- 对变更文件进行逐行审查

审查维度：
1. 功能性：是否满足需求？边界条件处理？
2. 代码质量：可读性、命名规范、复杂度
3. 安全性：敏感信息泄漏、权限控制
4. 兼容性：是否破坏向后兼容？

**权限约束**：你只能读取文件和运行 `git diff`/`git log` 等只读命令，**严禁使用 WriteFile、StrReplaceFile 或直接/间接修改代码的 Shell 命令**（如 sed -i、echo >> file 等）。

输出：将审查报告写入 `{session_dir}/REVIEW_CODE.md`
"""
)
```

#### 4b. 测试与检查（执行）

调用 `Agent` 工具：

```python
Agent(
    subagent_type="coder",
    description="运行测试与静态检查",
    prompt=f"""请运行项目的测试和静态检查工具，收集客观数据。

执行清单（参考 AGENTS.md 或 litemark-dev skill）：
1. `cargo test --workspace --exclude litemark-wasm`
2. `cargo clippy --workspace --all-targets --exclude litemark-wasm`
3. 如适用：`cargo check -p litemark-wasm --target wasm32-unknown-unknown`

要求：
- 长时命令使用 `run_in_background=true`
- 使用 rust-test-runner skill 的策略分析失败结果
- 只运行检查工具，**不修改代码**

输出：将测试结果汇总写入 `{session_dir}/REVIEW_TEST.md`
"""
)
```

### 阶段4.5: 审核决策（主 Agent 执行）

主 Agent 读取 `REVIEW_CODE.md` 和 `REVIEW_TEST.md`，进行汇总：

- **通过**：向用户汇报完成
- **有条件通过**：列出小问题，由主 Agent 决定是否需要修复
- **不通过**：
  1. 提取阻塞性问题清单
  2. 将问题反馈给 `coder` Agent 修复
  3. 修复完成后再次进入 4a/4b 复审
  4. **最多循环 2 次**，超过则上报用户决策

## 上下文管理原则

为避免主 Agent 上下文爆炸：

1. **不保留完整中间产物**：子 Agent 返回后，主 Agent 只提取决策所需的摘要
2. **按需读取**：需要引用细节时，使用 `ReadFile` 读取会话目录下的文件
3. **循环修复时传递精简上下文**：将 reviewer 的阻塞性问题以 bullet list 形式传给 developer，而不是传整个报告文件

## 与现有 Skill 的协作

流水线各阶段应主动利用项目已有的 Skill：

| 阶段     | 参考 Skill         | 说明                             |
| -------- | ------------------ | -------------------------------- |
| 需求分析 | `req-bug-analyzer` | 使用其结构化分析模板和只读方法论 |
| 开发实现 | `litemark-dev`     | 遵循其构建命令和项目结构规范     |
| 测试验证 | `rust-test-runner` | 使用其测试解析脚本和错误分析流程 |
| 发布准备 | `litemark-release` | 如涉及发布，参考其版本管理流程   |
