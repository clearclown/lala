<div align="center">

# 🎵 Lala

**现代化、轻量级多格式文本编辑器**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Tests](https://img.shields.io/badge/tests-171%20passing-brightgreen.svg)]()

[English](./docs/readmeLangs/README_en.md) | [日本語](../../README.md) | [简体中文](README_zh-CN.md) | [繁體中文](README_zh-TW.md) | [Русский](README_ru.md) | [فارسی](README_fa.md) | [العربية](README_ar.md)

</div>

---

## 💡 什么是Lala？

Lala是用**Rust**编写的下一代文本编辑器。使用纯Rust GUI框架**egui**，实现了轻量级和高速性能。

---

## ✨ 主要功能

### 🎨 多格式支持
- **Markdown**: 实时预览，语法高亮
- **HTML**: 解析和渲染
- **LaTeX**: 数学符号的Unicode转换预览
- **Mermaid**: 流程图和时序图可视化

### 🚀 高性能
- **轻量级**: Rust零成本抽象实现快速启动（<100ms）
- **高效**: 使用Rope数据结构快速编辑大文件
- **原生**: 无需Electron，最小化系统资源使用

### 🌏 完整的中文支持
- **IME支持**: 原生支持中文、日文和韩文输入法
- **Unicode支持**: 支持所有Unicode字符，包括表情符号和符号

### 🤖 AI集成（可选）
- **Gemini API集成**: 自动文本改进
- **语法纠正**: 自动检测和修复拼写/语法错误
- **摘要**: 长文本自动摘要

---

## 📦 安装

### 从Cargo安装（推荐）

```bash
cargo install lala
```

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/clearclown/lala.git
cd lala

# 发布构建
cargo build --release

# 二进制文件生成在 target/release/lala
cargo install --path .
```

---

## 🚀 使用

### GUI模式启动

```bash
# 启动空编辑器
lala

# 打开特定文件
lala README.md
```

### 键盘快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+N` | 新建文件 |
| `Ctrl+O` | 打开文件 |
| `Ctrl+S` | 保存 |
| `Ctrl+F` | 搜索 |
| `Ctrl+P` | 切换预览 |

---

## 📄 许可证

本项目采用双许可证：

- **MIT许可证** ([LICENSE-MIT](../../LICENSE-MIT))
- **Apache许可证 2.0** ([LICENSE-APACHE](../../LICENSE-APACHE))

您可以选择任一许可证。

---

<div align="center">

Made with ❤️ by the Lala contributors

</div>
