# Lala Editor

A modern text editor with a graphical user interface built with Rust, egui, and eframe.

## Features

### ✨ File Tree View (feature/file-tree)

- **Tree Display**: Browse directories and files in a hierarchical tree view in the left sidebar
- **Directory Expansion**: Click the folder icon to expand/collapse directories
- **File Opening**: Click on files to open them in editor tabs
- **Async Loading**: Non-blocking directory scanning prevents UI freezes
- **Smart Filtering**:
  - Respects `.gitignore` files (when in a git repository)
  - Filters out `.git` directories automatically
  - Does not follow symbolic links (for security)
- **Error Handling**: Displays access denied messages for restricted directories
- **Performance**: Optimized for large directories (like `node_modules`)

### 📝 Editor Features

- **Multi-tab Editing**: Open multiple files simultaneously
- **Syntax Highlighting**: Code editor with monospace font
- **File Management**: Save files with modification indicators
- **Tab Management**: Close tabs with the × button

## Architecture

The project is organized as a Cargo workspace with two main crates:

### `core-cli`
The command-line interface that launches the application:
```bash
lala [PATH]  # Opens the editor with the specified file or directory
lala .       # Opens current directory
```

### `gui-base`
The GUI application built with egui/eframe:
- `src/lib.rs`: Main application entry point
- `src/gui/mod.rs`: GUI application state and rendering
- `src/gui/file_tree.rs`: File tree component with async loading

## Building and Running

### Prerequisites
- Rust 1.70 or later
- Cargo

### Build
```bash
cargo build --release
```

### Run
```bash
# Run in development mode
cargo run -- .

# Or run the binary directly
./target/release/lala .
```

### Test
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

### Lint
```bash
cargo clippy --all-targets --all-features
```

## Implementation Details

### Async Directory Loading

The file tree uses async I/O to prevent UI freezes:

1. **Background Thread**: Directory scanning happens in a separate thread using `tokio::spawn`
2. **Channel Communication**: Results are sent to the GUI thread via `flume` channels
3. **Incremental Updates**: The tree updates as directories are scanned
4. **Depth Limiting**: Initial load is limited to 3 levels deep for performance

### Security Considerations

- **Symlink Handling**: Symbolic links are not followed to prevent infinite loops and security issues
- **Permission Errors**: Access denied errors are caught and displayed in the tree

### Dependencies

Core dependencies:
- `eframe` / `egui`: GUI framework
- `tokio`: Async runtime
- `ignore`: Git-aware file walking (respects .gitignore)
- `flume`: Fast multi-producer multi-consumer channels
- `anyhow`: Error handling

## Completion Checklist

- [x] cargo test passes all tests
- [x] cargo clippy has no lint errors
- [x] File tree displays in left sidebar
- [x] Folder expansion/collapse works
- [x] Files open in editor tabs when clicked
- [x] Async loading prevents UI freezes
- [x] Error handling for permission issues
- [x] .git directories filtered out
- [x] Unit tests for tree logic

## Future Enhancements

Potential features for future development:
- Right-click context menu (new file, delete, rename)
- File search in tree
- Git status indicators
- Drag and drop support
- Tree icons for different file types
- Keyboard navigation
