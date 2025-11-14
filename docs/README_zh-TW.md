# Lala 編輯器

**其他語言**: [English](../README.md) | [日本語](README_ja.md) | [فارسی](README_fa.md) | [العربية](README_ar.md) | [简体中文](README_zh-CN.md) | [Русский](README_ru.md)

現代化的輕量級文字編輯器，支援多格式預覽，使用 Rust 建置。

## 功能特性

### 圖形介面 (GUI)

- 📁 **檔案樹狀檢視** 非同步載入
- 🎨 **語法突顯** 支援多種程式語言
- 🔍 **進階搜尋與取代** 支援正規表達式
- ⌨️ **快速鍵** 提升工作效率
- 🖥️ **現代化介面** 使用 egui/eframe 建置

### 命令列介面 (CLI)

- 📝 **Markdown 預覽** 帶彩色格式化
- 🌐 **HTML 預覽** 純文字渲染
- 📊 **Mermaid 圖表預覽** ASCII 藝術風格
- 📐 **LaTeX 預覽** Unicode 數學符號
- 🎨 **彩色輸出** (可停用)
- 🚀 **高效能** 無需 WebView 相依性

## 安裝

### Cargo (crates.io)

```bash
cargo install lala
```

### Linux

#### Debian/Ubuntu

```bash
sudo apt install lala
```

或從 `.deb` 檔案安裝：

```bash
wget https://github.com/yourusername/lala/releases/latest/download/lala-linux-x86_64.deb
sudo dpkg -i lala-linux-x86_64.deb
```

#### Arch Linux

```bash
yay -S lala
```

或：

```bash
paru -S lala
```

#### Fedora/RHEL

```bash
sudo dnf install lala
```

或：

```bash
sudo yum install lala
```

#### 通用安裝指令碼 (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/yourusername/lala/main/packaging/scripts/install.sh | bash
```

### macOS

#### Homebrew

```bash
brew install lala
```

### Windows

#### Chocolatey

```bash
choco install lala
```

#### Scoop

```bash
scoop install lala
```

#### 安裝程式

從[發布頁面](https://github.com/yourusername/lala/releases)下載安裝程式。

## 使用方法

### GUI 模式

```bash
# 開啟編輯器
lala

# 開啟指定檔案
lala file.txt

# 開啟資料夾
lala /path/to/directory
```

### CLI 模式 - Markdown 預覽

```bash
# 預覽 Markdown 檔案
lala preview document.md

# 無色預覽
lala preview --no-color document.md
```

### CLI 模式 - HTML 預覽

```bash
# 預覽 HTML 檔案
lala html index.html

# 無色預覽
lala html --no-color page.html
```

### CLI 模式 - Mermaid 預覽

```bash
# 預覽 Mermaid 圖表
lala mermaid diagram.mmd

# 無色預覽
lala mermaid --no-color flowchart.mmd
```

支援的圖表類型：
- 流程圖 (Flowchart)
- 循序圖 (Sequence Diagram)
- 類別圖 (Class Diagram)
- 狀態圖 (State Diagram)
- 實體關係圖 (ER Diagram)
- 甘特圖 (Gantt Chart)
- 圓餅圖 (Pie Chart)

### CLI 模式 - LaTeX 預覽

```bash
# 預覽 LaTeX 文件
lala latex document.tex

# 無色預覽
lala latex --no-color paper.tex
```

支援的功能：
- 希臘字母 (α, β, γ, ...)
- 數學運算子 (√, ∫, ∑, ∏, ...)
- 分數與方程式
- 上標與下標
- 矩陣

### 完整指南

更多資訊請查看我們的文件：

- [安裝指南](zh-TW/安裝.md)
- [CLI 使用指南](zh-TW/CLI使用.md)
- [格式支援](zh-TW/格式支援.md)
- [貢獻指南](../CONTRIBUTING.md)

## 從原始碼建置

### 前置需求

- Rust 1.70 或更高版本
- Cargo
- 系統函式庫：
  - Linux: `libxcb-dev`
  - macOS: 無需額外函式庫
  - Windows: 無需額外函式庫

### 建置步驟

```bash
# 複製儲存庫
git clone https://github.com/yourusername/lala.git
cd lala

# 建置發布版本
cargo build --release

# 執行測試
cargo test

# 本機安裝
cargo install --path .
```

## 架構設計

Lala 編輯器採用模組化架構：

```
lala/
├── src/
│   ├── main.rs          # 應用程式進入點
│   ├── app.rs           # GUI 主應用邏輯
│   ├── cli/             # 命令列介面模組
│   │   ├── mod.rs       # CLI 設定與參數解析
│   │   ├── markdown_view.rs   # Markdown 渲染
│   │   ├── html_view.rs       # HTML 渲染
│   │   ├── mermaid_view.rs    # Mermaid 圖表渲染
│   │   └── latex_view.rs      # LaTeX 渲染
│   ├── editor/          # 編輯器元件
│   └── file_tree/       # 檔案樹狀檢視
├── tests/               # 整合測試
└── packaging/           # 打包指令碼
```

## 貢獻

我們歡迎貢獻！請閱讀我們的[貢獻指南](../CONTRIBUTING.md)。

### 貢獻領域

- 🐛 回報與修正 Bug
- ✨ 提議與實作新功能
- 📝 改進文件
- 🌍 翻譯成其他語言
- 🎨 改進使用者介面
- ⚡ 效能最佳化

## 授權條款

本專案採用雙授權：

- MIT 授權 ([LICENSE-MIT](../LICENSE-MIT))
- Apache 2.0 授權 ([LICENSE-APACHE](../LICENSE-APACHE))

您可以選擇其中任何一個。

## 致謝

Lala 編輯器使用了以下優秀的開源函式庫：

- [egui](https://github.com/emilk/egui) - UI 框架
- [ropey](https://github.com/cessen/ropey) - 文字 Rope 資料結構
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown 解析器
- [syntect](https://github.com/trishume/syntect) - 語法突顯
- [html2text](https://github.com/jugglerchris/rust-html2text) - HTML 轉文字
- 以及更多...

## 連結

- 🏠 [專案首頁](https://github.com/yourusername/lala)
- 📦 [Crates.io](https://crates.io/crates/lala)
- 📖 [文件](https://github.com/yourusername/lala/tree/main/docs)
- 🐛 [問題回報](https://github.com/yourusername/lala/issues)
- 💬 [討論區](https://github.com/yourusername/lala/discussions)

---

由 Lala 社群用 ❤️ 建置
