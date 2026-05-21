# 模块规划说明 — Sego-Forge

> **单人项目**，本节说明各模块的设计思路和职责划分，以目录结构体现工程规范。

---

## 模块概览

```
sego-Forge/
│
├── rust/crates/forge-core/       ← 【核心引擎层】数据模型 + 工作流状态机
│   ├── src/lib.rs                → 模块入口，统一导出
│   ├── src/error.rs              → 错误类型定义（ForgeError）
│   ├── src/model.rs              → Phase / Session / Task 数据结构
│   └── src/workflow.rs           → WorkflowEngine 阶段调度器
│
├── rust/crates/forge-api/        ← 【网络通信层】HTTP 客户端抽象
│   ├── src/lib.rs                → 模块入口
│   └── src/client.rs            → ForgeClient 统一 API 接口
│
├── rust/crates/forge-cli/        ← 【用户交互层】命令行工具
│   ├── src/main.rs               → CLI 入口 + 子命令路由
│   └── src/commands/
│       ├── init.rs               → `forge init` 项目初始化
│       ├── run.rs                → `forge run` 运行工作流
│       └── status.rs             → `forge status` 状态查询
│
├── src/                           ← 【Python 参考实现】
│   ├── core.py                    → 工作流引擎（Rust 镜像）
│   └── cli.py                     → CLI 入口（Rust 镜像）
│
├── tests/                         ← 【测试验证层】
│   └── test_core.py              → 工作流引擎 5 项单元测试
│
└── docs/                          ← 【文档说明层】
    ├── ARCHITECTURE.md            → 架构设计
    ├── SETUP.md                   → 环境搭建
    ├── MODULES.md                 → 本文件
    └── DEMO_GUIDE.md              → Demo 录制指南
```

---

## 各模块职责详解

### 1. forge-core — 核心引擎层

| 职责 | 说明 |
|------|------|
| 数据模型 | Phase(阶段)、Session(会话)、Task(任务) 的可序列化数据结构 |
| 工作流引擎 | `WorkflowEngine` 按顺序执行阶段，收集状态和耗时 |
| 错误处理 | 统一的 `ForgeError` 错误类型，支持 I/O、序列化、验证等场景 |

**设计决策**: 单一职责 — core 不依赖 CLI、不依赖 API，只负责模型和调度。

### 2. forge-api — 网络通信层

| 职责 | 说明 |
|------|------|
| HTTP 抽象 | `ForgeClient` 封装外部 API 通信 |
| 统一接口 | 隐藏具体 Provider 的差异，对上层暴露一致的方法 |

**设计决策**: 接口隔离 — API 层通过依赖注入与 core 解耦。

### 3. forge-cli — 用户交互层

| 命令 | 功能 |
|------|------|
| `forge init <name>` | 创建新项目，生成 `forge.toml` |
| `forge run` | 加载配置，启动 WorkflowEngine 流水线 |
| `forge status` | 读取 `forge.toml` 展示当前状态 |

**设计决策**: 薄 CLI — 只做参数解析和调用，业务逻辑全部在 core。

### 4. Python 参考实现

| 文件 | 对应 Rust 模块 | 作用 |
|------|---------------|------|
| `src/core.py` | `forge-core` | 快速验证工作流逻辑 |
| `src/cli.py` | `forge-cli` | 快速验证 CLI 流程 |

**设计决策**: 双语言不是冗余 — 评委可以直接 `python` 运行理解项目，无需安装 Rust 工具链。

### 5. 测试验证层

| 测试 | 覆盖内容 |
|------|---------|
| test_add_and_execute_phase | 基本阶段添加和执行 |
| test_phase_not_found | 错误处理 |
| test_phase_failure | 阶段失败状态标记 |
| test_multiple_phases_timing | 多阶段时序和耗时统计 |
| test_session_defaults | 空会话边界情况 |

---

## 单人开发的分工体现

虽然是一个人的项目，但通过以下方式体现工程规范：

1. **分层架构**: core → api → cli 三层分离，互不耦合
2. **双语言验证**: Rust 和 Python 互相校验正确性
3. **分支管理**: Day 1-3 分天提交，每天的 commit 在独立分支开发后合入 main
4. **Git 提交规范**: 使用 `feat` / `fix` / `docs` / `test` 前缀区分改动类型
