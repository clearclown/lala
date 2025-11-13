//! Cursor position management

use std::cmp;

/// Represents a cursor position in the text buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    /// Character index in the buffer
    position: usize,
}

impl Cursor {
    /// Create a new cursor at position 0
    pub fn new() -> Self {
        Self { position: 0 }
    }

    /// Create a cursor at a specific position
    pub fn at(position: usize) -> Self {
        Self { position }
    }

    /// Get the current cursor position
    pub fn position(&self) -> usize {
        self.position
    }

    /// Set the cursor position
    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    /// Move cursor forward by n characters
    pub fn move_forward(&mut self, n: usize, max_position: usize) {
        self.position = cmp::min(self.position.saturating_add(n), max_position);
    }

    /// Move cursor backward by n characters
    pub fn move_backward(&mut self, n: usize) {
        self.position = self.position.saturating_sub(n);
    }

    /// Adjust cursor position after an insertion before the cursor
    pub fn adjust_for_insert(&mut self, insert_pos: usize, insert_len: usize) {
        if insert_pos <= self.position {
            self.position = self.position.saturating_add(insert_len);
        }
    }

    /// Adjust cursor position after a deletion before the cursor
    pub fn adjust_for_delete(&mut self, delete_start: usize, delete_len: usize) {
        let delete_end = delete_start + delete_len;
        if delete_end <= self.position {
            self.position = self.position.saturating_sub(delete_len);
        } else if delete_start < self.position {
            self.position = delete_start;
        }
    }

    /// Clamp cursor position to valid range
    pub fn clamp(&mut self, max_position: usize) {
        self.position = cmp::min(self.position, max_position);
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cursor() {
        let cursor = Cursor::new();
        assert_eq!(cursor.position(), 0);
    }

    #[test]
    fn test_move_forward() {
        let mut cursor = Cursor::new();
        cursor.move_forward(5, 100);
        assert_eq!(cursor.position(), 5);
    }

    #[test]
    fn test_move_backward() {
        let mut cursor = Cursor::at(10);
        cursor.move_backward(3);
        assert_eq!(cursor.position(), 7);
    }

    #[test]
    fn test_adjust_for_insert() {
        let mut cursor = Cursor::at(10);
        cursor.adjust_for_insert(5, 3);
        assert_eq!(cursor.position(), 13);
    }

    #[test]
    fn test_adjust_for_delete() {
        let mut cursor = Cursor::at(10);
        cursor.adjust_for_delete(5, 3);
        assert_eq!(cursor.position(), 7);
    }
}
