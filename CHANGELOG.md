# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-17

### Added
- Modern text editor with GUI and CLI support
- Multi-format support (Markdown, HTML, LaTeX, Mermaid)
- Syntax highlighting for code blocks
- Real-time preview for Markdown, HTML, LaTeX, and Mermaid diagrams
- AI integration with Gemini API (optional feature)
- Full IME support for Japanese/CJK input
- Light and Dark theme support
- Search and replace functionality with regex support
- Project-wide grep search with .gitignore support
- Multiple tab support
- File tree navigation

### Features
- Fast startup time (<100ms)
- Lightweight (no Electron dependency)
- Cross-platform support (Windows, Linux, macOS)
- Efficient text editing with Rope data structure
- 171 comprehensive tests covering all features

### Technical
- Built with Rust 2021 Edition
- GUI framework: egui 0.33 + eframe
- Text processing: ropey (Rope structure)
- Markdown parser: pulldown-cmark
- Syntax highlighting: syntect
- Async runtime: Tokio
- Regular expressions: regex
- AI integration: reqwest + Gemini API

[0.1.0]: https://github.com/clearclown/lala/releases/tag/v0.1.0
