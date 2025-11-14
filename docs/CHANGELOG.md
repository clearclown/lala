# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Multiple cursors support
- Split view editing
- Integrated terminal
- Git integration with diff view
- Plugin system
- Customizable keybindings
- Themes and color schemes
- LSP (Language Server Protocol) support

## [0.1.0] - 2025-11-14

### Added
- Initial release of Lala Editor
- GUI mode with egui/eframe
- CLI mode with multiple subcommands
- File tree view with async loading
- `.gitignore` support in file tree
- Multi-tab editing
- Syntax highlighting for multiple languages
- Markdown preview (CLI and GUI)
- HTML preview (CLI)
- Mermaid diagram preview (CLI)
- LaTeX document preview with Unicode math symbols (CLI)
- Advanced search and replace with regex support
- Grep integration using ripgrep
- Multi-format file viewing with line numbers
- Comprehensive documentation in multiple languages
- Package manager support:
  - Cargo (crates.io)
  - Arch Linux (AUR)
  - Debian/Ubuntu (.deb)
  - Fedora/RHEL (RPM)
  - Homebrew (macOS/Linux)
  - Chocolatey (Windows)
- GitHub Actions CI/CD workflows
- Cross-platform binaries (Linux, macOS, Windows)

### Features

#### File Tree
- Hierarchical tree view
- Directory expansion/collapse
- Async non-blocking loading
- Smart filtering (.gitignore, .git)
- Symlink protection

#### Text Editing
- Rope-based text buffer for efficiency
- Full keyboard input support
- Multi-tab interface
- Unsaved changes indicators

#### Search & Replace
- File-wide search
- Multi-file search across directories
- Full regex support
- Context display

#### Syntax Highlighting
- Multiple programming languages
- Customizable themes
- Automatic language detection

#### Multi-Format Preview
- **Markdown**: GitHub-style rendering, rich formatting
- **HTML**: Tables, lists, links, styled display
- **Mermaid**: ASCII art diagrams (flowcharts, sequence, etc.)
- **LaTeX**: Unicode math symbols (√, ∫, ∑, α, β)

#### CLI Commands
- `markdown` - Preview Markdown files
- `html` - Preview HTML files
- `mermaid` - Preview Mermaid diagrams
- `latex` - Preview LaTeX documents
- `view` - View files with optional line numbers

### Documentation
- English README and guides
- Japanese (日本語) complete documentation
- Persian (فارسی) README
- Arabic (العربية) README
- Simplified Chinese (简体中文) README
- Traditional Chinese (繁體中文) README
- Russian (Русский) README
- Beginner-friendly guides
- Installation instructions for all platforms
- CLI usage examples
- Format support details
- Packaging and distribution guide

### Infrastructure
- MIT OR Apache-2.0 dual license
- GitHub Actions workflows
- Issue and PR templates
- Contributing guidelines
- Code of Conduct
- Security policy
- Comprehensive .gitignore
- EditorConfig support

## [0.0.1] - 2025-11-01

### Added
- Project initialization
- Basic project structure
- Core editing engine with Rope buffer
- Initial GUI implementation
- Basic file operations

---

## Release Notes Template

```
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Changes in existing functionality

### Deprecated
- Soon-to-be removed features

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security updates
```

[Unreleased]: https://github.com/yourusername/lala/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/lala/releases/tag/v0.1.0
[0.0.1]: https://github.com/yourusername/lala/releases/tag/v0.0.1
