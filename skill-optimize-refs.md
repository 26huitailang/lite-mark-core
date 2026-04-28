
# Subagent 构建最佳实践总结

> **核心原则**：采用渐进式构建，逐个节点稳定后再串联，拒绝一次性搭建大而全的流程。

---

## 一、架构选择：渐进式 vs 大而全

### 不要大而全的 3 个理由

1. **故障定位成本指数级上升**：N 个串联的 subagent 中任意一个波动，排查空间是 N! 组合
2. **上下文污染**：前期 subagent 的失误输出会作为后续节点的输入，错误级联放大
3. **冻结过早**：缺乏真实数据反馈的情况下，把错误的接口契约固化进流程

### 渐进式构建路线图

按以下顺序逐个攻破，**每个节点稳定运行至少一周**再添加下一个：

```
Phase 1: 需求拆解 → 技术方案（文档产出）
Phase 2: 技术方案 → 代码实现（可运行代码）
Phase 3: 代码实现 → 本地验证（测试通过+回归）
Phase 4: 全流程串联 + 人工接管点（fallback）
```

> **当前行动建议**：冻结 Phase 2 及以后的规划，全力把当前 dev 节点做到稳定。

---

## 二、问题诊断：Dev Subagent 不稳定的 5 个根因

| 序号 | 根因 | 症状 | 修复方案 |
|:---:|:---|:---|:---|
| 1 | **准入标准模糊** | 同样需求，有时产出完整代码，有时产出伪代码 | 定义严格的输入/产出契约模板 |
| 2 | **上下文窗口失控** | 长需求时，subagent 后期遗忘早期约束 | 分片上下文 + 上下文摘要器（2k token 内） |
| 3 | **触发器过于聪明** | 用自然语言描述复杂条件触发，导致理解偏差 | 用显式状态文件代替自然语言触发 |
| 4 | **缺乏 Ground Truth** | 不知道产出好坏，只能靠人工扫读 | 为每个 subagent 定义自动化守门员 |
| 5 | **副作用未隔离** | 修改了不该改的文件，或读取过时缓存 | 独立工作目录 + 显式权限控制 |

---

## 三、可立即执行的稳定化方案

本周内做这个实验：

1. **缩小范围**：只让 dev subagent 处理单一函数/模块的实现，而非整个需求
2. **冻结输入**：准备 3 个固定的、已验证的 Ground Truth 测试用例（需求文档+设计文档）
3. **运行 10 次**：记录每次产出差异，量化稳定性（编译通过率、测试通过率、文档产出率）
4. **修复最差项**：如果 10 次里有 3 次编译失败，先解决编译问题，不要优化别的

---

## 四、契约与交接模板

### 输入契约（Entry Contract）

```markdown
## 输入要求
- [ ] 需求文档包含：用户故事、验收标准、边界条件
- [ ] 技术方案包含：模块划分、接口签名、错误处理策略
- [ ] 缺少任一字段，subagent 必须拒绝执行并返回：BLOCKED: 缺少[字段名]

## 产出标准（Exit Criteria）
- [ ] 代码通过静态检查（cargo check / go build / tsc 等）
- [ ] 包含单元测试骨架（即使暂时只有 TODO）
- [ ] 输出 IMPLEMENTATION_NOTES.md 说明技术债和假设
```

### 阶段交接文档（Handoff Document）

```markdown
# Handoff Document v1.0

## 上游产出
- 文件路径：docs/design/auth-module.md
- 校验和：sha256:abc123...

## 当前阶段指令（不可协商）
- 必须实现 AuthService 接口，签名如下：...
- 必须使用 bcrypt 进行密码哈希，cost=12
- 禁止引入新的外部依赖

## 已知约束
- 数据库连接池已存在于 internal/db，直接引用，不要新建
- 上游遗留问题：JWT 刷新逻辑未设计，本阶段用 TODO 标记并写入 TECH_DEBT.md

## 产出物清单
- [ ] internal/auth/service.go（实现）
- [ ] internal/auth/service_test.go（测试）
- [ ] docs/handoff/03-auth-implementation.md（下游交接文档）
```

---

## 五、工作区状态管理（显式触发机制）

用文件存在性代替自然语言触发：

```
.subagent_state/
├── 01_requirement.md      # Phase 1 产出
├── 02_design.md          # Phase 2 产出（存在且非空时，触发 dev subagent）
└── 03_implementation/
    ├── status             # pending / running / done / failed
    └── handoff.md         # 向下游交接的摘要
```

触发逻辑示例：

> 检测到 02_design.md 存在且 03_implementation/status 不为 done，则触发 dev subagent。

---

## 六、多 Agent 工作流构建原则

1. **限制每个 Agent 的工作范围**，但保持 Agent 总数合理（不要过度拆分）
2. **使用 Memory 文件**：
   - `project-rules.md`（通用规范）
   - `input-template.md` / `output-template.md`（每个 Agent 的输入输出格式）
   - `workflow-state.md`（当前执行到第几步，每步状态）
3. **插入断点（Breakpoints）**：人工验证后才能继续
4. **能用代码执行就不用 LLM**：算法类任务写脚本，提高稳定性，降低 token 消耗

---

## 七、参考资源汇总

### 必读文章

| 资源 | 作者/来源 | 解决什么问题 | 链接 |
|:---|:---|:---|:---|
| AI Agent Orchestration Patterns | Microsoft Azure | 4 种编排模式（Sequential/Concurrent/Handoff/GroupChat/Magentic），架构级权威 | https://learn.microsoft.com/en-us/azure/architecture/ai-ml/guide/ai-agent-design-patterns |
| How to Build Multi Agent AI Systems With Context Engineering | Vellum | 5 个设计问题检查清单，上下文隔离与共享内存方案 | https://www.vellum.ai/blog/multi-agent-systems-building-with-context-engineering |
| Kimi CLI 官方仓库 | MoonshotAI | Kimi CLI 源码、AGENTS.md、开发文档 | https://github.com/MoonshotAI/kimi-cli |
| AGENTS.md | MoonshotAI/kimi-cli | 官方架构文档，LaborMarket、SubagentStore 持久化机制 | https://github.com/MoonshotAI/kimi-cli/blob/main/AGENTS.md |
| Kimi Agent SDK | MoonshotAI | 多语言 SDK，支持程序化编排 Kimi CLI agent | https://github.com/MoonshotAI/kimi-agent-sdk |

### 视频资源

| 资源 | 作者/来源 | 内容要点 | 链接 |
|:---|:---|:---|:---|
| Armchair Architects: Multi-agent Orchestration and Patterns | Microsoft Azure Essentials | 企业级多 agent 架构实战、DLP、成本、设计模式 | https://learn.microsoft.com/en-us/shows/azure-essentials-show/armchair-architects-multi-agent-orchestration-and-patterns |

### 开源参考项目

- **Spec Kit**：需求规格化工具
- **OpenSpec**：开放规范格式
- **BMAD-Method**：多 Agent 开发方法论

---

## 八、检查清单：启动下一个 Subagent 前

- [ ] 当前节点已用 3 个 Ground Truth 用例验证，连续 10 次产出一致
- [ ] 定义了明确的 Entry Contract 和 Exit Criteria
- [ ] 使用了独立工作目录（git worktree / 容器卷）隔离副作用
- [ ] 上游产出物有校验和或版本标识，防止读取过时文件
- [ ] 在关键节点插入了人工 Approval Gate
- [ ] 为当前阶段编写了自动化 Gatekeeper（编译/测试/格式检查）
- [ ] 上下文摘要控制在 2k token 以内，原始历史不直接传递给下游

---

## 九、Kimi CLI Subagent 关键机制

根据官方 AGENTS.md 和源码：

- **LaborMarket**：子代理劳动力市场，管理 subagent 的注册和调度
- **SubagentStore**：每个 subagent 实例独立持久化，保存在 `session/subagents/<agent_id>/` 下，包含 wire logs 和 context
- **上下文压缩（Compaction）**：在 subagent 生命周期中自动压缩上下文，防止窗口溢出
- **依赖注入**：通过配置文件和代码显式注入工具、技能、MCP server

---

> **总结**：大而全是目标，但逐个稳定的节点才是到达那里的唯一路径。当前的不稳定，大概率是接口契约和触发机制还不够机械（explicit），还依赖了太多隐式理解。先冻结范围，量化稳定性，再横向扩展。

