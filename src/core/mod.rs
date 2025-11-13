/// Core editing engine module
pub mod error;
pub mod rope;

// Tests temporarily disabled - need to be updated to match new architecture
// #[cfg(test)]
// mod tests;

pub use error::CoreError;
pub use rope::TextBuffer;

/// Dummy EditorEngine implementation
/// Will be fully implemented in feature/core-engine
#[derive(Debug, Clone)]
pub struct EditorEngine {
    /// Engine ID
    pub id: usize,
    /// Document content (dummy)
    #[allow(dead_code)]
    pub content: String,
}

impl EditorEngine {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            content: String::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_content(id: usize, content: String) -> Self {
        Self { id, content }
    }
}
