# Lala Editor

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Crates.io](https://img.shields.io/crates/v/lala.svg)](https://crates.io/crates/lala)
[![CI](https://github.com/yourusername/lala/workflows/CI/badge.svg)](https://github.com/yourusername/lala/actions)

**Read this in other languages**: [日本語](docs/README_ja.md) | [فارسی](docs/README_fa.md) | [العربية](docs/README_ar.md) | [简体中文](docs/README_zh-CN.md) | [繁體中文](docs/README_zh-TW.md) | [Русский](docs/README_ru.md)

A modern, lightweight text editor with **both GUI and CLI** built with Rust, egui, and eframe.

**NEW**: Now with comprehensive command-line interface! Preview **Markdown, HTML, Mermaid diagrams, and LaTeX** documents with beautiful terminal formatting, plus file viewing with line numbers - all from your terminal!

## Features

### ✨ File Tree View
- **Tree Display**: Browse directories and files in a hierarchical tree view in the left sidebar
- **Directory Expansion**: Click folder icons to expand/collapse directories
- **File Opening**: Click on files to open them in editor tabs
- **Async Loading**: Non-blocking directory scanning prevents UI freezes
- **Smart Filtering**:
  - Respects `.gitignore` files (when in a git repository)
  - Filters out `.git` directories automatically
  - Does not follow symbolic links (for security)
- **Error Handling**: Displays access denied messages for restricted directories
- **Performance**: Optimized for large directories (like `node_modules`)

### 📝 Text Editing
- **Text Display**: Efficiently displays text from Rope buffer
- **Text Input**: Full keyboard input support (characters, backspace, enter, delete)
- **Cursor Management**: Smooth cursor navigation and positioning
- **Multi-tab Editing**: Open multiple files simultaneously
- **Tab Management**: Close tabs with the × button
- **Unsaved Changes Indicator**: Visual indicators for modified files

### 🔍 Advanced Search
- **Grep Integration**: Fast file-wide search using ripgrep
- **Multi-file Search**: Search across multiple files in directories
- **Replace Functionality**: Find and replace text in files
- **Regex Support**: Full regular expression support for advanced patterns
- **Search Panel**: Dedicated panel for search operations
- **Context Display**: Shows matching lines with context

### 🎨 Syntax Highlighting
- **Code Editor**: Monospace font for code editing
- **Language Detection**: Automatic syntax detection based on file extension
- **Color Themes**: Syntax highlighting with customizable themes
- **Multiple Languages**: Support for various programming languages via syntect

### 📖 Multi-Format Preview
- **Markdown Preview**: GitHub-style rendering with rich formatting
- **HTML Preview**: Beautiful HTML display with tables, lists, and links
- **Mermaid Diagrams**: ASCII art flowcharts, sequence diagrams, and more
- **LaTeX Documents**: Unicode math symbols (√, ∫, ∑, α, β, etc.)
- **Terminal Rendering**: All formats viewable in CLI with color support
- **Pure Rust**: No WebView dependencies, rendered directly with egui or terminal

## Architecture

This is a single-crate application with a modular structure:

```
lala/
├── Cargo.toml          # Package configuration
├── README.md           # This file
└── src/
    ├── main.rs         # Entry point
    ├── lib.rs          # Library exports
    ├── app.rs          # Application state
    ├── cli/            # CLI argument parsing
    ├── core/           # Core editing engine
    │   ├── error.rs    # Error types
    │   ├── rope.rs     # Text buffer (Rope-based)
    │   └── mod.rs      # Module exports
    ├── core_engine/    # Core engine buffer implementation
    │   ├── buffer.rs   # Text buffer
    │   └── mod.rs      # Module exports
    ├── file_tree/      # File tree view
    │   └── mod.rs      # File tree implementation
    ├── gui/            # GUI components
    │   ├── app.rs      # Main application
    │   ├── app_state.rs# Application state
    │   ├── editor.rs   # Editor panel
    │   ├── highlighting.rs # Syntax highlighting
    │   ├── markdown_preview.rs # Markdown renderer
    │   ├── search_panel.rs # Search UI
    │   ├── grep_panel.rs   # Grep UI
    │   ├── tab.rs      # Tab management
    │   └── mod.rs      # Module exports
    └── search/         # Search functionality
        ├── buffer_search.rs # Buffer search
        ├── grep.rs     # Grep integration
        └── mod.rs      # Module exports
```

### Core Components

#### Text Buffer (`core/rope.rs`)
The core editing engine uses a Rope-based text buffer for efficient editing:
- Supports efficient insert/delete operations on large files
- Character-indexed operations
- Line-based operations
- Memory-efficient for large documents

#### Async Directory Loading (`file_tree/`)
The file tree uses async I/O to prevent UI freezes:
1. **Background Thread**: Directory scanning happens in separate threads using `tokio::spawn`
2. **Channel Communication**: Results are sent to the GUI thread via `flume` channels
3. **Incremental Updates**: The tree updates as directories are scanned
4. **Depth Limiting**: Initial load is limited to prevent overwhelming the UI

#### Syntax Highlighting (`gui/highlighting.rs`)
Uses `syntect` for professional-grade syntax highlighting:
- Supports multiple languages
- Theme customization
- Fast and efficient

#### Markdown Preview (`gui/markdown_preview.rs`)
Pure Rust Markdown rendering:
- Uses `pulldown-cmark` for parsing
- Renders directly to egui widgets
- No WebView or HTML dependencies
- Real-time updates

## Building and Running

### Prerequisites
- Rust 1.70 or later
- Cargo

### Build
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Run

#### GUI Mode (Default)
```bash
# Run in development mode
cargo run

# Run release build
./target/release/lala

# Open specific file or directory
./target/release/lala file.txt
./target/release/lala /path/to/project
```

#### CLI Mode (NEW!)
```bash
# Preview Markdown with beautiful formatting
./target/release/lala markdown README.md

# Preview HTML files
./target/release/lala html page.html

# Preview Mermaid diagrams
./target/release/lala mermaid diagram.mmd

# Preview LaTeX documents with Unicode math symbols
./target/release/lala latex document.tex

# View file with line numbers
./target/release/lala view -n src/main.rs

# See all available commands
./target/release/lala --help

# Get help for specific command
./target/release/lala markdown --help
./target/release/lala html --help
./target/release/lala mermaid --help
./target/release/lala latex --help
```

**See [CLI Usage Guide](docs/ja/CLI使い方.md) for complete CLI documentation and [Format Support](docs/ja/フォーマット対応.md) for format-specific details.**

### Test
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run tests for specific module
cargo test --lib core
```

### Lint
```bash
# Check for common mistakes
cargo clippy --all-targets --all-features

# Apply automatic fixes
cargo clippy --fix
```

## Dependencies

Core dependencies:
- **eframe** / **egui**: Immediate mode GUI framework
- **ropey**: Rope data structure for efficient text editing
- **tokio**: Async runtime for non-blocking operations
- **ignore**: Git-aware file walking (respects .gitignore)
- **flume**: Fast multi-producer multi-consumer channels
- **syntect**: Syntax highlighting engine
- **pulldown-cmark**: Markdown parser
- **html2text**: HTML to text conversion
- **scraper**: HTML parsing and DOM manipulation
- **regex**: Regular expression support
- **anyhow**: Error handling
- **thiserror**: Error type derivation
- **serde**: Serialization support
- **clap**: Command-line argument parsing
- **colored**: Terminal colors and formatting
- **terminal_size**: Terminal size detection

## Keyboard Shortcuts

### Editor (GUI Mode)
- **Ctrl+S**: Save file
- **Ctrl+Z**: Undo (planned)
- **Ctrl+Y** or **Ctrl+Shift+Z**: Redo (planned)

### Navigation (GUI Mode)
- **Arrow Keys**: Move cursor
- **Page Up/Down**: Scroll
- **Home/End**: Jump to line start/end

### Text Editing (GUI Mode)
- **Backspace**: Delete character before cursor
- **Delete**: Delete character at cursor
- **Enter**: Insert newline
- **Tab**: Insert tab/spaces

## CLI Commands

```bash
# Markdown preview
lala markdown <FILE> [--no-color]

# HTML preview
lala html <FILE> [--no-color]

# Mermaid diagram preview
lala mermaid <FILE> [--no-color]

# LaTeX document preview
lala latex <FILE> [--no-color]

# File viewing
lala view <FILE> [-n|--line-numbers]

# Help
lala --help
lala markdown --help
lala html --help
lala mermaid --help
lala latex --help
lala view --help
```

**For detailed CLI usage, examples, and tips, see [CLI Usage Guide](docs/ja/CLI使い方.md) and [Format Support](docs/ja/フォーマット対応.md).**

## Development

### Project Status

This project merges multiple feature branches:
- ✅ File tree view with async loading
- ✅ Basic text editing functionality
- ✅ GUI foundation with tab system
- ✅ CLI argument parser
- ✅ Core engine implementation
- ✅ Syntax highlighting
- ✅ Markdown preview
- ✅ Advanced search and replace

### Performance Considerations

The editor is designed for efficiency:
- **Rope Data Structure**: O(log n) insertions and deletions
- **Async I/O**: Non-blocking file operations
- **Incremental Rendering**: Only redraw changed regions
- **Memory Efficient**: Suitable for large files (tested up to 10MB)

### Security Considerations
- **Symlink Handling**: Symbolic links are not followed to prevent infinite loops
- **Permission Errors**: Access denied errors are caught and displayed gracefully
- **Input Sanitization**: All user input is properly validated

## Future Enhancements

Planned features:
- Multiple cursors support
- Split view editing
- Integrated terminal
- Git integration with diff view
- Plugin system
- Customizable keybindings
- Themes and color schemes
- Find in files with preview
- Code folding
- Minimap view
- LSP (Language Server Protocol) support

## Installation

### Quick Install

```bash
# Cargo (Recommended)
cargo install lala

# Universal installer (Linux/macOS)
curl -sSL https://raw.githubusercontent.com/yourusername/lala/main/packaging/scripts/install.sh | bash

# Homebrew (macOS)
brew install lala

# APT (Debian/Ubuntu)
sudo apt install lala

# Pacman (Arch Linux)
yay -S lala

# Chocolatey (Windows)
choco install lala
```

For detailed installation instructions, see [Installation Guide](docs/ja/インストール.md).

## Documentation

### For Users
- 📖 [Installation Guide](docs/ja/インストール.md) - Complete installation instructions
- 🚀 [Beginner's Guide](docs/ja/初心者ガイド.md) - Start here if you're new to Lala
- 💻 [CLI Usage Guide](docs/ja/CLI使い方.md) - Command-line interface documentation
- 📝 [Format Support](docs/ja/フォーマット対応.md) - Supported formats and rendering details

### For Developers
- 🏗️ [Implementation Details](docs/development/implementation.md) - Architecture and design
- 📋 [Project Status](docs/development/project-status.md) - Current development status
- 🔧 [CLI Design](docs/development/cli-design.md) - CLI architecture

### For Contributors
- 🤝 [Contributing Guidelines](docs/community/CONTRIBUTING.md) - How to contribute
- 📜 [Code of Conduct](docs/community/CODE_OF_CONDUCT.md) - Community standards
- 🔒 [Security Policy](docs/community/SECURITY.md) - Reporting security issues
- 📰 [Changelog](docs/CHANGELOG.md) - Release history

### Package Maintainers
- 📦 [Publishing Guide](docs/development/publishing-guide.md) - How to publish to package managers

## Contributing

We welcome contributions! Please read our [Contributing Guidelines](docs/community/CONTRIBUTING.md) before submitting pull requests.

## License

This project is dual-licensed under:
- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

You may choose either license.

## Links

- 🏠 [Homepage](https://github.com/yourusername/lala)
- 📦 [Crates.io](https://crates.io/crates/lala)
- 📖 [Documentation](https://github.com/yourusername/lala/tree/main/docs)
- 🐛 [Issue Tracker](https://github.com/yourusername/lala/issues)
- 💬 [Discussions](https://github.com/yourusername/lala/discussions)

---

Made with ❤️ by the Lala community
