# Sego-Forge 🔥

> **AI-powered development workflow engine** — 基于 Sego Agent 技术内核的智能开发工作流引擎

[![CI](https://github.com/007M7/sego-Forge/actions/workflows/ci.yml/badge.svg)](https://github.com/007M7/sego-Forge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Rust](https://img.shields.io/badge/rust-stable-orange.svg)
![Python](https://img.shields.io/badge/python-3.12-blue.svg)

---

## 项目简介

Sego-Forge 将 AI Agent 的工作流编排能力提炼为一个通用的开发引擎。它采用**阶段化流水线**，将任意开发任务拆分为可控的步骤，并提供完整的执行追踪和状态管理。

**核心价值：** 让 AI 驱动的开发流程可观测、可重放、可信任。

## 快速开始

```bash
# 运行所有测试（推荐第一步）
python -m pytest tests/ -v          # Python 参考实现
cd rust && cargo test --workspace   # Rust 核心引擎

# 快速验证
cd rust && cargo run -- run         # 运行模拟工作流
python -m src.cli run -c forge.toml # Python 版本
```

## 项目结构

```
sego-Forge/
├── rust/                         # Rust 核心（3-crate workspace）
│   ├── crates/
│   │   ├── forge-core/          # 数据模型 + 工作流引擎 + 错误类型
│   │   ├── forge-api/           # HTTP 客户端抽象层
│   │   └── forge-cli/           # CLI 入口（init / run / status）
│   └── Cargo.toml               # Workspace 配置
├── src/                          # Python 参考实现
│   ├── core.py                   # 工作流引擎（Rust 镜像）
│   └── cli.py                    # CLI 入口
├── tests/                        # 测试套件
│   └── test_core.py              # 工作流引擎单元测试
├── docs/                         # 文档
│   ├── ARCHITECTURE.md           # 架构设计
│   ├── SETUP.md                  # 环境搭建指南
│   ├── MODULES.md                # 模块规划说明
│   └── DEMO_GUIDE.md             # Demo 视频录制指南
└── .github/workflows/ci.yml      # CI/CD 流水线
```

## 技术栈

| 层级 | 技术 | 说明 |
|------|------|------|
| 核心引擎 | Rust (tokio async) | 高性能工作流编排 |
| 参考实现 | Python 3.12+ | 快速原型验证 |
| CLI | Clap (Rust) / Argparse (Python) | 命令行工具 |
| 测试 | cargo test / pytest | 双语言并行测试 |
| CI/CD | GitHub Actions | 自动构建 + 测试 + 代码检查 |

## 文档

- 📐 [架构设计](docs/ARCHITECTURE.md) — 系统架构和数据流
- 🚀 [环境搭建](docs/SETUP.md) — 一键启动指南
- 📦 [模块规划](docs/MODULES.md) — 各模块职责与接口
- 🎬 [Demo 录制](docs/DEMO_GUIDE.md) — 演示视频录制教程

## 开发路线

- [x] Day 0: 项目脚手架 + CI/CD + 文档框架
- [ ] Day 1: 议题公布后 — 业务功能开发
- [ ] Day 2: 功能完善 + 持续 commit
- [ ] Day 3: Demo 视频 + 最终交付

## License

MIT © 007M7
