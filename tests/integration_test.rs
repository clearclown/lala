/// Integration tests for Lala Editor
///
/// These tests verify that core functionality works correctly

use lala::core_engine::{Buffer, BufferId, Position, Range};

#[test]
fn test_buffer_creation() {
    let buffer = Buffer::from_string(
        BufferId(0),
        String::new(),
        None,
    );
    assert_eq!(buffer.content(), "");
}

#[test]
fn test_buffer_from_string() {
    let text = "Hello, World!";
    let buffer = Buffer::from_string(
        BufferId(0),
        text.to_string(),
        None,
    );
    assert_eq!(buffer.content(), text);
}

#[test]
fn test_buffer_replace() {
    let mut buffer = Buffer::from_string(
        BufferId(0),
        "Hello World".to_string(),
        None,
    );

    // Replace "World" with "Rust"
    let range = Range::new(
        Position::new(0, 6),  // Start of "World"
        Position::new(0, 11), // End of "World"
    );
    buffer.replace_range(range, "Rust").unwrap();
    assert_eq!(buffer.content(), "Hello Rust");
}

#[test]
fn test_buffer_multiline() {
    let text = "Line 1\nLine 2\nLine 3\n";
    let buffer = Buffer::from_string(
        BufferId(0),
        text.to_string(),
        None,
    );

    assert_eq!(buffer.content(), text);
    assert_eq!(buffer.line_count(), 4); // 3 lines + empty line at end
}

#[test]
fn test_buffer_line_access() {
    let buffer = Buffer::from_string(
        BufferId(0),
        "First\nSecond\nThird".to_string(),
        None,
    );

    assert_eq!(buffer.line(0), Some("First\n".to_string()));
    assert_eq!(buffer.line(1), Some("Second\n".to_string()));
    assert_eq!(buffer.line(2), Some("Third".to_string()));
    assert_eq!(buffer.line(3), None);
}

#[test]
fn test_buffer_dirty_flag() {
    let mut buffer = Buffer::from_string(
        BufferId(0),
        "Test".to_string(),
        None,
    );

    assert!(!buffer.is_dirty());

    let range = Range::new(Position::new(0, 0), Position::new(0, 1));
    buffer.replace_range(range, "B").unwrap();

    assert!(buffer.is_dirty());

    buffer.mark_clean();
    assert!(!buffer.is_dirty());
}

#[test]
fn test_position_to_char_idx() {
    let buffer = Buffer::from_string(
        BufferId(0),
        "Hello\nWorld\n".to_string(),
        None,
    );

    // First line, first char
    assert_eq!(buffer.position_to_char_idx(Position::new(0, 0)).unwrap(), 0);

    // First line, after "Hello"
    assert_eq!(buffer.position_to_char_idx(Position::new(0, 5)).unwrap(), 5);

    // Second line, first char (after newline)
    assert_eq!(buffer.position_to_char_idx(Position::new(1, 0)).unwrap(), 6);

    // Second line, after "World"
    assert_eq!(buffer.position_to_char_idx(Position::new(1, 5)).unwrap(), 11);
}

#[test]
fn test_empty_buffer_line_count() {
    let buffer = Buffer::from_string(
        BufferId(0),
        String::new(),
        None,
    );

    // Empty buffer should have 1 line
    assert_eq!(buffer.line_count(), 1);
}

#[test]
fn test_single_line_no_newline() {
    let buffer = Buffer::from_string(
        BufferId(0),
        "Single line".to_string(),
        None,
    );

    assert_eq!(buffer.line_count(), 1);
    assert_eq!(buffer.content(), "Single line");
}
