# Lala Editor - CLI Usage Guide

## Overview

Lala now supports comprehensive command-line operations! You can preview **Markdown, HTML, Mermaid diagrams, and LaTeX documents** with beautiful terminal formatting, view files, and more - all without launching the GUI.

---

## Available Commands

### 1. GUI Mode (Default)

```bash
# Launch GUI editor
lala                    # Empty editor
lala file.txt           # Open file in GUI
lala /path/to/dir       # Open directory in GUI
```

### 2. Markdown Preview (NEW!)

Preview Markdown files in your terminal with beautiful formatting:

```bash
lala markdown <FILE> [OPTIONS]
```

**Features:**
- ✨ **Colored output** - Syntax-highlighted display
- 📐 **Proper formatting** - Headers with size variation
- 📝 **Styled text** - Bold, italic, inline code
- 📋 **Lists** - Both ordered and unordered
- 💻 **Code blocks** - Syntax-highlighted code
- 🔗 **Links** - Displayed with URLs
- 📖 **Blockquotes** - Styled quotes
- ➖ **Horizontal rules** - Visual separators

**Options:**
- `--no-color` - Disable colored output

**Examples:**
```bash
# Preview README with colors
lala markdown README.md

# Preview without colors (plain text)
lala markdown README.md --no-color

# Pipe to less for scrolling
lala markdown README.md | less -R

# Render specific doc
lala markdown docs/guide.md
```

**Visual Output:**
```
Heading 1
=========

Heading 2
---------

### Heading 3

This is **bold** and this is *italic*.

  • List item 1
  • List item 2

```rust
fn main() {
    println!("Hello!");
}
\```

│ This is a blockquote
│ with multiple lines

`inline code` and [Link](https://example.com)

────────────────────────────────────────
```

### 3. HTML Preview (NEW!)

Preview HTML files in your terminal with beautiful formatting:

```bash
lala html <FILE> [OPTIONS]
```

**Features:**
- ✨ **Colored output** - Styled headings, links, code
- 📐 **Proper structure** - Tables, lists, paragraphs
- 📝 **Text styling** - Bold, italic, inline code
- 📋 **Lists** - Both ordered and unordered
- 💻 **Code blocks** - Preserved formatting
- 🔗 **Links** - Displayed with URLs
- 📊 **Tables** - Well-formatted tables

**Options:**
- `--no-color` - Disable colored output

**Examples:**
```bash
# Preview HTML with colors
lala html page.html

# Preview without colors (plain text)
lala html page.html --no-color

# Pipe to less for scrolling
lala html page.html | less -R
```

### 4. Mermaid Diagram Preview (NEW!)

Preview Mermaid diagrams as ASCII art:

```bash
lala mermaid <FILE> [OPTIONS]
```

**Supported Diagram Types:**
- 📊 **Flowcharts** - Graph TD/LR with boxes and arrows
- 🔄 **Sequence Diagrams** - Message flows between actors
- 📦 **Class Diagrams** - OOP class structures
- 🔀 **State Diagrams** - State transitions
- 🗃️ **ER Diagrams** - Entity relationships
- 📅 **Gantt Charts** - Project timelines
- 🥧 **Pie Charts** - Data visualization

**Options:**
- `--no-color` - Disable colored output

**Examples:**
```bash
# Preview flowchart
lala mermaid flowchart.mmd

# Preview sequence diagram
lala mermaid sequence.mmd

# View without colors
lala mermaid diagram.mmd --no-color
```

**Note:** For high-quality SVG/PNG output, install mermaid-cli:
```bash
npm install -g @mermaid-js/mermaid-cli
mmdc -i diagram.mmd -o diagram.svg
```

### 5. LaTeX Document Preview (NEW!)

Preview LaTeX documents with Unicode math symbols:

```bash
lala latex <FILE> [OPTIONS]
```

**Features:**
- 🔤 **Unicode Math Symbols** - √, ∫, ∑, ∂, ∇, ∞
- 🇬🇷 **Greek Letters** - α, β, γ, δ, θ, λ, μ, π, σ, ω
- ➕ **Operators** - ±, ×, ÷, ≠, ≤, ≥, ≈, ≡
- 📐 **Math Rendering** - Fractions, square roots, equations
- 📑 **Document Structure** - Sections, subsections, lists
- 📝 **Text Formatting** - Bold, italic, emphasis

**Options:**
- `--no-color` - Disable colored output

**Examples:**
```bash
# Preview LaTeX document
lala latex paper.tex

# Preview without colors
lala latex document.tex --no-color

# Check math rendering
lala latex equations.tex
```

**Note:** For full PDF compilation:
```bash
pdflatex document.tex
# or use Overleaf: https://www.overleaf.com
```

### 6. View File

Display file content with optional line numbers:

```bash
lala view <FILE> [OPTIONS]
```

**Options:**
- `-n, --line-numbers` - Show line numbers

**Examples:**
```bash
# View file content
lala view README.md

# View with line numbers
lala view -n src/main.rs

# View specific file
lala view Cargo.toml
```

**Sample Output:**
```
     1 [package]
     2 name = "lala"
     3 version = "0.1.0"
     4 edition = "2021"
```

---

## Command Reference

### Help

```bash
# Show general help
lala --help

# Show Markdown command help
lala markdown --help

# Show view command help
lala view --help
```

### Version

```bash
lala --version
```

---

## Use Cases

### 1. Quick Markdown Preview

```bash
# Check your README before committing
lala markdown README.md

# Preview documentation
lala markdown docs/api.md
```

### 2. Code Review

```bash
# View source file with line numbers
lala view -n src/main.rs

# Compare files
diff <(lala view old.txt) <(lala view new.txt)
```

### 3. Pipeline Integration

```bash
# Generate and preview markdown
echo "# Title\n\nContent here" | tee doc.md && lala markdown doc.md

# View multiple files
for file in docs/*.md; do
    echo "=== $file ==="
    lala markdown "$file"
done
```

### 4. Documentation Workflow

```bash
# Edit -> Preview cycle
vim README.md && lala markdown README.md

# Watch and preview (with entr)
echo README.md | entr lala markdown README.md
```

---

## Markdown Formatting Details

### Headers

```markdown
# H1 - Large, blue, underlined with ====
## H2 - Large, cyan, underlined with ----
### H3 - Green with ###
#### H4 - Yellow with ####
##### H5 - Yellow with #####
###### H6 - Dimmed with ######
```

### Text Styling

```markdown
**Bold text** - Displayed in bold
*Italic text* - Displayed in italic
***Bold and italic*** - Both styles applied
`inline code` - Magenta with gray background
```

### Lists

```markdown
Unordered (•):
• Item 1
• Item 2

Ordered:
1. First
2. Second
```

### Code Blocks

````markdown
```rust
fn main() {
    println!("Syntax-highlighted!");
}
```
````

Displayed with:
- Language identifier (```rust)
- Dark background
- Monospace font
- Syntax hints

### Blockquotes

```markdown
> This is a quote
> with multiple lines
```

Displayed with:
- Leading │ bar
- Italic styling
- Dimmed appearance

### Links

```markdown
[Text](https://example.com)
```

Displayed as:
- Underlined blue text
- URL in parentheses (dimmed)

### Horizontal Rules

```markdown
---
```

Displayed as 80-character line

---

## Configuration

### Terminal Requirements

For best experience:
- **True color support** - For syntax highlighting
- **Unicode support** - For special characters (•, │, etc.)
- **ANSI colors** - For colored output

### Environment

```bash
# Force color output (if auto-detection fails)
FORCE_COLOR=1 lala markdown README.md

# Disable colors
NO_COLOR=1 lala markdown README.md
# or use --no-color flag
```

---

## Troubleshooting

### Colors Not Showing

```bash
# Try:
lala markdown README.md --no-color  # Check if content is correct
echo $TERM  # Should be xterm-256color or similar
```

### File Not Found

```bash
# Use absolute path
lala markdown /full/path/to/file.md

# Or relative from current directory
cd /path/to/project
lala markdown README.md
```

### Broken Pipe Error

This is normal when piping to `head` or similar commands:
```bash
lala markdown README.md | head -20  # May show "Broken pipe"
# This is expected behavior, not an error
```

---

## Future Commands (Planned)

```bash
# Search in files
lala search "pattern" [path]

# Replace text
lala replace "old" "new" [path]

# Show file statistics
lala stats [path]

# File tree display
lala tree [path]

# Format code
lala format <file>

# Interactive TUI editor
lala edit --interactive <file>
```

See `CLI_DESIGN.md` for complete roadmap.

---

## Examples Gallery

### Example 1: Preview README

```bash
$ lala markdown README.md

Lala Editor
===========

A modern text editor...

Features
--------

### ✨ File Tree View
  • Tree Display: Browse directories...
```

### Example 2: View Source with Line Numbers

```bash
$ lala view -n src/main.rs

     1 use eframe::egui;
     2 use lala::cli::{markdown_view, parse_args_default, StartupMode};
     3 use lala::LalaApp;
     4 use std::fs;
     5 use std::process;
```

### Example 3: Quick Doc Check

```bash
$ lala markdown CHANGELOG.md | less -R
# Navigate with arrow keys, q to quit
```

---

## Tips & Tricks

### 1. Alias for Quick Preview

```bash
# Add to ~/.bashrc or ~/.zshrc
alias mdp='lala markdown'

# Usage
mdp README.md
```

### 2. Integration with Editor

```bash
# Vim: Preview current file
:!lala markdown %

# VS Code: Add to tasks.json
{
    "label": "Preview Markdown",
    "type": "shell",
    "command": "lala markdown ${file}"
}
```

### 3. Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit
if git diff --cached --name-only | grep -q 'README.md'; then
    echo "=== README Preview ==="
    lala markdown README.md | head -50
fi
```

---

## Performance

- **Fast**: Renders instantly for files up to 10MB
- **Efficient**: Uses pulldown-cmark (pure Rust parser)
- **Lightweight**: No external dependencies for rendering

---

## Comparison with Other Tools

| Feature | lala | bat | glow | pandoc |
|---------|------|-----|------|--------|
| Markdown preview | ✅ | ❌ | ✅ | ✅ |
| Colored output | ✅ | ✅ | ✅ | ❌ |
| No dependencies | ✅ | ❌ | ❌ | ❌ |
| GUI + CLI | ✅ | ❌ | ❌ | ❌ |
| Code editor | ✅ | ❌ | ❌ | ❌ |

---

## Contributing

Want more CLI features? See:
- `CLI_DESIGN.md` - Comprehensive CLI design
- GitHub Issues - Feature requests
- Pull Requests - Contributions welcome!

---

## Support

- 📖 Documentation: See `README.md`
- 🐛 Issues: GitHub Issues
- 💬 Discussions: GitHub Discussions

---

**Enjoy using Lala CLI!** 🎉

For GUI features, see the main `README.md`.
