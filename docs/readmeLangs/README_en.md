<div align="center">

# 🎵 Lala

**Modern, Lightweight Multi-Format Text Editor**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Tests](https://img.shields.io/badge/tests-171%20passing-brightgreen.svg)]()

[English](./docs/readmeLangs/README_en.md) | [日本語](README.md) | [简体中文](./docs/readmeLangs/README_zh-CN.md) | [繁體中文](./docs/readmeLangs/README_zh-TW.md) | [Русский](./docs/readmeLangs/README_ru.md) | [فارسی](./docs/readmeLangs/README_fa.md) | [العربية](./docs/readmeLangs/README_ar.md)

</div>

---

## 📸 Screenshots

### Markdown Editing with Dark Theme
*Code block syntax highlighting, real-time preview*

### Writing with Light Theme
*Eye-friendly colors, full IME support*

### Multi-Format Preview
*Instant preview for Markdown, HTML, LaTeX, and Mermaid diagrams*

---

## 💡 What is Lala?

Lala is a next-generation text editor written in **Rust**. Using the pure Rust GUI framework **egui**, it achieves lightweight and high-speed performance.

### Why Lala?

Existing text editors face the following challenges:

- 🐌 **Heavy**: Electron-based editors consume significant memory and CPU
- 🌐 **Lack of Native Feel**: Web-based editors lack responsiveness
- 📝 **Inconvenient Preview**: Insufficient real-time preview for Markdown and other formats
- 🌏 **Incomplete IME Support**: Poor support for complex input methods like Japanese and Chinese

Lala solves these problems.

---

## ✨ Key Features

### 🎨 Multi-Format Support
- **Markdown**: Real-time preview, syntax highlighting
- **HTML**: Parse & render
- **LaTeX**: Unicode conversion preview for math symbols
- **Mermaid**: Flowchart and sequence diagram visualization

### 🚀 High Performance
- **Lightweight**: Fast startup (<100ms) with Rust's zero-cost abstractions
- **Efficient**: Fast editing of large files using Rope data structure
- **Native**: No Electron, minimal system resource usage

### 🌏 Full Japanese Support
- **IME Support**: Native support for Japanese, Chinese, and Korean input methods
- **Unicode Support**: All Unicode characters including emojis and symbols

### 🤖 AI Integration (Optional)
- **Gemini API Integration**: Automatic text improvement
- **Grammar Correction**: Auto-detect and fix spelling/grammar errors
- **Summarization**: Automatic summarization of long texts

### 🔍 Powerful Search
- **Buffer Search**: Fast search & replace with regex support
- **Grep**: Fast search across entire projects
- **.gitignore Support**: Automatically excludes unnecessary files

### 🎨 Themes
- **Dark Theme**: Optimal for extended work sessions
- **Light Theme**: Customized high-visibility color scheme

---

## 📦 Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo (included with Rust toolchain)

### Install from Cargo (Recommended)

```bash
cargo install lala
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/clearclown/lala.git
cd lala

# Release build
cargo build --release

# Binary generated at target/release/lala
# Optional: Install to system
cargo install --path .
```

### Enable AI Features (Optional)

```bash
# Build with llm feature
cargo build --release --features llm

# Or
cargo install --path . --features llm
```

---

## 🚀 Usage

### Launch in GUI Mode

```bash
# Launch empty editor
lala

# Open specific file
lala README.md

# Open directory
lala ./docs
```

### CLI Mode (Preview)

```bash
# Preview Markdown in terminal
lala README.md --preview

# Preview HTML
lala index.html --preview

# Preview LaTeX
lala document.tex --preview
```

### Keyboard Shortcuts

| Shortcut | Function |
|----------|----------|
| `Ctrl+N` | New file |
| `Ctrl+O` | Open file |
| `Ctrl+S` | Save |
| `Ctrl+Shift+S` | Save as |
| `Ctrl+F` | Search |
| `Ctrl+H` | Replace |
| `Ctrl+Shift+F` | Search entire project (Grep) |
| `Ctrl+P` | Toggle preview |
| `Esc` | Close panels |

---

## ⚙️ Configuration

### AI Feature Setup

1. **Get Gemini API Key**
   - Obtain API key from [Google AI Studio](https://ai.google.dev/tutorials/setup)

2. **Set Environment Variable**
   ```bash
   export GEMINI_API_KEY="your-api-key-here"
   ```

3. **Or Configure via GUI Settings**
   - Open `Tools > Settings` menu
   - Enter API key
   - Check "Enable AI Features"

### Using AI Features
- **🤖 Improve Markdown**: Enhance Markdown structure and formatting
- **✨ Fix Grammar**: Correct grammar and spelling mistakes
- **📝 Summarize**: Summarize text

---

## 🗑️ Uninstall

```bash
# If installed via Cargo
cargo uninstall lala

# Or manually delete
rm ~/.cargo/bin/lala  # Linux/macOS
# Windows: Delete %USERPROFILE%\.cargo\bin\lala.exe
```

---

## 📚 Documentation

### Architecture

```
lala/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli/                 # CLI interface
│   │   ├── markdown_view.rs # Markdown preview
│   │   ├── html_view.rs     # HTML preview
│   │   ├── latex_view.rs    # LaTeX preview
│   │   └── mermaid_view.rs  # Mermaid diagram preview
│   ├── gui/                 # GUI interface
│   │   ├── app.rs          # Main application
│   │   ├── theme.rs        # Theme settings
│   │   ├── dialogs.rs      # Dialog UI
│   │   ├── menu.rs         # Menu bar
│   │   ├── previews.rs     # Preview features
│   │   ├── markdown_preview.rs  # Markdown renderer
│   │   └── search_panel.rs # Search panel
│   ├── core_engine/        # Core engine
│   │   └── buffer.rs       # Text buffer management
│   ├── search/             # Search features
│   │   ├── buffer_search.rs # Buffer search
│   │   └── grep.rs         # Grep search
│   ├── llm/                # LLM integration (optional)
│   │   └── mod.rs          # Gemini API client
│   └── file_tree/          # File tree
└── tests/                  # 171 tests
```

### Tech Stack

- **Language**: Rust 2021 Edition
- **GUI Framework**: egui 0.33 + eframe
- **Text Processing**: ropey (Rope structure)
- **Markdown Parser**: pulldown-cmark
- **Syntax Highlighting**: syntect
- **Async Runtime**: Tokio
- **Regular Expressions**: regex
- **AI Integration**: reqwest + Gemini API

### Testing

```bash
# Run all tests
cargo test --all-features

# Run specific test suite
cargo test --test core_engine_test
cargo test --test llm_test
cargo test --test preview_test

# Integration tests
cargo test --test end_to_end_test
```

Currently, **171 tests** all pass:
- Core Engine: 9 tests
- LLM Integration: 12 tests
- Preview: 26 tests
- File Operations: 23 tests
- Search: 15 tests
- Others: 86 tests

---

## 🤝 Contributing

Contributions are welcome! You can contribute in the following ways:

### Bug Reports & Feature Requests
1. Create a new issue at [Issues](https://github.com/clearclown/lala/issues)
2. For bugs, include reproduction steps
3. For feature requests, explain use cases

### Pull Requests
1. Fork this repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m '✨ Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Create Pull Request

### Development Guidelines
- Code style: Use `cargo fmt`
- Linter: Check with `cargo clippy`
- Tests: Always add tests for new features
- Documentation: Add doc comments for public APIs

---

## 📖 Resources

- [Official Documentation](https://github.com/clearclown/lala/blob/main/README.md)
- [Issue Tracker](https://github.com/clearclown/lala/issues)
- [Changelog](https://github.com/clearclown/lala/blob/main/CHANGELOG.md)
- [Roadmap](https://github.com/clearclown/lala/blob/main/ROADMAP.md)

### Related Projects
- [egui](https://github.com/emilk/egui) - Pure Rust GUI framework
- [ropey](https://github.com/cessen/ropey) - Fast text Rope library
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown parser
- [syntect](https://github.com/trishume/syntect) - Syntax highlighting library

---

## 📄 License

This project is dual-licensed:

- **MIT License** ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License 2.0** ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache License 2.0, shall be dual-licensed as above, without any additional terms or conditions.

---

## 🙏 Acknowledgments

Thanks to the following open source projects:
- [The Rust Programming Language](https://www.rust-lang.org/)
- [egui Community](https://github.com/emilk/egui)
- All contributors

---

## 📊 Project Statistics

- **Language**: 100% Rust
- **Lines of Code**: ~15,000
- **Tests**: 171 (all passing)
- **Files**: 40+ modules
- **Dependencies**: Minimal (security-focused)

---

<div align="center">

**[⬆ Back to top](#-lala)**

Made with ❤️ by the Lala contributors

</div>
