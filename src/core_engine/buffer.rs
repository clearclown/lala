use ropey::Rope;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// A text buffer backed by a Rope for efficient editing
pub struct Buffer {
    #[allow(dead_code)]
    id: BufferId,
    rope: Rope,
    #[allow(dead_code)]
    file_path: Option<PathBuf>,
    dirty: bool,
}

impl Buffer {
    #[allow(dead_code)]
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            rope: Rope::new(),
            file_path: None,
            dirty: false,
        }
    }

    pub fn from_string(id: BufferId, content: String, file_path: Option<PathBuf>) -> Self {
        Self {
            id,
            rope: Rope::from_str(&content),
            file_path,
            dirty: false,
        }
    }

    #[allow(dead_code)]
    pub fn id(&self) -> BufferId {
        self.id
    }

    #[allow(dead_code)]
    pub fn rope(&self) -> &Rope {
        &self.rope
    }

    #[allow(dead_code)]
    pub fn rope_mut(&mut self) -> &mut Rope {
        self.dirty = true;
        &mut self.rope
    }

    #[allow(dead_code)]
    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    #[allow(dead_code)]
    pub fn set_file_path(&mut self, path: PathBuf) {
        self.file_path = Some(path);
    }

    #[allow(dead_code)]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[allow(dead_code)]
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Get text content as a string (renamed to avoid clippy warning)
    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        self.rope.to_string()
    }

    /// Get text content as a string
    pub fn content(&self) -> String {
        self.rope.to_string()
    }

    /// Get line count
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get a specific line as a string
    #[allow(dead_code)]
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.rope.len_lines() {
            let line = self.rope.line(line_idx);
            Some(line.to_string())
        } else {
            None
        }
    }

    /// Replace text in a range
    pub fn replace_range(&mut self, range: Range, new_text: &str) -> Result<(), String> {
        let start_char = self.position_to_char_idx(range.start)?;
        let end_char = self.position_to_char_idx(range.end)?;

        if start_char > end_char {
            return Err("Invalid range: start > end".to_string());
        }

        self.rope.remove(start_char..end_char);
        self.rope.insert(start_char, new_text);
        self.dirty = true;

        Ok(())
    }

    /// Convert a Position to a character index in the rope
    pub fn position_to_char_idx(&self, pos: Position) -> Result<usize, String> {
        if pos.line >= self.rope.len_lines() {
            return Err(format!("Line {} out of bounds", pos.line));
        }

        let line_start = self.rope.line_to_char(pos.line);
        let line = self.rope.line(pos.line);
        let line_len = line.len_chars();

        if pos.col > line_len {
            return Err(format!(
                "Column {} out of bounds for line {}",
                pos.col, pos.line
            ));
        }

        Ok(line_start + pos.col)
    }

    /// Convert a character index to a Position
    pub fn char_idx_to_position(&self, char_idx: usize) -> Result<Position, String> {
        if char_idx > self.rope.len_chars() {
            return Err(format!("Character index {} out of bounds", char_idx));
        }

        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let col = char_idx - line_start;

        Ok(Position::new(line, col))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = Buffer::new(BufferId(0));
        assert_eq!(buffer.id(), BufferId(0));
        assert_eq!(buffer.content(), "");
        assert!(!buffer.is_dirty());
    }

    #[test]
    fn test_buffer_from_string() {
        let content = "Hello\nWorld\n".to_string();
        let buffer = Buffer::from_string(BufferId(1), content.clone(), None);
        assert_eq!(buffer.content(), content);
        assert_eq!(buffer.line_count(), 3); // Includes empty line at end
    }

    #[test]
    fn test_buffer_line() {
        let buffer = Buffer::from_string(BufferId(0), "Hello\nWorld\n".to_string(), None);
        assert_eq!(buffer.line(0), Some("Hello\n".to_string()));
        assert_eq!(buffer.line(1), Some("World\n".to_string()));
        assert_eq!(buffer.line(2), Some("".to_string()));
        assert_eq!(buffer.line(3), None);
    }

    #[test]
    fn test_position_conversion() {
        let buffer = Buffer::from_string(BufferId(0), "Hello\nWorld\n".to_string(), None);

        let pos = Position::new(0, 0);
        let char_idx = buffer.position_to_char_idx(pos).unwrap();
        assert_eq!(char_idx, 0);

        let pos = Position::new(1, 0);
        let char_idx = buffer.position_to_char_idx(pos).unwrap();
        assert_eq!(char_idx, 6); // After "Hello\n"

        let converted_pos = buffer.char_idx_to_position(char_idx).unwrap();
        assert_eq!(converted_pos, pos);
    }

    #[test]
    fn test_replace_range() {
        let mut buffer = Buffer::from_string(BufferId(0), "Hello World".to_string(), None);

        let range = Range::new(Position::new(0, 6), Position::new(0, 11));
        buffer.replace_range(range, "Rust").unwrap();

        assert_eq!(buffer.content(), "Hello Rust");
        assert!(buffer.is_dirty());
    }
}
