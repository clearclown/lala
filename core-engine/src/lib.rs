//! Core text editing engine with Rope-based buffer, undo/redo, and file I/O
//!
//! This crate provides the fundamental text editing operations that are
//! independent of any GUI framework. It manages:
//! - Text buffer using Rope data structure for efficient editing
//! - Cursor position tracking
//! - Undo/Redo history
//! - File loading and saving

pub mod buffer;
pub mod cursor;
pub mod editor;
pub mod history;

pub use buffer::TextBuffer;
pub use cursor::Cursor;
pub use editor::{Editor, EditorError};
pub use history::History;
