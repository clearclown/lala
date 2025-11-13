//! Undo/Redo history management

use crate::buffer::TextBuffer;
use crate::cursor::Cursor;
use std::ops::Range;

/// Represents a single edit operation that can be undone/redone
#[derive(Debug, Clone)]
pub enum Edit {
    /// Insert text at a position
    Insert {
        position: usize,
        text: String,
        cursor_before: Cursor,
        cursor_after: Cursor,
    },
    /// Delete text from a range
    Delete {
        range: Range<usize>,
        text: String,
        cursor_before: Cursor,
        cursor_after: Cursor,
    },
}

impl Edit {
    /// Apply this edit to a buffer
    pub fn apply(&self, buffer: &mut TextBuffer, cursor: &mut Cursor) {
        match self {
            Edit::Insert { position, text, cursor_after, .. } => {
                buffer.insert(*position, text);
                *cursor = *cursor_after;
            }
            Edit::Delete { range, cursor_after, .. } => {
                buffer.delete_range(range.clone());
                *cursor = *cursor_after;
            }
        }
    }

    /// Undo this edit on a buffer
    pub fn undo(&self, buffer: &mut TextBuffer, cursor: &mut Cursor) {
        match self {
            Edit::Insert { position, text, cursor_before, .. } => {
                buffer.delete_range(*position..*position + text.len());
                *cursor = *cursor_before;
            }
            Edit::Delete { range, text, cursor_before, .. } => {
                buffer.insert(range.start, text);
                *cursor = *cursor_before;
            }
        }
    }
}

/// History manager for undo/redo operations
#[derive(Debug, Clone)]
pub struct History {
    /// Stack of edit operations
    edits: Vec<Edit>,
    /// Current position in the history
    current: usize,
    /// Maximum history size
    max_size: usize,
    /// Position in history when last saved (None if never saved)
    saved_position: Option<usize>,
}

impl History {
    /// Create a new history with default max size
    pub fn new() -> Self {
        Self::with_capacity(1000)
    }

    /// Create a new history with specified max size
    pub fn with_capacity(max_size: usize) -> Self {
        Self {
            edits: Vec::new(),
            current: 0,
            max_size,
            saved_position: Some(0), // Start at saved state
        }
    }

    /// Add an edit to the history
    pub fn add_edit(&mut self, edit: Edit) {
        // Remove any edits after current position (they were undone)
        self.edits.truncate(self.current);

        // Invalidate saved position if we're branching from it
        if let Some(saved_pos) = self.saved_position {
            if saved_pos > self.current {
                self.saved_position = None;
            }
        }

        // Add new edit
        self.edits.push(edit);
        self.current += 1;

        // Enforce max size
        if self.edits.len() > self.max_size {
            self.edits.remove(0);
            self.current -= 1;
            // Adjust saved position
            if let Some(saved_pos) = self.saved_position {
                if saved_pos > 0 {
                    self.saved_position = Some(saved_pos - 1);
                } else {
                    self.saved_position = None;
                }
            }
        }
    }

    /// Undo the last edit
    pub fn undo(&mut self, buffer: &mut TextBuffer, cursor: &mut Cursor) -> bool {
        if self.can_undo() {
            self.current -= 1;
            self.edits[self.current].undo(buffer, cursor);
            true
        } else {
            false
        }
    }

    /// Redo the next edit
    pub fn redo(&mut self, buffer: &mut TextBuffer, cursor: &mut Cursor) -> bool {
        if self.can_redo() {
            self.edits[self.current].apply(buffer, cursor);
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        self.current > 0
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        self.current < self.edits.len()
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.edits.clear();
        self.current = 0;
        self.saved_position = Some(0);
    }

    /// Check if the buffer has been modified since last save
    pub fn is_modified(&self) -> bool {
        match self.saved_position {
            Some(saved_pos) => self.current != saved_pos,
            None => true, // Never saved or saved position invalidated
        }
    }

    /// Mark the current state as saved
    pub fn mark_saved(&mut self) {
        self.saved_position = Some(self.current);
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_undo() {
        let mut buffer = TextBuffer::from_str("hello");
        let mut cursor = Cursor::at(5);
        let mut history = History::new();

        let edit = Edit::Insert {
            position: 5,
            text: " world".to_string(),
            cursor_before: Cursor::at(5),
            cursor_after: Cursor::at(11),
        };

        edit.apply(&mut buffer, &mut cursor);
        assert_eq!(buffer.to_string(), "hello world");
        assert_eq!(cursor.position(), 11);

        history.add_edit(edit.clone());
        history.undo(&mut buffer, &mut cursor);
        assert_eq!(buffer.to_string(), "hello");
        assert_eq!(cursor.position(), 5);
    }

    #[test]
    fn test_delete_undo() {
        let mut buffer = TextBuffer::from_str("hello world");
        let mut cursor = Cursor::at(5);
        let mut history = History::new();

        let edit = Edit::Delete {
            range: 5..11,
            text: " world".to_string(),
            cursor_before: Cursor::at(11),
            cursor_after: Cursor::at(5),
        };

        edit.apply(&mut buffer, &mut cursor);
        assert_eq!(buffer.to_string(), "hello");

        history.add_edit(edit.clone());
        history.undo(&mut buffer, &mut cursor);
        assert_eq!(buffer.to_string(), "hello world");
    }

    #[test]
    fn test_undo_redo() {
        let mut buffer = TextBuffer::from_str("test");
        let mut cursor = Cursor::at(4);
        let mut history = History::new();

        let edit = Edit::Insert {
            position: 4,
            text: "ing".to_string(),
            cursor_before: Cursor::at(4),
            cursor_after: Cursor::at(7),
        };

        // Apply the edit first
        edit.apply(&mut buffer, &mut cursor);
        assert_eq!(buffer.to_string(), "testing");

        // Add to history
        history.add_edit(edit);

        // Undo
        history.undo(&mut buffer, &mut cursor);
        assert_eq!(buffer.to_string(), "test");

        // Redo
        history.redo(&mut buffer, &mut cursor);
        assert_eq!(buffer.to_string(), "testing");
    }

    #[test]
    fn test_is_modified() {
        let mut buffer = TextBuffer::from_str("hello");
        let mut cursor = Cursor::at(5);
        let mut history = History::new();

        // Initially not modified
        assert!(!history.is_modified());

        // Add an edit - should be modified
        let edit = Edit::Insert {
            position: 5,
            text: " world".to_string(),
            cursor_before: Cursor::at(5),
            cursor_after: Cursor::at(11),
        };
        edit.apply(&mut buffer, &mut cursor);
        history.add_edit(edit);
        assert!(history.is_modified());

        // Mark as saved - should not be modified
        history.mark_saved();
        assert!(!history.is_modified());

        // Add another edit - should be modified again
        let edit2 = Edit::Insert {
            position: 11,
            text: "!".to_string(),
            cursor_before: Cursor::at(11),
            cursor_after: Cursor::at(12),
        };
        edit2.apply(&mut buffer, &mut cursor);
        history.add_edit(edit2);
        assert!(history.is_modified());

        // Undo - should not be modified (back to saved state)
        history.undo(&mut buffer, &mut cursor);
        assert!(!history.is_modified());

        // Undo again - should be modified (before saved state)
        history.undo(&mut buffer, &mut cursor);
        assert!(history.is_modified());

        // Redo - should not be modified (back to saved state)
        history.redo(&mut buffer, &mut cursor);
        assert!(!history.is_modified());
    }
}
