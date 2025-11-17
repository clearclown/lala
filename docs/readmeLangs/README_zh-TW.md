<div align="center">

# 🎵 Lala

**現代化、輕量級多格式文字編輯器**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Tests](https://img.shields.io/badge/tests-171%20passing-brightgreen.svg)]()

[English](./docs/readmeLangs/README_en.md) | [日本語](../../README.md) | [简体中文](README_zh-CN.md) | [繁體中文](README_zh-TW.md) | [Русский](README_ru.md) | [فارسی](README_fa.md) | [العربية](README_ar.md)

</div>

---

## 💡 什麼是Lala？

Lala是用**Rust**編寫的下一代文字編輯器。使用純Rust GUI框架**egui**，實現了輕量級和高速效能。

---

## ✨ 主要功能

### 🎨 多格式支援
- **Markdown**: 即時預覽，語法高亮
- **HTML**: 解析和渲染
- **LaTeX**: 數學符號的Unicode轉換預覽
- **Mermaid**: 流程圖和時序圖視覺化

### 🚀 高效能
- **輕量級**: Rust零成本抽象實現快速啟動（<100ms）
- **高效**: 使用Rope資料結構快速編輯大檔案
- **原生**: 無需Electron，最小化系統資源使用

### 🌏 完整的中文支援
- **IME支援**: 原生支援中文、日文和韓文輸入法
- **Unicode支援**: 支援所有Unicode字元，包括表情符號和符號

### 🤖 AI整合（可選）
- **Gemini API整合**: 自動文字改進
- **語法糾正**: 自動檢測和修復拼寫/語法錯誤
- **摘要**: 長文字自動摘要

---

## 📦 安裝

### 從Cargo安裝（推薦）

```bash
cargo install lala
```

### 從原始碼建置

```bash
# 複製儲存庫
git clone https://github.com/clearclown/lala.git
cd lala

# 發布建置
cargo build --release

# 二進位檔案生成在 target/release/lala
cargo install --path .
```

---

## 🚀 使用

### GUI模式啟動

```bash
# 啟動空編輯器
lala

# 開啟特定檔案
lala README.md
```

### 鍵盤快捷鍵

| 快捷鍵 | 功能 |
|--------|------|
| `Ctrl+N` | 新建檔案 |
| `Ctrl+O` | 開啟檔案 |
| `Ctrl+S` | 儲存 |
| `Ctrl+F` | 搜尋 |
| `Ctrl+P` | 切換預覽 |

---

## 📄 授權

本專案採用雙授權：

- **MIT授權** ([LICENSE-MIT](../../LICENSE-MIT))
- **Apache授權 2.0** ([LICENSE-APACHE](../../LICENSE-APACHE))

您可以選擇任一授權。

---

<div align="center">

Made with ❤️ by the Lala contributors

</div>
