# Lala - Simple Text Editor

A simple, efficient text editor built with Rust, featuring a modular architecture with a Rope-based text buffer and an egui-based GUI.

## Features

### Implemented (feature/basic-editing)

- ✅ **Text Display**: Efficiently displays text from Rope buffer in GUI
- ✅ **Text Input**: Full keyboard input support (characters, backspace, enter, delete)
- ✅ **Cursor Synchronization**: Bidirectional sync between GUI and core engine
- ✅ **Undo/Redo**: Full undo/redo support with keyboard shortcuts
  - Ctrl+Z: Undo
  - Ctrl+Y or Ctrl+Shift+Z: Redo
- ✅ **Save Functionality**: Async file saving with keyboard shortcut
  - Ctrl+S: Save file
- ✅ **Unsaved Changes Indicator**: Shows `*` in status bar when file is modified
- ✅ **Status Bar**: Displays file name, cursor position, and character count

## Architecture

### Core Components

#### core-engine (Crate)

The core editing engine, independent of any GUI framework:

- **TextBuffer** (`buffer.rs`): Rope-based text buffer for efficient editing
  - Supports efficient insert/delete operations
  - Character-indexed operations
  - Line-based operations

- **Cursor** (`cursor.rs`): Cursor position management
  - Forward/backward movement
  - Auto-adjustment after edits

- **History** (`history.rs`): Undo/Redo system
  - Edit history tracking
  - Configurable history size (default: 1000 operations)

- **Editor** (`editor.rs`): Main editor state
  - Integrates buffer, cursor, and history
  - Async file I/O
  - Change tracking

#### gui (Crate)

The GUI application built with egui:

- **LalaApp** (`app.rs`): Main application state
  - Menu bar (File, Edit)
  - Status bar
  - Keyboard shortcut handling
  - Async runtime for file operations

- **EditorView** (`editor_view.rs`): Editor view component
  - Text synchronization between egui and core-engine
  - Implements "方針A" (Approach A): Rope to String conversion
  - Change detection and propagation to core-engine

## Text Synchronization Strategy

The editor uses **Approach A** (方針A) for text synchronization:

1. Convert Rope to String for display in egui's TextEdit
2. Detect changes after user input
3. Calculate diff between old and new text
4. Send changes to core-engine via insert_char/delete_range APIs
5. Synchronize cursor position bidirectionally

This approach is simple and reliable, though it may have performance limitations with very large files. If needed, the implementation can be migrated to **Approach B** (方針B), which uses egui's LayoutJob/Galley APIs for direct Rope rendering.

## Building and Running

### Prerequisites

- Rust 1.70 or later
- Cargo

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Run

```bash
# Run in debug mode
cargo run

# Run in release mode
cargo run --release
```

### Test

```bash
# Run all tests
cargo test

# Run tests for core-engine only
cargo test --package core-engine
```

### Lint

```bash
# Run clippy
cargo clippy --all-targets --all-features -- -D warnings
```

## Project Structure

```
lala/
├── Cargo.toml          # Workspace configuration
├── README.md           # This file
├── core-engine/        # Core editing engine
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs      # Module exports
│       ├── buffer.rs   # Rope-based text buffer
│       ├── cursor.rs   # Cursor management
│       ├── history.rs  # Undo/Redo system
│       └── editor.rs   # Main editor state
└── gui/                # GUI application
    ├── Cargo.toml
    └── src/
        ├── main.rs         # Entry point
        ├── app.rs          # Application state
        └── editor_view.rs  # Editor view component
```

## Usage

### Keyboard Shortcuts

- **Ctrl+Z**: Undo last edit
- **Ctrl+Y** or **Ctrl+Shift+Z**: Redo
- **Ctrl+S**: Save file
- **Backspace**: Delete character before cursor
- **Delete**: Delete character at cursor
- **Enter**: Insert newline

### Menu

- **File Menu**:
  - New (not yet implemented)
  - Open... (not yet implemented)
  - Save
  - Quit

- **Edit Menu**:
  - Undo
  - Redo

## Completion Criteria

All completion criteria for feature/basic-editing have been met:

- [x] `cargo test` passes all tests
- [x] `cargo clippy` has no lint errors
- [x] Can open and edit text
- [x] Can input and delete text
- [x] Ctrl+Z (Undo) and Ctrl+S (Save) work correctly
- [x] `*` indicator appears after changes and disappears after save

## Performance Considerations

The current implementation uses String conversion for simplicity. This approach works well for files up to several thousand lines. For larger files, consider:

1. Profiling with `cargo bench` (if benchmarks are added)
2. Migrating to Approach B (direct Rope rendering) if conversion overhead becomes significant
3. Implementing viewport-based rendering for very large files

## Future Enhancements

Potential features for future development:

- **File Operations**: New file, Open dialog, Save As
- **Search**: Find and replace
- **Syntax Highlighting**: Language-aware highlighting (feature/syntax-highlighting)
- **Line Numbers**: Display line numbers in gutter
- **Multiple Tabs**: Edit multiple files simultaneously
- **Configuration**: User preferences and settings
- **Themes**: Dark mode and custom color schemes

## Dependencies

### Core Dependencies

- `ropey`: Rope data structure for efficient text editing
- `tokio`: Async runtime for file I/O
- `anyhow`: Error handling
- `thiserror`: Error type derivation
- `serde`: Serialization (future use)

### GUI Dependencies

- `eframe`: egui framework wrapper
- `egui`: Immediate mode GUI library
- `env_logger`: Logging

## License

(Add your license information here)

## Contributors

(Add contributor information here)
