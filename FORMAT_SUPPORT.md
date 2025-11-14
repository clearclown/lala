# Format Support Design

## Overview

Lala editor will support multiple document and markup formats beyond Markdown:
- **LaTeX** - Mathematical notation and academic documents
- **Mermaid** - Diagrams (flowcharts, sequence diagrams, etc.)
- **HTML** - Web content preview

## Architecture

### Module Structure

```
src/
├── cli/
│   ├── markdown_view.rs    # Existing Markdown renderer
│   ├── latex_view.rs        # NEW: LaTeX renderer
│   ├── mermaid_view.rs      # NEW: Mermaid diagram renderer
│   ├── html_view.rs         # NEW: HTML renderer
│   └── format_detector.rs   # NEW: File type detection
├── gui/
│   ├── markdown_preview.rs  # Existing
│   ├── latex_preview.rs     # NEW: LaTeX GUI preview
│   ├── mermaid_preview.rs   # NEW: Mermaid GUI preview
│   └── html_preview.rs      # NEW: HTML GUI preview
```

### Format Detection

Detect format based on:
1. File extension (`.tex`, `.latex`, `.mmd`, `.html`, `.htm`)
2. Content analysis (shebang, document headers)
3. User-specified format flag

## Implementation Plan

### Phase 1: HTML Support (Easiest)

**Goal**: Preview HTML files as formatted text in CLI and GUI

**CLI Implementation**:
- Use `html2text` crate to convert HTML to readable text
- Preserve structure (headings, lists, tables)
- Syntax highlight code blocks in HTML

**GUI Implementation**:
- Use `egui` widgets to render HTML elements
- Support basic tags: `<h1>-<h6>`, `<p>`, `<ul>`, `<ol>`, `<code>`, `<pre>`, etc.
- No full browser rendering (keep it lightweight)

**Command**:
```bash
lala html <file>              # Auto-detect and preview
lala view --format html <file>  # Explicit format
```

**Dependencies**:
```toml
html2text = "0.12"
scraper = "0.19"
```

### Phase 2: Mermaid Support

**Goal**: Render Mermaid diagrams as ASCII art or via external tool

**Approach 1: ASCII Art** (Pure Rust):
- Parse Mermaid syntax manually
- Generate ASCII box diagrams
- Limited to simple flowcharts initially

**Approach 2: External Tool** (Better quality):
- Use `mermaid-cli` (`mmdc`) if available
- Generate SVG/PNG and display path
- CLI: Show text representation or path to generated image
- GUI: Render generated SVG/PNG

**Command**:
```bash
lala mermaid <file>           # Preview diagram
lala mermaid <file> --output diagram.svg  # Generate SVG
```

**Dependencies**:
```toml
# For Approach 1 (pure Rust, limited):
# Custom parser

# For Approach 2 (external tool):
# System requirement: npm install -g @mermaid-js/mermaid-cli
```

### Phase 3: LaTeX Support

**Goal**: Preview LaTeX documents and render math equations

**Approach 1: Math-only** (Easier):
- Use `katex` or similar for math rendering
- Focus on inline math `$...$` and display math `$$...$$`
- CLI: Unicode approximation of math symbols
- GUI: Use `resvg` to render equations as SVG

**Approach 2: Full Document** (Complex):
- Use `tectonic` to compile LaTeX to PDF
- Display PDF preview or conversion result
- Requires full LaTeX distribution

**Command**:
```bash
lala latex <file>             # Preview LaTeX
lala latex <file> --math-only   # Only render math
lala latex <file> --compile     # Compile to PDF
```

**Dependencies**:
```toml
# Math rendering:
# pulldown-cmark already supports some math
# May need custom renderer

# Full compilation:
tectonic = "0.14"  # Optional, for full LaTeX compilation
```

## CLI Command Structure

### Extended Subcommands

```
lala [OPTIONS] [PATH] [COMMAND]

Commands:
  markdown  Preview Markdown file in terminal
  html      Preview HTML file in terminal
  mermaid   Preview Mermaid diagram
  latex     Preview LaTeX document
  view      View file content (auto-detect format)
  help      Print help
```

### Auto-Detection in `view` Command

```bash
# Auto-detect format and preview appropriately
lala view document.md     # → Markdown preview
lala view page.html       # → HTML preview
lala view diagram.mmd     # → Mermaid preview
lala view paper.tex       # → LaTeX preview

# Force specific format
lala view --format markdown document.txt
lala view --format html snippet.txt
```

## Format Capabilities Matrix

| Format   | CLI Preview | GUI Preview | Export | Syntax Highlight |
|----------|-------------|-------------|--------|------------------|
| Markdown | ✅ Done     | ✅ Done     | -      | ✅ Done          |
| HTML     | 🔄 Planned  | 🔄 Planned  | -      | 🔄 Planned       |
| Mermaid  | 🔄 Planned  | 🔄 Planned  | → SVG  | 🔄 Planned       |
| LaTeX    | 🔄 Planned  | 🔄 Planned  | → PDF  | 🔄 Planned       |

## Technical Details

### HTML Rendering (CLI)

```rust
use html2text::from_read;

pub fn render_html_to_terminal(html: &str) {
    let text = from_read(html.as_bytes(), 80); // 80 char width
    println!("{}", text);
}
```

**Enhanced Version**:
- Parse HTML with `scraper`
- Apply colors and styles like Markdown renderer
- Handle tables, lists, headings with proper formatting

### Mermaid Rendering (ASCII)

Example flow:
```
Input:
graph TD
    A[Start] --> B[Process]
    B --> C[End]

Output:
┌─────────┐
│  Start  │
└────┬────┘
     │
     ▼
┌─────────┐
│ Process │
└────┬────┘
     │
     ▼
┌─────────┐
│   End   │
└─────────┘
```

### LaTeX Math Rendering (CLI)

Unicode approximations:
- `\sqrt{x}` → `√x`
- `\frac{a}{b}` → `a/b` or multi-line fraction
- `\sum` → `Σ`
- `\int` → `∫`
- Greek letters: `\alpha` → `α`, `\beta` → `β`

## Dependencies Summary

```toml
[dependencies]
# Existing
eframe = "0.29"
egui = "0.29"
ropey = "1.6"
tokio = { version = "1.35", features = ["full"] }
ignore = "0.4"
flume = "0.11"
syntect = "5.2"
pulldown-cmark = "0.12"
regex = "1.10"
anyhow = "1.0"
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
clap = { version = "4.4", features = ["derive"] }
colored = "2.1"
terminal_size = "0.3"
env_logger = "0.11"

# NEW: Format support
html2text = "0.12"           # HTML to text conversion
scraper = "0.19"             # HTML parsing
# tectonic = "0.14"          # Optional: LaTeX compilation
# resvg = "0.42"             # Optional: SVG rendering
```

## Implementation Order

1. ✅ **Markdown** - Already complete
2. 🔄 **HTML** - Start here (simplest)
   - CLI: html2text conversion with colors
   - GUI: Basic HTML widget rendering
3. 🔄 **Mermaid** - Medium difficulty
   - CLI: ASCII art diagrams
   - GUI: External tool + image display
4. 🔄 **LaTeX** - Most complex
   - CLI: Math symbol Unicode rendering
   - GUI: Math equation rendering

## Testing Strategy

### Unit Tests
- Format detection accuracy
- HTML parsing and rendering
- Mermaid syntax parsing
- LaTeX math symbol conversion

### Integration Tests
- End-to-end preview for each format
- CLI command execution
- GUI rendering (where applicable)

### Test Files
```
tests/
├── fixtures/
│   ├── sample.html
│   ├── diagram.mmd
│   ├── equations.tex
│   └── document.tex
└── format_tests.rs
```

## User Documentation

Update documentation:
- `README.md` - Add format support section
- `CLI_USAGE.md` - Add examples for new commands
- `FORMAT_EXAMPLES.md` - NEW: Example files for each format

## Future Enhancements

- **PDF Preview**: Direct PDF rendering
- **YAML/TOML**: Configuration file preview with syntax highlighting
- **CSV/TSV**: Table rendering in terminal
- **PlantUML**: UML diagram support (similar to Mermaid)
- **AsciiDoc**: Alternative to Markdown
- **Org-mode**: Emacs org-mode files
- **Jupyter Notebooks**: `.ipynb` preview

## Notes

- Keep all renderers lightweight and fast
- Prefer pure Rust solutions when possible
- Fall back to external tools when necessary (with clear error messages)
- Maintain consistent command structure across formats
- Always provide graceful degradation (e.g., show source if rendering fails)
