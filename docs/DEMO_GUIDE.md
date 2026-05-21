# Demo 视频录制指南 — Sego-Forge

> 面向**零经验者**的 5 分钟录屏教程。无需专业设备，一台电脑即可完成。

---

## 你需要准备

| 项目 | 要求 |
|------|------|
| 电脑 | 能运行 Sego-Forge 即可 |
| 麦克风 | 笔记本电脑自带的就行 |
| 安静环境 | 减少背景噪音 |

---

## 方案一：OBS Studio（推荐，免费）

### 1. 下载安装

```
https://obsproject.com/download
```

下载 Windows 版，一路下一步即可。（约 2 分钟）

### 2. 首次配置

打开 OBS → 点击"自动配置向导" → 选择"仅优化录制" → 完成。

### 3. 添加录制源

1. 点击左下角 `+` → **显示器采集** → 确定
2. 点击左下角 `+` → **音频输入采集** → 选择你的麦克风 → 确定
3. 调整音量条，说话时绿色条到黄色区域即可

### 4. 录制

1. 点击右下角 **开始录制**
2. 按以下脚本演示你的项目
3. 点击右下角 **停止录制**
4. 视频保存在 `此电脑 → 视频` 文件夹

---

## 方案二：Windows 自带（更简单，无需安装）

1. 按 `Win + G` 打开 Xbox Game Bar
2. 点击"捕获"面板 → 点圆形录制按钮（或按 `Win + Alt + R`）
3. 录制完成后再次按 `Win + Alt + R` 停止
4. 视频保存在 `此电脑 → 视频 → 捕获` 文件夹

---

## 推荐演示脚本（3-5 分钟）

按照这个顺序演示，评委能完整理解你的项目：

### 第 1 分钟：项目介绍（30 秒）

> "大家好，我是 [名字]，今天演示的项目是 Sego-Forge——一个基于 AI Agent 技术的工作流编排引擎…"

展示 README.md 页面。

### 第 2 分钟：环境搭建和测试（60 秒）

```bash
# 打开终端
cd sego-Forge

# Python 测试
python -m pytest tests/ -v
# → 展示 5 passed

# Rust 测试（可选）
cd rust && cargo test --workspace
# → 展示 4 passed
```

### 第 3-4 分钟：核心功能演示（90 秒）

```bash
# 运行 Rust 版工作流
cd rust && cargo run -- run
# → ✅ Workflow complete: 3/3 phases

# 展示 Python 版等价功能
cd ..
python -m src.cli run
# → ✅ Workflow complete: 3/3 phases
```

### 第 4-5 分钟：架构讲解 + 总结（60 秒）

- 快速过一遍 `docs/ARCHITECTURE.md` 中的 Mermaid 图
- 解释三层架构（core → api → cli）
- "感谢观看，项目链接在仓库 README 中"

---

## 上传到 B 站

1. 打开 [bilibili.com](https://www.bilibili.com) → 登录
2. 点击右上角"投稿" → 上传视频
3. 标题示例：`【2026 暑期夏令营】Sego-Forge — AI 工作流编排引擎`
4. 发布后复制视频链接
5. 把链接加到仓库 README.md 顶部

---

## 上传到 YouTube（备选）

1. 打开 [YouTube Studio](https://studio.youtube.com)
2. 点击"创建" → "上传视频"
3. 设为"公开"
4. 复制链接加入 README

---

## 常见问题

**Q: 录制的视频太大怎么办？**

Windows 推荐用 OBS → 设置 → 输出 → 输出模式改为"简单" → 视频比特率调到 2500 Kbps。

**Q: 说话紧张怎么办？**

可以先用手机录音练习 3 遍，习惯了再正式录。或者写逐字稿照着念。

**Q: 口误了怎么办？**

不要停，继续说。小口误不扣分。如果大段说错，剪辑掉或用 OBS 重新录那一段。

**Q: 我完全没有时间学 OBS？**

用 Windows 自带的 Xbox Game Bar（Win+G），零学习成本。
