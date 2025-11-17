# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-17

### Added
- Modern text editor with GUI and CLI support
- Multi-format support:
  - Markdown with real-time preview and syntax highlighting
  - HTML parsing and rendering
  - LaTeX with Unicode conversion for math symbols
  - Mermaid diagram visualization
- Syntax highlighting for code blocks using syntect
- AI integration with Gemini API (optional feature)
  - Text improvement
  - Grammar correction
  - Summarization
- IME support for Japanese/CJK input
- Light/Dark theme support with custom color schemes
- Search and replace functionality with regex support
- Grep functionality for project-wide search
- Multiple tab support
- File tree navigation

### Features
- Fast startup time (<100ms)
- Lightweight (no Electron, pure Rust)
- Cross-platform support (Windows, Linux, macOS)
- Efficient text editing with Rope data structure
- .gitignore support in file search
- Zero-cost abstractions for high performance

### Technical
- Built with Rust 2021 edition
- GUI framework: egui/eframe 0.33
- Text editing: ropey 1.6
- Markdown parsing: pulldown-cmark 0.12
- Syntax highlighting: syntect 5.3
- Async runtime: tokio 1.48

[0.1.0]: https://github.com/clearclown/lala/releases/tag/v0.1.0
