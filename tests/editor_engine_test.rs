/// Comprehensive tests for EditorEngine
///
/// Tests cover:
/// - Engine creation
/// - Engine ID handling
/// - Content management
/// - Clone behavior
/// - Debug trait
use lala::core::EditorEngine;

// ============================================
// === Basic Creation Tests ===
// ============================================

mod creation_tests {
    use super::*;

    #[test]
    fn test_new_engine() {
        let engine = EditorEngine::new(0);

        assert_eq!(engine.id, 0);
        assert_eq!(engine.content, "");
    }

    #[test]
    fn test_new_engine_with_different_ids() {
        let engine1 = EditorEngine::new(0);
        let engine2 = EditorEngine::new(1);
        let engine3 = EditorEngine::new(100);

        assert_eq!(engine1.id, 0);
        assert_eq!(engine2.id, 1);
        assert_eq!(engine3.id, 100);
    }

    #[test]
    fn test_new_engine_max_id() {
        let engine = EditorEngine::new(usize::MAX);

        assert_eq!(engine.id, usize::MAX);
    }

    #[test]
    fn test_with_content() {
        let engine = EditorEngine::with_content(0, "Hello, World!".to_string());

        assert_eq!(engine.id, 0);
        assert_eq!(engine.content, "Hello, World!");
    }

    #[test]
    fn test_with_content_empty() {
        let engine = EditorEngine::with_content(0, String::new());

        assert_eq!(engine.content, "");
    }

    #[test]
    fn test_with_content_multiline() {
        let content = "Line 1\nLine 2\nLine 3".to_string();
        let engine = EditorEngine::with_content(0, content.clone());

        assert_eq!(engine.content, content);
    }

    #[test]
    fn test_with_content_unicode() {
        let content = "日本語テキスト 🎉".to_string();
        let engine = EditorEngine::with_content(0, content.clone());

        assert_eq!(engine.content, content);
    }
}

// ============================================
// === Clone Tests ===
// ============================================

mod clone_tests {
    use super::*;

    #[test]
    fn test_clone_empty_engine() {
        let engine = EditorEngine::new(42);
        let cloned = engine.clone();

        assert_eq!(engine.id, cloned.id);
        assert_eq!(engine.content, cloned.content);
    }

    #[test]
    fn test_clone_with_content() {
        let engine = EditorEngine::with_content(10, "Some content".to_string());
        let cloned = engine.clone();

        assert_eq!(cloned.id, 10);
        assert_eq!(cloned.content, "Some content");
    }

    #[test]
    fn test_clone_independence() {
        let engine = EditorEngine::with_content(0, "Original".to_string());
        let mut cloned = engine.clone();

        cloned.content = "Modified".to_string();

        assert_eq!(engine.content, "Original");
        assert_eq!(cloned.content, "Modified");
    }
}

// ============================================
// === Debug Trait Tests ===
// ============================================

mod debug_tests {
    use super::*;

    #[test]
    fn test_engine_debug() {
        let engine = EditorEngine::new(42);
        let debug_str = format!("{:?}", engine);

        assert!(debug_str.contains("EditorEngine"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_engine_debug_with_content() {
        let engine = EditorEngine::with_content(0, "test content".to_string());
        let debug_str = format!("{:?}", engine);

        assert!(debug_str.contains("test content"));
    }
}

// ============================================
// === Content Tests ===
// ============================================

mod content_tests {
    use super::*;

    #[test]
    fn test_content_field_access() {
        let engine = EditorEngine::with_content(0, "Content".to_string());

        // Direct field access is allowed
        assert_eq!(engine.content, "Content");
    }

    #[test]
    fn test_content_large() {
        let content = "x".repeat(100000);
        let engine = EditorEngine::with_content(0, content.clone());

        assert_eq!(engine.content.len(), 100000);
    }

    #[test]
    fn test_content_special_chars() {
        let content = "Tab:\tNewline:\nCarriage:\r".to_string();
        let engine = EditorEngine::with_content(0, content.clone());

        assert!(engine.content.contains('\t'));
        assert!(engine.content.contains('\n'));
        assert!(engine.content.contains('\r'));
    }

    #[test]
    fn test_content_null_char() {
        let content = "before\0after".to_string();
        let engine = EditorEngine::with_content(0, content.clone());

        assert!(engine.content.contains('\0'));
    }
}

// ============================================
// === ID Tests ===
// ============================================

mod id_tests {
    use super::*;

    #[test]
    fn test_id_field_access() {
        let engine = EditorEngine::new(999);

        // Direct field access is allowed
        assert_eq!(engine.id, 999);
    }

    #[test]
    fn test_id_zero() {
        let engine = EditorEngine::new(0);

        assert_eq!(engine.id, 0);
    }

    #[test]
    fn test_id_sequence() {
        let engines: Vec<EditorEngine> = (0..100).map(EditorEngine::new).collect();

        for (i, engine) in engines.iter().enumerate() {
            assert_eq!(engine.id, i);
        }
    }
}

// ============================================
// === Edge Cases ===
// ============================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_many_engines() {
        let engines: Vec<EditorEngine> = (0..1000).map(EditorEngine::new).collect();

        assert_eq!(engines.len(), 1000);
        assert_eq!(engines[0].id, 0);
        assert_eq!(engines[999].id, 999);
    }

    #[test]
    fn test_engine_with_windows_line_endings() {
        let content = "Line1\r\nLine2\r\n".to_string();
        let engine = EditorEngine::with_content(0, content);

        assert!(engine.content.contains("\r\n"));
    }

    #[test]
    fn test_engine_with_emoji() {
        let content = "🎉🎊🎁📝✨".to_string();
        let engine = EditorEngine::with_content(0, content.clone());

        assert_eq!(engine.content, content);
    }

    #[test]
    fn test_engine_with_mixed_unicode() {
        let content = "English 日本語 한국어 العربية".to_string();
        let engine = EditorEngine::with_content(0, content.clone());

        assert_eq!(engine.content, content);
    }
}
