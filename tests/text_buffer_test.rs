/// Comprehensive tests for TextBuffer (rope-based text storage)
///
/// Tests cover:
/// - Buffer creation
/// - Text manipulation (insert, remove)
/// - Line operations
/// - File path handling
/// - Edge cases
use lala::core::TextBuffer;

// ============================================
// === Basic Creation Tests ===
// ============================================

mod creation_tests {
    use super::*;

    #[test]
    fn test_new_empty_buffer() {
        let buffer = TextBuffer::new();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len_bytes(), 0);
    }

    #[test]
    fn test_from_str_simple() {
        let buffer = TextBuffer::from_str("Hello, World!");
        assert!(!buffer.is_empty());
        assert_eq!(buffer.as_string(), "Hello, World!");
    }

    #[test]
    fn test_from_str_empty() {
        let buffer = TextBuffer::from_str("");
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_from_str_multiline() {
        let buffer = TextBuffer::from_str("Line 1\nLine 2\nLine 3");
        assert!(!buffer.is_empty());
        assert_eq!(buffer.len_lines(), 3);
    }

    #[test]
    fn test_default() {
        let buffer = TextBuffer::default();
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_clone() {
        let buffer = TextBuffer::from_str("Test content");
        let cloned = buffer.clone();
        assert_eq!(cloned.as_string(), buffer.as_string());
    }
}

// ============================================
// === Length Tests ===
// ============================================

mod length_tests {
    use super::*;

    #[test]
    fn test_len_bytes_ascii() {
        let buffer = TextBuffer::from_str("Hello");
        assert_eq!(buffer.len_bytes(), 5);
    }

    #[test]
    fn test_len_bytes_unicode() {
        let buffer = TextBuffer::from_str("日本語");
        // Each Japanese character is 3 bytes in UTF-8
        assert_eq!(buffer.len_bytes(), 9);
    }

    #[test]
    fn test_len_bytes_mixed() {
        let buffer = TextBuffer::from_str("Hello 日本語");
        // "Hello " = 6 bytes, "日本語" = 9 bytes
        assert_eq!(buffer.len_bytes(), 15);
    }

    #[test]
    fn test_len_bytes_empty() {
        let buffer = TextBuffer::new();
        assert_eq!(buffer.len_bytes(), 0);
    }

    #[test]
    fn test_len_lines_single() {
        let buffer = TextBuffer::from_str("Single line");
        assert_eq!(buffer.len_lines(), 1);
    }

    #[test]
    fn test_len_lines_multiple() {
        let buffer = TextBuffer::from_str("Line 1\nLine 2\nLine 3\n");
        assert_eq!(buffer.len_lines(), 4); // Including empty line after last \n
    }

    #[test]
    fn test_len_lines_empty() {
        let buffer = TextBuffer::new();
        assert_eq!(buffer.len_lines(), 1); // Empty buffer has 1 line
    }

    #[test]
    fn test_len_lines_newlines_only() {
        let buffer = TextBuffer::from_str("\n\n\n");
        assert_eq!(buffer.len_lines(), 4);
    }

    #[test]
    fn test_is_empty_true() {
        let buffer = TextBuffer::new();
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_is_empty_false() {
        let buffer = TextBuffer::from_str("a");
        assert!(!buffer.is_empty());
    }
}

// ============================================
// === Insert Tests ===
// ============================================

mod insert_tests {
    use super::*;

    #[test]
    fn test_insert_at_start() {
        let mut buffer = TextBuffer::from_str("World");
        buffer.insert(0, "Hello ");
        assert_eq!(buffer.as_string(), "Hello World");
    }

    #[test]
    fn test_insert_at_end() {
        let mut buffer = TextBuffer::from_str("Hello");
        buffer.insert(5, " World");
        assert_eq!(buffer.as_string(), "Hello World");
    }

    #[test]
    fn test_insert_in_middle() {
        let mut buffer = TextBuffer::from_str("HWorld");
        buffer.insert(1, "ello ");
        assert_eq!(buffer.as_string(), "Hello World");
    }

    #[test]
    fn test_insert_into_empty() {
        let mut buffer = TextBuffer::new();
        buffer.insert(0, "Hello");
        assert_eq!(buffer.as_string(), "Hello");
    }

    #[test]
    fn test_insert_newline() {
        let mut buffer = TextBuffer::from_str("Line1Line2");
        buffer.insert(5, "\n");
        assert_eq!(buffer.as_string(), "Line1\nLine2");
        assert_eq!(buffer.len_lines(), 2);
    }

    #[test]
    fn test_insert_unicode() {
        let mut buffer = TextBuffer::from_str("Hello World");
        buffer.insert(6, "日本語 ");
        assert_eq!(buffer.as_string(), "Hello 日本語 World");
    }

    #[test]
    fn test_multiple_inserts() {
        let mut buffer = TextBuffer::new();
        buffer.insert(0, "a");
        buffer.insert(1, "b");
        buffer.insert(2, "c");
        assert_eq!(buffer.as_string(), "abc");
    }
}

// ============================================
// === Remove Tests ===
// ============================================

mod remove_tests {
    use super::*;

    #[test]
    fn test_remove_from_start() {
        let mut buffer = TextBuffer::from_str("Hello World");
        buffer.remove(0..6);
        assert_eq!(buffer.as_string(), "World");
    }

    #[test]
    fn test_remove_from_end() {
        let mut buffer = TextBuffer::from_str("Hello World");
        buffer.remove(5..11);
        assert_eq!(buffer.as_string(), "Hello");
    }

    #[test]
    fn test_remove_from_middle() {
        let mut buffer = TextBuffer::from_str("Hello World");
        buffer.remove(5..6);
        assert_eq!(buffer.as_string(), "HelloWorld");
    }

    #[test]
    fn test_remove_all() {
        let mut buffer = TextBuffer::from_str("Hello");
        buffer.remove(0..5);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_remove_single_char() {
        let mut buffer = TextBuffer::from_str("abc");
        buffer.remove(1..2);
        assert_eq!(buffer.as_string(), "ac");
    }

    #[test]
    fn test_remove_newline() {
        let mut buffer = TextBuffer::from_str("Line1\nLine2");
        buffer.remove(5..6);
        assert_eq!(buffer.as_string(), "Line1Line2");
    }
}

// ============================================
// === Line Access Tests ===
// ============================================

mod line_tests {
    use super::*;

    #[test]
    fn test_line_first() {
        let buffer = TextBuffer::from_str("Line 1\nLine 2\nLine 3");
        assert_eq!(buffer.line(0), Some("Line 1\n".to_string()));
    }

    #[test]
    fn test_line_middle() {
        let buffer = TextBuffer::from_str("Line 1\nLine 2\nLine 3");
        assert_eq!(buffer.line(1), Some("Line 2\n".to_string()));
    }

    #[test]
    fn test_line_last() {
        let buffer = TextBuffer::from_str("Line 1\nLine 2\nLine 3");
        assert_eq!(buffer.line(2), Some("Line 3".to_string()));
    }

    #[test]
    fn test_line_out_of_bounds() {
        let buffer = TextBuffer::from_str("Single line");
        assert_eq!(buffer.line(1), None);
        assert_eq!(buffer.line(100), None);
    }

    #[test]
    fn test_line_single_line_buffer() {
        let buffer = TextBuffer::from_str("Only one line");
        assert_eq!(buffer.line(0), Some("Only one line".to_string()));
    }

    #[test]
    fn test_line_empty_buffer() {
        let buffer = TextBuffer::new();
        assert_eq!(buffer.line(0), Some("".to_string()));
    }

    #[test]
    fn test_line_with_unicode() {
        let buffer = TextBuffer::from_str("日本語\nEnglish\nMixed 日本語");
        assert_eq!(buffer.line(0), Some("日本語\n".to_string()));
        assert_eq!(buffer.line(1), Some("English\n".to_string()));
        assert_eq!(buffer.line(2), Some("Mixed 日本語".to_string()));
    }
}

// ============================================
// === File Path Tests ===
// ============================================

mod file_path_tests {
    use super::*;

    #[test]
    fn test_file_path_none_by_default() {
        let buffer = TextBuffer::new();
        assert!(buffer.file_path().is_none());
    }

    #[test]
    fn test_set_file_path() {
        let mut buffer = TextBuffer::new();
        buffer.set_file_path("/path/to/file.txt".to_string());
        assert_eq!(buffer.file_path(), Some("/path/to/file.txt"));
    }

    #[test]
    fn test_extension_txt() {
        let mut buffer = TextBuffer::new();
        buffer.set_file_path("/path/to/file.txt".to_string());
        assert_eq!(buffer.extension(), Some("txt"));
    }

    #[test]
    fn test_extension_rs() {
        let mut buffer = TextBuffer::new();
        buffer.set_file_path("/path/to/main.rs".to_string());
        assert_eq!(buffer.extension(), Some("rs"));
    }

    #[test]
    fn test_extension_md() {
        let mut buffer = TextBuffer::new();
        buffer.set_file_path("README.md".to_string());
        assert_eq!(buffer.extension(), Some("md"));
    }

    #[test]
    fn test_extension_none_no_path() {
        let buffer = TextBuffer::new();
        assert!(buffer.extension().is_none());
    }

    #[test]
    fn test_extension_none_no_extension() {
        let mut buffer = TextBuffer::new();
        buffer.set_file_path("Makefile".to_string());
        assert!(buffer.extension().is_none());
    }

    #[test]
    fn test_extension_hidden_file() {
        let mut buffer = TextBuffer::new();
        buffer.set_file_path(".gitignore".to_string());
        // Rust's Path::extension() returns None for dotfiles like .gitignore
        // because .gitignore is considered a file name without extension
        assert_eq!(buffer.extension(), None);
    }

    #[test]
    fn test_extension_multiple_dots() {
        let mut buffer = TextBuffer::new();
        buffer.set_file_path("archive.tar.gz".to_string());
        assert_eq!(buffer.extension(), Some("gz"));
    }

    #[test]
    fn test_file_path_unicode() {
        let mut buffer = TextBuffer::new();
        buffer.set_file_path("/path/日本語/ファイル.md".to_string());
        assert_eq!(buffer.file_path(), Some("/path/日本語/ファイル.md"));
        assert_eq!(buffer.extension(), Some("md"));
    }
}

// ============================================
// === As String Tests ===
// ============================================

mod as_string_tests {
    use super::*;

    #[test]
    fn test_as_string_simple() {
        let buffer = TextBuffer::from_str("Hello, World!");
        assert_eq!(buffer.as_string(), "Hello, World!");
    }

    #[test]
    fn test_as_string_empty() {
        let buffer = TextBuffer::new();
        assert_eq!(buffer.as_string(), "");
    }

    #[test]
    fn test_as_string_multiline() {
        let buffer = TextBuffer::from_str("Line 1\nLine 2\n");
        assert_eq!(buffer.as_string(), "Line 1\nLine 2\n");
    }

    #[test]
    fn test_as_string_preserves_whitespace() {
        let buffer = TextBuffer::from_str("  leading\ntrailing  \n  both  ");
        assert_eq!(buffer.as_string(), "  leading\ntrailing  \n  both  ");
    }

    #[test]
    fn test_as_string_unicode() {
        let buffer = TextBuffer::from_str("日本語テスト 🎉");
        assert_eq!(buffer.as_string(), "日本語テスト 🎉");
    }
}

// ============================================
// === Clone Tests ===
// ============================================

mod clone_tests {
    use super::*;

    #[test]
    fn test_clone_content() {
        let buffer = TextBuffer::from_str("Test content");
        let cloned = buffer.clone();
        assert_eq!(buffer.as_string(), cloned.as_string());
    }

    #[test]
    fn test_clone_with_path() {
        let mut buffer = TextBuffer::from_str("Content");
        buffer.set_file_path("/path/to/file.txt".to_string());

        let cloned = buffer.clone();
        assert_eq!(cloned.file_path(), buffer.file_path());
    }

    #[test]
    fn test_clone_independence() {
        let mut buffer = TextBuffer::from_str("Original");
        let mut cloned = buffer.clone();

        buffer.insert(8, " modified");
        cloned.insert(8, " cloned");

        assert_eq!(buffer.as_string(), "Original modified");
        assert_eq!(cloned.as_string(), "Original cloned");
    }
}

// ============================================
// === Edge Cases ===
// ============================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_large_content() {
        let content = "x".repeat(100000);
        let buffer = TextBuffer::from_str(&content);
        assert_eq!(buffer.len_bytes(), 100000);
    }

    #[test]
    fn test_many_lines() {
        let content = "line\n".repeat(10000);
        let buffer = TextBuffer::from_str(&content);
        assert_eq!(buffer.len_lines(), 10001); // Including empty line at end
    }

    #[test]
    fn test_very_long_line() {
        let content = "a".repeat(100000);
        let buffer = TextBuffer::from_str(&content);
        assert_eq!(buffer.len_lines(), 1);
        assert_eq!(buffer.line(0), Some(content));
    }

    #[test]
    fn test_special_characters() {
        let buffer = TextBuffer::from_str("Tab:\tNewline:\nCarriage:\r");
        assert!(buffer.as_string().contains('\t'));
        assert!(buffer.as_string().contains('\n'));
        assert!(buffer.as_string().contains('\r'));
    }

    #[test]
    fn test_emoji() {
        let buffer = TextBuffer::from_str("🎉🎊🎁");
        assert!(!buffer.is_empty());
    }

    #[test]
    fn test_null_character() {
        let buffer = TextBuffer::from_str("before\0after");
        assert!(buffer.as_string().contains('\0'));
    }

    #[test]
    fn test_windows_line_endings() {
        let buffer = TextBuffer::from_str("Line 1\r\nLine 2\r\n");
        assert!(buffer.as_string().contains("\r\n"));
    }
}
