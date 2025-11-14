# Lala 编辑器

**其他语言**: [English](../README.md) | [日本語](README_ja.md) | [فارسی](README_fa.md) | [العربية](README_ar.md) | [繁體中文](README_zh-TW.md) | [Русский](README_ru.md)

现代化的轻量级文本编辑器，支持多格式预览，使用 Rust 构建。

## 功能特性

### 图形界面 (GUI)

- 📁 **文件树视图** 异步加载
- 🎨 **语法高亮** 支持多种编程语言
- 🔍 **高级搜索和替换** 支持正则表达式
- ⌨️ **快捷键** 提高工作效率
- 🖥️ **现代化界面** 使用 egui/eframe 构建

### 命令行界面 (CLI)

- 📝 **Markdown 预览** 带彩色格式化
- 🌐 **HTML 预览** 纯文本渲染
- 📊 **Mermaid 图表预览** ASCII 艺术风格
- 📐 **LaTeX 预览** Unicode 数学符号
- 🎨 **彩色输出** (可禁用)
- 🚀 **高性能** 无需 WebView 依赖

## 安装

### Cargo (crates.io)

```bash
cargo install lala
```

### Linux

#### Debian/Ubuntu

```bash
sudo apt install lala
```

或从 `.deb` 文件安装：

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

#### 通用安装脚本 (Linux/macOS)

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

#### 安装程序

从[发布页面](https://github.com/yourusername/lala/releases)下载安装程序。

## 使用方法

### GUI 模式

```bash
# 打开编辑器
lala

# 打开指定文件
lala file.txt

# 打开文件夹
lala /path/to/directory
```

### CLI 模式 - Markdown 预览

```bash
# 预览 Markdown 文件
lala preview document.md

# 无色预览
lala preview --no-color document.md
```

### CLI 模式 - HTML 预览

```bash
# 预览 HTML 文件
lala html index.html

# 无色预览
lala html --no-color page.html
```

### CLI 模式 - Mermaid 预览

```bash
# 预览 Mermaid 图表
lala mermaid diagram.mmd

# 无色预览
lala mermaid --no-color flowchart.mmd
```

支持的图表类型：
- 流程图 (Flowchart)
- 时序图 (Sequence Diagram)
- 类图 (Class Diagram)
- 状态图 (State Diagram)
- 实体关系图 (ER Diagram)
- 甘特图 (Gantt Chart)
- 饼图 (Pie Chart)

### CLI 模式 - LaTeX 预览

```bash
# 预览 LaTeX 文档
lala latex document.tex

# 无色预览
lala latex --no-color paper.tex
```

支持的功能：
- 希腊字母 (α, β, γ, ...)
- 数学运算符 (√, ∫, ∑, ∏, ...)
- 分数和方程
- 上标和下标
- 矩阵

### 完整指南

更多信息请查看我们的文档：

- [安装指南](zh-CN/安装.md)
- [CLI 使用指南](zh-CN/CLI使用.md)
- [格式支持](zh-CN/格式支持.md)
- [贡献指南](../CONTRIBUTING.md)

## 从源代码构建

### 前置要求

- Rust 1.70 或更高版本
- Cargo
- 系统库：
  - Linux: `libxcb-dev`
  - macOS: 无需额外库
  - Windows: 无需额外库

### 构建步骤

```bash
# 克隆仓库
git clone https://github.com/yourusername/lala.git
cd lala

# 构建发布版本
cargo build --release

# 运行测试
cargo test

# 本地安装
cargo install --path .
```

## 架构设计

Lala 编辑器采用模块化架构：

```
lala/
├── src/
│   ├── main.rs          # 应用程序入口
│   ├── app.rs           # GUI 主应用逻辑
│   ├── cli/             # 命令行界面模块
│   │   ├── mod.rs       # CLI 配置和参数解析
│   │   ├── markdown_view.rs   # Markdown 渲染
│   │   ├── html_view.rs       # HTML 渲染
│   │   ├── mermaid_view.rs    # Mermaid 图表渲染
│   │   └── latex_view.rs      # LaTeX 渲染
│   ├── editor/          # 编辑器组件
│   └── file_tree/       # 文件树视图
├── tests/               # 集成测试
└── packaging/           # 打包脚本
```

## 贡献

我们欢迎贡献！请阅读我们的[贡献指南](../CONTRIBUTING.md)。

### 贡献领域

- 🐛 报告和修复 Bug
- ✨ 提议和实现新功能
- 📝 改进文档
- 🌍 翻译成其他语言
- 🎨 改进用户界面
- ⚡ 性能优化

## 许可证

本项目采用双许可证：

- MIT 许可证 ([LICENSE-MIT](../LICENSE-MIT))
- Apache 2.0 许可证 ([LICENSE-APACHE](../LICENSE-APACHE))

您可以选择其中任何一个。

## 致谢

Lala 编辑器使用了以下优秀的开源库：

- [egui](https://github.com/emilk/egui) - UI 框架
- [ropey](https://github.com/cessen/ropey) - 文本 Rope 数据结构
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown 解析器
- [syntect](https://github.com/trishume/syntect) - 语法高亮
- [html2text](https://github.com/jugglerchris/rust-html2text) - HTML 转文本
- 以及更多...

## 链接

- 🏠 [项目主页](https://github.com/yourusername/lala)
- 📦 [Crates.io](https://crates.io/crates/lala)
- 📖 [文档](https://github.com/yourusername/lala/tree/main/docs)
- 🐛 [问题反馈](https://github.com/yourusername/lala/issues)
- 💬 [讨论区](https://github.com/yourusername/lala/discussions)

---

由 Lala 社区用 ❤️ 构建
