# Lala Editor

A modern text editor with a graphical user interface built with Rust, egui, and eframe.

## Features

### ✨ File Tree View
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

### 📝 Basic Text Editing
- **Text Display**: Efficiently displays text from Rope buffer in GUI
- **Text Input**: Full keyboard input support (characters, backspace, enter, delete)
- **Cursor Synchronization**: Bidirectional sync between GUI and core engine
- **Undo/Redo**: Full undo/redo support with keyboard shortcuts
  - Ctrl+Z: Undo
  - Ctrl+Y or Ctrl+Shift+Z: Redo
- **Save Functionality**: Async file saving with keyboard shortcut
  - Ctrl+S: Save file
- **Unsaved Changes Indicator**: Shows `*` in status bar when file is modified
- **Status Bar**: Displays file name, cursor position, and character count

### 🔍 Advanced Search and Replace
- **Grep Integration**: Fast file-wide search using ripgrep
- **Multi-file Search**: Search across multiple files in directories
- **Replace Functionality**: Find and replace text in files
- **Regex Support**: Full regular expression support for advanced patterns

### 🎨 Editor Features
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

### Core Components

#### Text Buffer
The core editing engine uses a Rope-based text buffer for efficient editing:
- Supports efficient insert/delete operations
- Character-indexed operations
- Line-based operations

#### Text Synchronization Strategy
The editor uses an efficient synchronization approach:
1. Convert Rope to String for display in egui's TextEdit
2. Detect changes after user input
3. Calculate diff between old and new text
4. Send changes to core engine via insert_char/delete_range APIs
5. Synchronize cursor position bidirectionally

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

## Dependencies

Core dependencies:
- `eframe` / `egui`: GUI framework
- `ropey`: Rope data structure for efficient text editing
- `tokio`: Async runtime
- `ignore`: Git-aware file walking (respects .gitignore)
- `flume`: Fast multi-producer multi-consumer channels
- `anyhow`: Error handling
- `thiserror`: Error type derivation
- `regex`: Regular expressions
- `serde`: Serialization

## Keyboard Shortcuts

- **Ctrl+Z**: Undo last edit
- **Ctrl+Y** or **Ctrl+Shift+Z**: Redo
- **Ctrl+S**: Save file
- **Backspace**: Delete character before cursor
- **Delete**: Delete character at cursor
- **Enter**: Insert newline

## Future Enhancements

Potential features for future development:
- Right-click context menu (new file, delete, rename)
- File search in tree
- Git status indicators
- Drag and drop support
- Tree icons for different file types
- Keyboard navigation
- Themes and syntax highlighting
- Configuration system
- Multiple cursors
- Split view editing
