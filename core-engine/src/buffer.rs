//! Text buffer implementation using Rope data structure

use ropey::Rope;
use std::fmt;
use std::ops::Range;
use std::str::FromStr;

/// Text buffer backed by a Rope for efficient text editing operations
#[derive(Debug, Clone)]
pub struct TextBuffer {
    rope: Rope,
}

impl TextBuffer {
    /// Create a new empty text buffer
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
        }
    }

    /// Create a text buffer from a string
    ///
    /// Note: This type also implements the `FromStr` trait.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
        }
    }

    /// Get the total length of the text in bytes
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Get the total length of the text in characters
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get the number of lines
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Insert a character at the given character index
    pub fn insert_char(&mut self, idx: usize, ch: char) {
        if idx <= self.len_chars() {
            self.rope.insert_char(idx, ch);
        }
    }

    /// Insert a string at the given character index
    pub fn insert(&mut self, idx: usize, text: &str) {
        if idx <= self.len_chars() {
            self.rope.insert(idx, text);
        }
    }

    /// Delete a range of characters
    pub fn delete_range(&mut self, range: Range<usize>) {
        if range.start <= range.end && range.end <= self.len_chars() {
            self.rope.remove(range);
        }
    }

    /// Delete a single character at the given index
    pub fn delete_char(&mut self, idx: usize) {
        if idx < self.len_chars() {
            self.rope.remove(idx..idx + 1);
        }
    }

    /// Get the entire text as a String
    ///
    /// Note: This type also implements the `Display` trait.
    #[allow(clippy::inherent_to_string)]
    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Get a slice of text as a String
    pub fn slice(&self, range: Range<usize>) -> String {
        if range.start <= range.end && range.end <= self.len_chars() {
            self.rope.slice(range).to_string()
        } else {
            String::new()
        }
    }

    /// Get a line as a String (0-indexed)
    pub fn line(&self, idx: usize) -> String {
        if idx < self.len_lines() {
            self.rope.line(idx).to_string()
        } else {
            String::new()
        }
    }

    /// Get the character at the given index
    pub fn char_at(&self, idx: usize) -> Option<char> {
        if idx < self.len_chars() {
            self.rope.char(idx).into()
        } else {
            None
        }
    }

    /// Replace the entire buffer content
    pub fn replace_all(&mut self, text: &str) {
        self.rope = Rope::from_str(text);
    }

    /// Get a reference to the underlying Rope
    pub fn rope(&self) -> &Rope {
        &self.rope
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TextBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rope)
    }
}

impl FromStr for TextBuffer {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rope: Rope::from_str(s),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buffer = TextBuffer::new();
        assert_eq!(buffer.len_chars(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_from_str() {
        let buffer = TextBuffer::from_str("hello\nworld");
        assert_eq!(buffer.len_chars(), 11);
        assert_eq!(buffer.to_string(), "hello\nworld");
    }

    #[test]
    fn test_insert_char() {
        let mut buffer = TextBuffer::from_str("hllo");
        buffer.insert_char(1, 'e');
        assert_eq!(buffer.to_string(), "hello");
    }

    #[test]
    fn test_insert_text() {
        let mut buffer = TextBuffer::from_str("hllo");
        buffer.insert(1, "e");
        assert_eq!(buffer.to_string(), "hello");
    }

    #[test]
    fn test_delete_range() {
        let mut buffer = TextBuffer::from_str("hello world");
        buffer.delete_range(5..11);
        assert_eq!(buffer.to_string(), "hello");
    }

    #[test]
    fn test_delete_char() {
        let mut buffer = TextBuffer::from_str("hello");
        buffer.delete_char(1);
        assert_eq!(buffer.to_string(), "hllo");
    }
}
