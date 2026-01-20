/// Comprehensive tests for EditorTabState
///
/// Tests cover:
/// - Tab creation
/// - Title generation
/// - Modification state
/// - Engine integration
use lala::gui::EditorTabState;

// ============================================
// === Basic Creation Tests ===
// ============================================

mod creation_tests {
    use super::*;

    #[test]
    fn test_new_tab() {
        let tab = EditorTabState::new(0, "test.md");

        assert_eq!(tab.id, 0);
        assert_eq!(tab.file_name, "test.md");
        assert!(!tab.is_modified);
    }

    #[test]
    fn test_new_tab_with_string() {
        let tab = EditorTabState::new(1, String::from("owned.md"));

        assert_eq!(tab.id, 1);
        assert_eq!(tab.file_name, "owned.md");
    }

    #[test]
    fn test_new_tab_different_ids() {
        let tab1 = EditorTabState::new(0, "a.md");
        let tab2 = EditorTabState::new(1, "b.md");
        let tab3 = EditorTabState::new(100, "c.md");

        assert_eq!(tab1.id, 0);
        assert_eq!(tab2.id, 1);
        assert_eq!(tab3.id, 100);
    }

    #[test]
    fn test_new_tab_empty_name() {
        let tab = EditorTabState::new(0, "");

        assert_eq!(tab.file_name, "");
    }

    #[test]
    fn test_new_tab_unicode_name() {
        let tab = EditorTabState::new(0, "日本語ファイル.md");

        assert_eq!(tab.file_name, "日本語ファイル.md");
    }

    #[test]
    fn test_new_tab_special_chars() {
        let tab = EditorTabState::new(0, "file with spaces.md");

        assert_eq!(tab.file_name, "file with spaces.md");
    }

    #[test]
    fn test_new_tab_various_extensions() {
        let extensions = vec!["md", "rs", "txt", "py", "js", "json", "yaml"];

        for ext in extensions {
            let name = format!("file.{}", ext);
            let tab = EditorTabState::new(0, &name);
            assert_eq!(tab.file_name, name);
        }
    }
}

// ============================================
// === Title Tests ===
// ============================================

mod title_tests {
    use super::*;

    #[test]
    fn test_title_unmodified() {
        let tab = EditorTabState::new(0, "test.md");

        assert_eq!(tab.title(), "test.md");
    }

    #[test]
    fn test_title_modified() {
        let mut tab = EditorTabState::new(0, "test.md");
        tab.mark_modified();

        assert_eq!(tab.title(), "test.md*");
    }

    #[test]
    fn test_title_after_save() {
        let mut tab = EditorTabState::new(0, "test.md");
        tab.mark_modified();
        tab.mark_saved();

        assert_eq!(tab.title(), "test.md");
    }

    #[test]
    fn test_title_empty_name() {
        let tab = EditorTabState::new(0, "");

        assert_eq!(tab.title(), "");
    }

    #[test]
    fn test_title_empty_name_modified() {
        let mut tab = EditorTabState::new(0, "");
        tab.mark_modified();

        assert_eq!(tab.title(), "*");
    }

    #[test]
    fn test_title_unicode() {
        let tab = EditorTabState::new(0, "日本語.md");

        assert_eq!(tab.title(), "日本語.md");
    }

    #[test]
    fn test_title_unicode_modified() {
        let mut tab = EditorTabState::new(0, "日本語.md");
        tab.mark_modified();

        assert_eq!(tab.title(), "日本語.md*");
    }
}

// ============================================
// === Modification State Tests ===
// ============================================

mod modification_tests {
    use super::*;

    #[test]
    fn test_initial_not_modified() {
        let tab = EditorTabState::new(0, "test.md");

        assert!(!tab.is_modified);
    }

    #[test]
    fn test_mark_modified() {
        let mut tab = EditorTabState::new(0, "test.md");
        tab.mark_modified();

        assert!(tab.is_modified);
    }

    #[test]
    fn test_mark_saved() {
        let mut tab = EditorTabState::new(0, "test.md");
        tab.mark_modified();
        tab.mark_saved();

        assert!(!tab.is_modified);
    }

    #[test]
    fn test_mark_modified_multiple_times() {
        let mut tab = EditorTabState::new(0, "test.md");

        tab.mark_modified();
        tab.mark_modified();
        tab.mark_modified();

        assert!(tab.is_modified);
    }

    #[test]
    fn test_mark_saved_when_not_modified() {
        let mut tab = EditorTabState::new(0, "test.md");

        tab.mark_saved();

        assert!(!tab.is_modified);
    }

    #[test]
    fn test_modification_cycle() {
        let mut tab = EditorTabState::new(0, "test.md");

        assert!(!tab.is_modified);

        tab.mark_modified();
        assert!(tab.is_modified);

        tab.mark_saved();
        assert!(!tab.is_modified);

        tab.mark_modified();
        assert!(tab.is_modified);

        tab.mark_saved();
        assert!(!tab.is_modified);
    }
}

// ============================================
// === Clone Tests ===
// ============================================

mod clone_tests {
    use super::*;

    #[test]
    fn test_clone_tab() {
        let tab = EditorTabState::new(0, "test.md");
        let cloned = tab.clone();

        assert_eq!(tab.id, cloned.id);
        assert_eq!(tab.file_name, cloned.file_name);
        assert_eq!(tab.is_modified, cloned.is_modified);
    }

    #[test]
    fn test_clone_modified_tab() {
        let mut tab = EditorTabState::new(0, "test.md");
        tab.mark_modified();

        let cloned = tab.clone();

        assert!(cloned.is_modified);
    }

    #[test]
    fn test_clone_independence() {
        let tab = EditorTabState::new(0, "test.md");
        let mut cloned = tab.clone();

        cloned.mark_modified();

        assert!(!tab.is_modified);
        assert!(cloned.is_modified);
    }
}

// ============================================
// === Debug Trait Tests ===
// ============================================

mod debug_tests {
    use super::*;

    #[test]
    fn test_tab_debug() {
        let tab = EditorTabState::new(0, "test.md");
        let debug_str = format!("{:?}", tab);

        assert!(debug_str.contains("EditorTabState"));
        assert!(debug_str.contains("test.md"));
    }
}

// ============================================
// === Engine Tests ===
// ============================================

mod engine_tests {
    use super::*;

    #[test]
    fn test_tab_has_engine() {
        let tab = EditorTabState::new(0, "test.md");

        // Engine should be initialized
        assert_eq!(tab.engine.id, 0);
    }

    #[test]
    fn test_engine_id_matches_tab_id() {
        let tab = EditorTabState::new(42, "test.md");

        assert_eq!(tab.engine.id, 42);
    }
}

// ============================================
// === Edge Cases ===
// ============================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_very_long_filename() {
        let long_name = "a".repeat(1000) + ".md";
        let tab = EditorTabState::new(0, &long_name);

        assert_eq!(tab.file_name, long_name);
    }

    #[test]
    fn test_filename_with_asterisk() {
        let tab = EditorTabState::new(0, "file*.md");

        assert_eq!(tab.title(), "file*.md");
    }

    #[test]
    fn test_filename_with_asterisk_modified() {
        let mut tab = EditorTabState::new(0, "file*.md");
        tab.mark_modified();

        assert_eq!(tab.title(), "file*.md*");
    }

    #[test]
    fn test_large_id() {
        let tab = EditorTabState::new(usize::MAX, "test.md");

        assert_eq!(tab.id, usize::MAX);
    }

    #[test]
    fn test_emoji_in_filename() {
        let tab = EditorTabState::new(0, "readme_📝.md");

        assert_eq!(tab.file_name, "readme_📝.md");
    }
}
