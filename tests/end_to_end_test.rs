/// End-to-end integration tests
///
/// These tests verify complete workflows combining multiple modules
use lala::core_engine::{Buffer, BufferId, Position, Range};
use std::fs;
use tempfile::TempDir;

// === Complete Edit Workflow ===

#[test]
fn test_complete_edit_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // 1. Create initial file
    let initial_content = "Hello World\nThis is a test\nGoodbye World";
    fs::write(&file_path, initial_content).unwrap();

    // 2. Load into buffer
    let content = fs::read_to_string(&file_path).unwrap();
    let mut buffer = Buffer::from_string(BufferId(0), content, Some(file_path.clone()));

    assert_eq!(buffer.content(), initial_content);
    assert!(!buffer.is_dirty());

    // 3. Make edits
    let range = Range::new(Position::new(0, 6), Position::new(0, 11)); // "World"
    buffer.replace_range(range, "Rust").unwrap();

    assert_eq!(
        buffer.content(),
        "Hello Rust\nThis is a test\nGoodbye World"
    );
    assert!(buffer.is_dirty());

    // 4. Save
    fs::write(&file_path, buffer.content()).unwrap();
    buffer.mark_clean();

    assert!(!buffer.is_dirty());

    // 5. Verify saved content
    let saved_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(saved_content, "Hello Rust\nThis is a test\nGoodbye World");
}

// === Manual Search and Replace Workflow ===

#[test]
fn test_manual_search_and_replace_workflow() {
    let mut buffer = Buffer::from_string(BufferId(0), "foo bar foo baz".to_string(), None);

    // Manually find and replace first "foo"
    let range1 = Range::new(Position::new(0, 0), Position::new(0, 3));
    buffer.replace_range(range1, "qux").unwrap();

    assert_eq!(buffer.content(), "qux bar foo baz");

    // Manually find and replace second "foo"
    let range2 = Range::new(Position::new(0, 8), Position::new(0, 11));
    buffer.replace_range(range2, "qux").unwrap();

    assert_eq!(buffer.content(), "qux bar qux baz");
}

// === Multi-file Workflow ===

#[test]
fn test_multi_file_workflow() {
    let temp_dir = TempDir::new().unwrap();

    // Create multiple files
    let file1_path = temp_dir.path().join("file1.txt");
    let file2_path = temp_dir.path().join("file2.txt");
    let file3_path = temp_dir.path().join("file3.txt");

    fs::write(&file1_path, "Content 1").unwrap();
    fs::write(&file2_path, "Content 2").unwrap();
    fs::write(&file3_path, "Content 3").unwrap();

    // Load all files into buffers
    let buffer1 = Buffer::from_string(
        BufferId(0),
        fs::read_to_string(&file1_path).unwrap(),
        Some(file1_path.clone()),
    );

    let buffer2 = Buffer::from_string(
        BufferId(1),
        fs::read_to_string(&file2_path).unwrap(),
        Some(file2_path.clone()),
    );

    let buffer3 = Buffer::from_string(
        BufferId(2),
        fs::read_to_string(&file3_path).unwrap(),
        Some(file3_path.clone()),
    );

    assert_eq!(buffer1.content(), "Content 1");
    assert_eq!(buffer2.content(), "Content 2");
    assert_eq!(buffer3.content(), "Content 3");
}

// === Markdown Editing Workflow ===

#[test]
fn test_markdown_editing_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("document.md");

    // 1. Create markdown file
    let initial_markdown =
        "# Title\n\n## Section 1\n\nParagraph text.\n\n## Section 2\n\nMore text.";
    fs::write(&file_path, initial_markdown).unwrap();

    // 2. Load and edit
    let mut buffer = Buffer::from_string(
        BufferId(0),
        fs::read_to_string(&file_path).unwrap(),
        Some(file_path.clone()),
    );

    // Add new section
    let insert_pos = Position::new(4, 11); // After "Paragraph text."
    let range = Range::new(insert_pos, insert_pos);
    buffer
        .replace_range(range, "\n\n### Subsection\n\nNew content.")
        .unwrap();

    // 3. Verify structure
    let content = buffer.content();
    assert!(content.contains("# Title"));
    assert!(content.contains("## Section 1"));
    assert!(content.contains("### Subsection"));
    assert!(content.contains("## Section 2"));
}

// === Japanese Text Workflow ===

#[test]

// === Large File Workflow ===

// === Undo/Redo Workflow (if implemented) ===

fn test_buffer_state_tracking() {
    let mut buffer = Buffer::from_string(BufferId(0), "Original".to_string(), None);

    // Track state changes
    assert!(!buffer.is_dirty());

    // Change 1
    let range1 = Range::new(Position::new(0, 0), Position::new(0, 8));
    buffer.replace_range(range1, "First").unwrap();
    assert!(buffer.is_dirty());
    assert_eq!(buffer.content(), "First");

    // Mark clean (simulate save)
    buffer.mark_clean();
    assert!(!buffer.is_dirty());

    // Change 2
    let range2 = Range::new(Position::new(0, 0), Position::new(0, 5));
    buffer.replace_range(range2, "Second").unwrap();
    assert!(buffer.is_dirty());
    assert_eq!(buffer.content(), "Second");
}

// === Error Recovery Workflow ===

#[test]
fn test_error_recovery_workflow() {
    let mut buffer = Buffer::from_string(BufferId(0), "Test content".to_string(), None);

    // Try invalid operation
    let invalid_range = Range::new(Position::new(100, 0), Position::new(100, 5));
    let result = buffer.replace_range(invalid_range, "X");

    assert!(result.is_err());

    // Buffer should remain unchanged
    assert_eq!(buffer.content(), "Test content");
    assert!(!buffer.is_dirty());

    // Valid operation should still work
    let valid_range = Range::new(Position::new(0, 0), Position::new(0, 4));
    let result = buffer.replace_range(valid_range, "Best");

    assert!(result.is_ok());
    assert_eq!(buffer.content(), "Best content");
}

// === Concurrent Buffer Management ===

#[test]
fn test_multiple_buffers() {
    let mut buffers = vec![];

    // Create multiple buffers
    for i in 0..10 {
        let buffer = Buffer::from_string(BufferId(i), format!("Buffer {i}"), None);
        buffers.push(buffer);
    }

    // Verify each buffer
    for (i, buffer) in buffers.iter().enumerate() {
        assert_eq!(buffer.content(), format!("Buffer {i}"));
    }

    // Edit specific buffer
    let range = Range::new(Position::new(0, 0), Position::new(0, 8));
    buffers[5]
        .replace_range(range, "Modified Buffer 5")
        .unwrap();

    assert_eq!(buffers[5].content(), "Modified Buffer 5");
    assert_eq!(buffers[4].content(), "Buffer 4"); // Other buffers unchanged
}

// === Complex Search Pattern Workflow ===

#[test]

// === File Extension Detection ===

fn test_file_extension_detection() {
    let markdown_path = std::path::PathBuf::from("test.md");
    let html_path = std::path::PathBuf::from("test.html");
    let latex_path = std::path::PathBuf::from("test.tex");
    let text_path = std::path::PathBuf::from("test.txt");

    let markdown_buffer = Buffer::from_string(
        BufferId(0),
        "# Markdown".to_string(),
        Some(markdown_path.clone()),
    );

    let html_buffer = Buffer::from_string(
        BufferId(1),
        "<html></html>".to_string(),
        Some(html_path.clone()),
    );

    let latex_buffer = Buffer::from_string(
        BufferId(2),
        "\\documentclass".to_string(),
        Some(latex_path.clone()),
    );

    let text_buffer = Buffer::from_string(
        BufferId(3),
        "Plain text".to_string(),
        Some(text_path.clone()),
    );

    assert_eq!(
        markdown_buffer.file_path().unwrap().extension().unwrap(),
        "md"
    );
    assert_eq!(
        html_buffer.file_path().unwrap().extension().unwrap(),
        "html"
    );
    assert_eq!(
        latex_buffer.file_path().unwrap().extension().unwrap(),
        "tex"
    );
    assert_eq!(text_buffer.file_path().unwrap().extension().unwrap(), "txt");
}
