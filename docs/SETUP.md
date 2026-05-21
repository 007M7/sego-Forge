# 环境搭建指南 — Sego-Forge

> 从克隆仓库到运行第一个工作流，只需 3 分钟

---

## 前置要求

| 工具 | 最低版本 | 安装方式 |
|------|---------|----------|
| Git | 2.x | `winget install Git.Git` (Windows) |
| Rust | 1.80+ | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Python | 3.12+ | `winget install Python.Python.3.12` (Windows) |
| 文本编辑器 | 任意 | VSCode 推荐 |

---

## 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/007M7/sego-Forge.git
cd sego-Forge
```

### 2. 运行 Rust 测试

```bash
cd rust
cargo test --workspace
```

预期输出:

```
running 4 tests
test forge_core::workflow::tests::test_add_and_execute_phase ... ok
test forge_core::workflow::tests::test_phase_not_found ... ok
test forge_api::client::tests::test_health_check ... ok
test forge_cli::commands::init::tests::test_init ... (if added)

test result: ok. 4 passed; 0 failed; 0 ignored
```

### 3. 运行 Python 测试

```bash
pip install pytest pytest-asyncio
python -m pytest tests/ -v
```

预期输出:

```
tests/test_core.py::test_add_and_execute_phase PASSED
tests/test_core.py::test_phase_not_found PASSED
tests/test_core.py::test_phase_failure PASSED
tests/test_core.py::test_multiple_phases_timing PASSED
tests/test_core.py::test_session_defaults PASSED

5 passed
```

### 4. 运行模拟工作流

**Rust 版本:**

```bash
cd rust && cargo run -- run
```

**Python 版本:**

```bash
python -m src.cli run -c forge.toml
```

预期输出:

```bash
✅ Workflow complete: 3/3 phases
```

---

## 项目结构速览

```
sego-Forge/
├── rust/crates/forge-core/   ← 修改这里改核心逻辑
├── rust/crates/forge-cli/    ← 修改这里改 CLI 命令
├── src/core.py               ← Python 上快速验证想法
├── tests/test_core.py        ← 添加测试
└── docs/                     ← 更新文档
```

---

## 常见问题

### Q: 我没有 Rust，只想用 Python？

完全没问题。Rust 核心和 Python 参考实现是**功能等价的**，评委可以直接用 `python` 运行。

### Q: 如何添加新的工作流阶段？

```python
engine.add_phase("my_phase", "My Custom Phase")
await engine.execute_phase("my_phase", my_async_function)
```

### Q: CI 通过是什么状态？

查看 GitHub Actions 标签：`https://github.com/007M7/sego-Forge/actions`

---

## 开发工作流

```bash
# 1. 写代码 → 跑测试 → 提交
python -m pytest tests/ -v
git add -A
git commit -m "feat: xxx"

# 2. 推送 → 自动触发 GitHub Actions 测试
git push origin main
```
