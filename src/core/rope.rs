use ropey::Rope;

/// TextBuffer wraps ropey::Rope and provides text editing functionality
#[derive(Clone)]
pub struct TextBuffer {
    #[allow(dead_code)]
    rope: Rope,
    file_path: Option<String>,
}

impl TextBuffer {
    /// Creates a new empty text buffer
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            file_path: None,
        }
    }

    /// Creates a text buffer from a string
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            file_path: None,
        }
    }

    /// Sets the file path associated with this buffer
    pub fn set_file_path(&mut self, path: String) {
        self.file_path = Some(path);
    }

    /// Gets the file path if set
    pub fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }

    /// Gets the file extension from the file path
    #[allow(dead_code)]
    pub fn extension(&self) -> Option<&str> {
        self.file_path.as_ref().and_then(|path| {
            std::path::Path::new(path)
                .extension()
                .and_then(|ext| ext.to_str())
        })
    }

    /// Returns the entire text as a String
    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        self.rope.to_string()
    }

    /// Returns the length in bytes
    #[allow(dead_code)]
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Returns the number of lines
    #[allow(dead_code)]
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Returns true if the buffer is empty
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.rope.len_bytes() == 0
    }

    /// Inserts text at the given byte position
    #[allow(dead_code)]
    pub fn insert(&mut self, byte_idx: usize, text: &str) {
        self.rope.insert(byte_idx, text);
    }

    /// Removes text in the given byte range
    #[allow(dead_code)]
    pub fn remove(&mut self, byte_range: std::ops::Range<usize>) {
        self.rope.remove(byte_range);
    }

    /// Returns a line by index
    #[allow(dead_code)]
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.rope.len_lines() {
            Some(self.rope.line(line_idx).to_string())
        } else {
            None
        }
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}
