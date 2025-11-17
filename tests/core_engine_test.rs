/// Comprehensive tests for core_engine module
///
/// Tests cover:
/// - Buffer operations (insert, delete, replace)
/// - Position and Range handling
/// - Multi-byte character support (Japanese, emojis)
/// - Edge cases and error handling
use lala::core_engine::{Buffer, BufferId, Position, Range};

// === Basic Buffer Operations ===

#[test]
fn test_buffer_insert_at_position() {
    let mut buffer = Buffer::from_string(BufferId(0), "Hello World".to_string(), None);

    // Insert at middle
    let pos = Position::new(0, 6);
    let range = Range::new(pos, pos);
    buffer.replace_range(range, "Beautiful ").unwrap();

    assert_eq!(buffer.content(), "Hello Beautiful World");
}

#[test]
fn test_buffer_delete_range() {
    let mut buffer = Buffer::from_string(BufferId(0), "Hello World".to_string(), None);

    // Delete "World"
    let range = Range::new(Position::new(0, 6), Position::new(0, 11));
    buffer.replace_range(range, "").unwrap();

    assert_eq!(buffer.content(), "Hello ");
}

#[test]
fn test_buffer_insert_newline() {
    let mut buffer = Buffer::from_string(BufferId(0), "Hello World".to_string(), None);

    // Insert newline after "Hello"
    let pos = Position::new(0, 5);
    let range = Range::new(pos, pos);
    buffer.replace_range(range, "\n").unwrap();

    assert_eq!(buffer.content(), "Hello\n World");
    assert_eq!(buffer.line_count(), 2);
}

#[test]
fn test_buffer_delete_newline() {
    let mut buffer = Buffer::from_string(BufferId(0), "Hello\nWorld".to_string(), None);

    // Delete newline
    let range = Range::new(Position::new(0, 5), Position::new(1, 0));
    buffer.replace_range(range, "").unwrap();

    assert_eq!(buffer.content(), "HelloWorld");
    assert_eq!(buffer.line_count(), 1);
}

// === Multi-byte Character Support ===

#[test]
fn test_japanese_characters() {
    let text = "こんにちは、世界！";
    let buffer = Buffer::from_string(BufferId(0), text.to_string(), None);

    assert_eq!(buffer.content(), text);
    assert_eq!(buffer.line_count(), 1);
}

#[test]
fn test_emoji_characters() {
    let text = "Hello 😀🎉🚀 World";
    let buffer = Buffer::from_string(BufferId(0), text.to_string(), None);

    assert_eq!(buffer.content(), text);
}

#[test]
fn test_mixed_multibyte_text() {
    let text = "Hello こんにちは 😀 World 世界";
    let buffer = Buffer::from_string(BufferId(0), text.to_string(), None);

    assert_eq!(buffer.content(), text);
}

#[test]
fn test_japanese_multiline() {
    let text = "日本語の\nテキスト\nエディタ";
    let buffer = Buffer::from_string(BufferId(0), text.to_string(), None);

    assert_eq!(buffer.content(), text);
    assert_eq!(buffer.line_count(), 3);
    assert_eq!(buffer.line(0), Some("日本語の\n".to_string()));
    assert_eq!(buffer.line(1), Some("テキスト\n".to_string()));
    assert_eq!(buffer.line(2), Some("エディタ".to_string()));
}

// === Edge Cases ===

#[test]
fn test_very_long_line() {
    let long_text = "a".repeat(10000);
    let buffer = Buffer::from_string(BufferId(0), long_text.clone(), None);

    assert_eq!(buffer.content(), long_text);
    assert_eq!(buffer.line_count(), 1);
}

#[test]
fn test_many_lines() {
    let text = (0..1000)
        .map(|i| format!("Line {i}"))
        .collect::<Vec<_>>()
        .join("\n");
    let buffer = Buffer::from_string(BufferId(0), text.clone(), None);

    assert_eq!(buffer.line_count(), 1000);
}

#[test]
fn test_empty_lines() {
    let text = "\n\n\n\n\n";
    let buffer = Buffer::from_string(BufferId(0), text.to_string(), None);

    assert_eq!(buffer.content(), text);
    assert_eq!(buffer.line_count(), 6); // 5 newlines = 6 lines
}

#[test]
fn test_trailing_newlines() {
    let text = "Line 1\nLine 2\n\n\n";
    let buffer = Buffer::from_string(BufferId(0), text.to_string(), None);

    assert_eq!(buffer.content(), text);
    assert_eq!(buffer.line_count(), 5); // Line1, Line2, empty, empty, empty
}

// === Position and Range Tests ===

#[test]
fn test_position_equality() {
    let pos1 = Position::new(5, 10);
    let pos2 = Position::new(5, 10);
    let pos3 = Position::new(5, 11);

    assert_eq!(pos1, pos2);
    assert_ne!(pos1, pos3);
}

#[test]
fn test_range_creation() {
    let range = Range::new(Position::new(0, 5), Position::new(0, 10));

    // Verify range was created
    assert_eq!(range.start, Position::new(0, 5));
    assert_eq!(range.end, Position::new(0, 10));
}

#[test]
fn test_multiline_range_creation() {
    let _buffer = Buffer::from_string(BufferId(0), "Line 1\nLine 2\nLine 3".to_string(), None);

    // Get range from middle of line 1 to middle of line 2
    let start = Position::new(0, 3); // "e 1\n"
    let end = Position::new(1, 4); // "Line"

    let range = Range::new(start, end);
    assert_eq!(range.start, start);
    assert_eq!(range.end, end);
}

// === Buffer State Management ===

#[test]
fn test_buffer_dirty_after_multiple_edits() {
    let mut buffer = Buffer::from_string(BufferId(0), "Test".to_string(), None);

    // Multiple edits
    buffer
        .replace_range(Range::new(Position::new(0, 0), Position::new(0, 1)), "B")
        .unwrap();
    buffer
        .replace_range(Range::new(Position::new(0, 1), Position::new(0, 2)), "o")
        .unwrap();

    assert!(buffer.is_dirty());
    assert_eq!(buffer.content(), "Bost");
}

#[test]
fn test_buffer_mark_clean_and_dirty_again() {
    let mut buffer = Buffer::from_string(BufferId(0), "Test".to_string(), None);

    buffer
        .replace_range(Range::new(Position::new(0, 0), Position::new(0, 1)), "B")
        .unwrap();
    assert!(buffer.is_dirty());

    buffer.mark_clean();
    assert!(!buffer.is_dirty());

    buffer
        .replace_range(Range::new(Position::new(0, 1), Position::new(0, 2)), "o")
        .unwrap();
    assert!(buffer.is_dirty());
}

// === File Path Management ===

#[test]
fn test_buffer_with_file_path() {
    use std::path::PathBuf;

    let path = PathBuf::from("/tmp/test.txt");
    let buffer = Buffer::from_string(BufferId(0), "Test".to_string(), Some(path.clone()));

    assert_eq!(buffer.file_path(), Some(&path));
}

#[test]
fn test_buffer_without_file_path() {
    let buffer = Buffer::from_string(BufferId(0), "Test".to_string(), None);

    assert_eq!(buffer.file_path(), None);
}

// === Error Handling ===

#[test]
fn test_invalid_position() {
    let buffer = Buffer::from_string(BufferId(0), "Hello".to_string(), None);

    // Position beyond buffer
    let result = buffer.position_to_char_idx(Position::new(10, 0));
    assert!(result.is_err());
}

#[test]
fn test_replace_range_beyond_buffer() {
    let mut buffer = Buffer::from_string(BufferId(0), "Hello".to_string(), None);

    // Try to replace beyond buffer
    let range = Range::new(Position::new(0, 0), Position::new(10, 0));
    let result = buffer.replace_range(range, "X");

    assert!(result.is_err());
}

// === Performance Tests ===

#[test]
fn test_large_buffer_performance() {
    // Create a large buffer (1MB of text)
    let text = "a".repeat(1_000_000);
    let buffer = Buffer::from_string(BufferId(0), text.clone(), None);

    assert_eq!(buffer.content().len(), 1_000_000);
}

#[test]
fn test_many_small_edits() {
    let mut buffer = Buffer::from_string(BufferId(0), "".to_string(), None);

    // Perform 1000 small insertions
    for i in 0..1000 {
        let pos = Position::new(0, i);
        let range = Range::new(pos, pos);
        buffer.replace_range(range, "a").unwrap();
    }

    assert_eq!(buffer.content().len(), 1000);
}

// === Line Access Tests ===

#[test]
fn test_line_access_with_crlf() {
    // Windows-style line endings
    let buffer = Buffer::from_string(BufferId(0), "Line1\r\nLine2\r\nLine3".to_string(), None);

    assert_eq!(buffer.line_count(), 3);
}

#[test]
fn test_last_line_without_newline() {
    let buffer = Buffer::from_string(BufferId(0), "Line1\nLine2".to_string(), None);

    assert_eq!(buffer.line(0), Some("Line1\n".to_string()));
    assert_eq!(buffer.line(1), Some("Line2".to_string()));
    assert_eq!(buffer.line(2), None);
}

#[test]
fn test_line_access_empty_buffer() {
    let buffer = Buffer::from_string(BufferId(0), "".to_string(), None);

    assert_eq!(buffer.line(0), Some("".to_string()));
    assert_eq!(buffer.line(1), None);
}
