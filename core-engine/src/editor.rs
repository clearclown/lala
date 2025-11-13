//! Main editor state combining buffer, cursor, history, and file operations

use crate::buffer::TextBuffer;
use crate::cursor::Cursor;
use crate::history::{Edit, History};
use std::ops::Range;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors that can occur during editor operations
#[derive(Error, Debug)]
pub enum EditorError {
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid cursor position: {0}")]
    InvalidPosition(usize),

    #[error("No file path set")]
    NoFilePath,
}

/// Main editor state
#[derive(Debug, Clone)]
pub struct Editor {
    /// Text buffer
    buffer: TextBuffer,
    /// Current cursor position
    cursor: Cursor,
    /// Undo/redo history
    history: History,
    /// File path (if any)
    file_path: Option<PathBuf>,
    /// Whether the buffer has unsaved changes
    modified: bool,
}

impl Editor {
    /// Create a new empty editor
    pub fn new() -> Self {
        Self {
            buffer: TextBuffer::new(),
            cursor: Cursor::new(),
            history: History::new(),
            file_path: None,
            modified: false,
        }
    }

    /// Create an editor with initial text
    pub fn with_text(text: &str) -> Self {
        Self {
            buffer: TextBuffer::from_str(text),
            cursor: Cursor::new(),
            history: History::new(),
            file_path: None,
            modified: false,
        }
    }

    /// Load a file into the editor
    pub async fn load_file<P: AsRef<Path>>(path: P) -> Result<Self, EditorError> {
        let path = path.as_ref();
        let content = tokio::fs::read_to_string(path).await?;

        Ok(Self {
            buffer: TextBuffer::from_str(&content),
            cursor: Cursor::new(),
            history: History::new(),
            file_path: Some(path.to_path_buf()),
            modified: false,
        })
    }

    /// Save the current buffer to its file path
    pub async fn save_file(&mut self) -> Result<(), EditorError> {
        if let Some(path) = self.file_path.clone() {
            self.save_to_file(path).await
        } else {
            Err(EditorError::NoFilePath)
        }
    }

    /// Save the buffer to a specific file path
    pub async fn save_to_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), EditorError> {
        let path = path.as_ref();
        tokio::fs::write(path, self.buffer.to_string()).await?;
        self.file_path = Some(path.to_path_buf());
        self.modified = false;
        Ok(())
    }

    /// Get the file path
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    /// Get the file name
    pub fn file_name(&self) -> Option<&str> {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
    }

    /// Check if the editor has unsaved changes
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Get a reference to the text buffer
    pub fn buffer(&self) -> &TextBuffer {
        &self.buffer
    }

    /// Get the current cursor position
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Set the cursor position
    pub fn set_cursor_position(&mut self, position: usize) {
        self.cursor.set_position(position);
        self.cursor.clamp(self.buffer.len_chars());
    }

    /// Get the entire text as a String
    pub fn text(&self) -> String {
        self.buffer.to_string()
    }

    /// Insert a character at the current cursor position
    pub fn insert_char(&mut self, ch: char) {
        let pos = self.cursor.position();
        let cursor_before = self.cursor;

        self.buffer.insert_char(pos, ch);
        self.cursor.move_forward(1, self.buffer.len_chars());

        let cursor_after = self.cursor;

        let edit = Edit::Insert {
            position: pos,
            text: ch.to_string(),
            cursor_before,
            cursor_after,
        };

        self.history.add_edit(edit);
        self.modified = true;
    }

    /// Insert text at the current cursor position
    pub fn insert_text(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        let pos = self.cursor.position();
        let cursor_before = self.cursor;

        self.buffer.insert(pos, text);
        self.cursor.move_forward(text.len(), self.buffer.len_chars());

        let cursor_after = self.cursor;

        let edit = Edit::Insert {
            position: pos,
            text: text.to_string(),
            cursor_before,
            cursor_after,
        };

        self.history.add_edit(edit);
        self.modified = true;
    }

    /// Delete a range of text
    pub fn delete_range(&mut self, range: Range<usize>) {
        if range.start >= range.end || range.end > self.buffer.len_chars() {
            return;
        }

        let cursor_before = self.cursor;
        let deleted_text = self.buffer.slice(range.clone());

        self.buffer.delete_range(range.clone());

        if self.cursor.position() > range.start {
            self.cursor.adjust_for_delete(range.start, range.end - range.start);
        }

        let cursor_after = self.cursor;

        let edit = Edit::Delete {
            range,
            text: deleted_text,
            cursor_before,
            cursor_after,
        };

        self.history.add_edit(edit);
        self.modified = true;
    }

    /// Delete the character before the cursor (backspace)
    pub fn delete_before_cursor(&mut self) {
        let pos = self.cursor.position();
        if pos > 0 {
            self.delete_range(pos - 1..pos);
        }
    }

    /// Delete the character at the cursor (delete key)
    pub fn delete_at_cursor(&mut self) {
        let pos = self.cursor.position();
        if pos < self.buffer.len_chars() {
            self.delete_range(pos..pos + 1);
        }
    }

    /// Undo the last edit
    pub fn undo(&mut self) -> bool {
        if self.history.undo(&mut self.buffer, &mut self.cursor) {
            self.modified = true;
            true
        } else {
            false
        }
    }

    /// Redo the next edit
    pub fn redo(&mut self) -> bool {
        if self.history.redo(&mut self.buffer, &mut self.cursor) {
            self.modified = true;
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// Move cursor forward
    pub fn move_cursor_forward(&mut self, n: usize) {
        self.cursor.move_forward(n, self.buffer.len_chars());
    }

    /// Move cursor backward
    pub fn move_cursor_backward(&mut self, n: usize) {
        self.cursor.move_backward(n);
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_editor() {
        let editor = Editor::new();
        assert_eq!(editor.text(), "");
        assert_eq!(editor.cursor().position(), 0);
        assert!(!editor.is_modified());
    }

    #[test]
    fn test_insert_char() {
        let mut editor = Editor::new();
        editor.insert_char('h');
        editor.insert_char('i');
        assert_eq!(editor.text(), "hi");
        assert_eq!(editor.cursor().position(), 2);
        assert!(editor.is_modified());
    }

    #[test]
    fn test_delete_before_cursor() {
        let mut editor = Editor::with_text("hello");
        editor.set_cursor_position(5);
        editor.delete_before_cursor();
        assert_eq!(editor.text(), "hell");
        assert_eq!(editor.cursor().position(), 4);
    }

    #[test]
    fn test_undo_redo() {
        let mut editor = Editor::new();
        editor.insert_char('a');
        editor.insert_char('b');
        assert_eq!(editor.text(), "ab");

        editor.undo();
        assert_eq!(editor.text(), "a");

        editor.undo();
        assert_eq!(editor.text(), "");

        editor.redo();
        assert_eq!(editor.text(), "a");

        editor.redo();
        assert_eq!(editor.text(), "ab");
    }

    #[tokio::test]
    async fn test_save_and_load() {
        use std::env;
        use std::fs;

        let temp_dir = env::temp_dir();
        let test_file = temp_dir.join("test_editor.txt");

        let mut editor = Editor::with_text("hello world");
        editor.save_to_file(&test_file).await.unwrap();

        let loaded = Editor::load_file(&test_file).await.unwrap();
        assert_eq!(loaded.text(), "hello world");

        fs::remove_file(test_file).ok();
    }
}
