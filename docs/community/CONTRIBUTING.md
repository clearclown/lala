# Contributing to Lala Editor

First off, thank you for considering contributing to Lala Editor! It's people like you that make Lala such a great tool.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How Can I Contribute?](#how-can-i-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Style Guidelines](#style-guidelines)
- [Testing](#testing)

## Code of Conduct

This project and everyone participating in it is governed by the [Lala Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/lala.git`
3. Add upstream remote: `git remote add upstream https://github.com/yourusername/lala.git`
4. Create a new branch: `git checkout -b feature/your-feature-name`

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the [existing issues](https://github.com/yourusername/lala/issues) to avoid duplicates.

When creating a bug report, please include:
- Clear and descriptive title
- Detailed steps to reproduce
- Expected vs actual behavior
- Screenshots if applicable
- Environment details (OS, version, installation method)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:
- Clear and descriptive title
- Detailed explanation of the suggested enhancement
- Use cases and examples
- Why this enhancement would be useful

### Code Contributions

1. **Find an Issue**: Look for issues labeled `good first issue` or `help wanted`
2. **Discuss**: Comment on the issue to let others know you're working on it
3. **Implement**: Write your code following our style guidelines
4. **Test**: Add tests and ensure all tests pass
5. **Document**: Update documentation if needed
6. **Submit**: Create a pull request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (included with Rust)
- Git

### Building

```bash
# Clone repository
git clone https://github.com/yourusername/lala.git
cd lala

# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run linter
cargo clippy --all-targets --all-features

# Format code
cargo fmt
```

### Project Structure

```
lala/
├── src/
│   ├── main.rs         # Entry point
│   ├── lib.rs          # Library exports
│   ├── app.rs          # Application state
│   ├── cli/            # CLI modules
│   ├── core/           # Core editing engine
│   ├── file_tree/      # File tree view
│   ├── gui/            # GUI components
│   └── search/         # Search functionality
├── tests/              # Integration tests
├── docs/               # Documentation
└── packaging/          # Package files
```

## Pull Request Process

1. **Update Documentation**: Ensure README.md and other docs are updated
2. **Add Tests**: Include tests for new features
3. **Run Tests**: Ensure all tests pass
4. **Follow Style Guide**: Run `cargo fmt` and `cargo clippy`
5. **Update CHANGELOG**: Add entry to CHANGELOG.md under "Unreleased"
6. **Write Clear Commit Messages**: Use conventional commits format

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Build process or auxiliary tool changes

**Example:**
```
feat(cli): add HTML preview support

Implemented HTML preview with colored terminal output,
table rendering, and link display.

Closes #42
```

### Review Process

1. At least one maintainer will review your PR
2. Address any requested changes
3. Once approved, a maintainer will merge your PR

## Style Guidelines

### Rust Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` and fix all warnings
- Write rustdoc comments for public API
- Use meaningful variable and function names

### Example

```rust
/// Renders Markdown content to terminal with formatting.
///
/// # Arguments
///
/// * `markdown` - The markdown content as a string slice
///
/// # Examples
///
/// ```
/// use lala::cli::markdown_view;
/// markdown_view::render_markdown_to_terminal("# Hello");
/// ```
pub fn render_markdown_to_terminal(markdown: &str) {
    // Implementation
}
```

### Documentation Style

- Use clear, concise language
- Include code examples
- Keep documentation up-to-date
- Support multiple languages where possible

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = "test input";

        // Act
        let result = my_function(input);

        // Assert
        assert_eq!(result, "expected output");
    }
}
```

## Documentation Contributions

Documentation is just as important as code! You can help by:
- Fixing typos or unclear wording
- Adding examples
- Translating documentation
- Writing tutorials or guides

## Translation Contributions

We welcome translations! Currently supported languages:
- English (en)
- Japanese (ja)
- Persian (fa)
- Arabic (ar)
- Chinese Simplified (zh-CN)
- Chinese Traditional (zh-TW)
- Russian (ru)

To add a new translation:
1. Create `docs/README_XX.md` (where XX is language code)
2. Translate relevant documentation
3. Add language to README.md language selector
4. Submit pull request

## Questions?

- 💬 [GitHub Discussions](https://github.com/yourusername/lala/discussions)
- 🐛 [GitHub Issues](https://github.com/yourusername/lala/issues)

## Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- CHANGELOG.md

Thank you for contributing! 🎉
