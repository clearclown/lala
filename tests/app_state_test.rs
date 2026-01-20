/// Comprehensive tests for AppState (tab management)
///
/// Tests cover:
/// - Tab creation and management
/// - Active tab handling
/// - Tab closing edge cases
/// - Multiple tab operations
/// - State consistency
use lala::gui::AppState;

// ============================================
// === Basic AppState Tests ===
// ============================================

mod basic_tests {
    use super::*;

    #[test]
    fn test_new_state_is_empty() {
        let state = AppState::new();
        assert_eq!(state.tab_count(), 0);
        assert!(state.is_empty());
        assert!(state.active_tab().is_none());
        assert!(state.active_tab_index().is_none());
    }

    #[test]
    fn test_default_state() {
        let state = AppState::default();
        assert_eq!(state.tab_count(), 0);
        assert!(state.is_empty());
    }

    #[test]
    fn test_tabs_returns_empty_slice() {
        let state = AppState::new();
        assert!(state.tabs().is_empty());
    }
}

// ============================================
// === Tab Opening Tests ===
// ============================================

mod tab_opening_tests {
    use super::*;

    #[test]
    fn test_open_first_tab() {
        let mut state = AppState::new();
        let id = state.open_new_tab("test.md");

        assert_eq!(id, 0);
        assert_eq!(state.tab_count(), 1);
        assert!(!state.is_empty());
        assert_eq!(state.active_tab_index(), Some(0));
    }

    #[test]
    fn test_open_multiple_tabs() {
        let mut state = AppState::new();

        let id1 = state.open_new_tab("first.md");
        let id2 = state.open_new_tab("second.md");
        let id3 = state.open_new_tab("third.md");

        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(id3, 2);
        assert_eq!(state.tab_count(), 3);
    }

    #[test]
    fn test_first_tab_becomes_active() {
        let mut state = AppState::new();
        state.open_new_tab("test.md");

        // First tab should automatically become active
        assert_eq!(state.active_tab_index(), Some(0));
        assert!(state.active_tab().is_some());
    }

    #[test]
    fn test_subsequent_tabs_dont_change_active() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");
        state.open_new_tab("third.md");

        // First tab should still be active
        assert_eq!(state.active_tab_index(), Some(0));
        assert_eq!(state.active_tab().unwrap().file_name, "first.md");
    }

    #[test]
    fn test_tab_ids_increment() {
        let mut state = AppState::new();

        for i in 0..10 {
            let id = state.open_new_tab(format!("file{}.md", i));
            assert_eq!(id, i);
        }
    }

    #[test]
    fn test_open_tab_with_string() {
        let mut state = AppState::new();
        let name = String::from("owned_string.md");
        state.open_new_tab(name);

        assert_eq!(state.tabs()[0].file_name, "owned_string.md");
    }

    #[test]
    fn test_open_tab_with_str_slice() {
        let mut state = AppState::new();
        state.open_new_tab("str_slice.md");

        assert_eq!(state.tabs()[0].file_name, "str_slice.md");
    }

    #[test]
    fn test_open_tab_with_empty_name() {
        let mut state = AppState::new();
        state.open_new_tab("");

        assert_eq!(state.tabs()[0].file_name, "");
    }

    #[test]
    fn test_open_tab_with_unicode_name() {
        let mut state = AppState::new();
        state.open_new_tab("日本語ファイル.md");

        assert_eq!(state.tabs()[0].file_name, "日本語ファイル.md");
    }

    #[test]
    fn test_open_tab_with_special_characters() {
        let mut state = AppState::new();
        state.open_new_tab("file with spaces.md");
        state.open_new_tab("file-with-dashes.md");
        state.open_new_tab("file_with_underscores.md");

        assert_eq!(state.tab_count(), 3);
    }
}

// ============================================
// === Active Tab Tests ===
// ============================================

mod active_tab_tests {
    use super::*;

    #[test]
    fn test_set_active_tab() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");
        state.open_new_tab("third.md");

        assert!(state.set_active_tab(1));
        assert_eq!(state.active_tab_index(), Some(1));
        assert_eq!(state.active_tab().unwrap().file_name, "second.md");
    }

    #[test]
    fn test_set_active_tab_first() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");

        state.set_active_tab(1);
        assert!(state.set_active_tab(0));
        assert_eq!(state.active_tab_index(), Some(0));
    }

    #[test]
    fn test_set_active_tab_last() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");
        state.open_new_tab("third.md");

        assert!(state.set_active_tab(2));
        assert_eq!(state.active_tab_index(), Some(2));
    }

    #[test]
    fn test_set_active_tab_invalid_index() {
        let mut state = AppState::new();
        state.open_new_tab("test.md");

        assert!(!state.set_active_tab(5));
        assert_eq!(state.active_tab_index(), Some(0)); // Unchanged
    }

    #[test]
    fn test_set_active_tab_on_empty_state() {
        let mut state = AppState::new();

        assert!(!state.set_active_tab(0));
        assert!(state.active_tab_index().is_none());
    }

    #[test]
    fn test_active_tab_returns_correct_tab() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");

        state.set_active_tab(1);
        let tab = state.active_tab().unwrap();
        assert_eq!(tab.file_name, "second.md");
        assert_eq!(tab.id, 1);
    }

    #[test]
    fn test_active_tab_mut_allows_modification() {
        let mut state = AppState::new();
        state.open_new_tab("test.md");

        if let Some(tab) = state.active_tab_mut() {
            tab.mark_modified();
        }

        assert!(state.active_tab().unwrap().is_modified);
    }

    #[test]
    fn test_active_tab_none_when_empty() {
        let state = AppState::new();
        assert!(state.active_tab().is_none());
    }
}

// ============================================
// === Tab Closing Tests ===
// ============================================

mod tab_closing_tests {
    use super::*;

    #[test]
    fn test_close_single_tab() {
        let mut state = AppState::new();
        state.open_new_tab("test.md");

        assert!(state.close_tab(0));
        assert_eq!(state.tab_count(), 0);
        assert!(state.is_empty());
        assert!(state.active_tab_index().is_none());
    }

    #[test]
    fn test_close_first_tab_of_many() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");
        state.open_new_tab("third.md");

        assert!(state.close_tab(0));
        assert_eq!(state.tab_count(), 2);
        assert_eq!(state.tabs()[0].file_name, "second.md");
        assert_eq!(state.tabs()[1].file_name, "third.md");
    }

    #[test]
    fn test_close_middle_tab() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");
        state.open_new_tab("third.md");

        assert!(state.close_tab(1));
        assert_eq!(state.tab_count(), 2);
        assert_eq!(state.tabs()[0].file_name, "first.md");
        assert_eq!(state.tabs()[1].file_name, "third.md");
    }

    #[test]
    fn test_close_last_tab_of_many() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");
        state.open_new_tab("third.md");

        assert!(state.close_tab(2));
        assert_eq!(state.tab_count(), 2);
        assert_eq!(state.tabs()[0].file_name, "first.md");
        assert_eq!(state.tabs()[1].file_name, "second.md");
    }

    #[test]
    fn test_close_invalid_index() {
        let mut state = AppState::new();
        state.open_new_tab("test.md");

        assert!(!state.close_tab(5));
        assert_eq!(state.tab_count(), 1);
    }

    #[test]
    fn test_close_empty_state() {
        let mut state = AppState::new();

        assert!(!state.close_tab(0));
        assert!(state.is_empty());
    }

    #[test]
    fn test_close_active_tab_adjusts_index() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");
        state.open_new_tab("third.md");

        state.set_active_tab(1);
        state.close_tab(1);

        // Active tab should be adjusted
        assert!(state.active_tab_index().is_some());
    }

    #[test]
    fn test_close_tab_before_active() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");
        state.open_new_tab("third.md");

        state.set_active_tab(2);
        state.close_tab(0);

        // Active index should decrease
        assert_eq!(state.active_tab_index(), Some(1));
        assert_eq!(state.active_tab().unwrap().file_name, "third.md");
    }

    #[test]
    fn test_close_tab_after_active() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");
        state.open_new_tab("third.md");

        state.set_active_tab(0);
        state.close_tab(2);

        // Active index should stay same
        assert_eq!(state.active_tab_index(), Some(0));
        assert_eq!(state.active_tab().unwrap().file_name, "first.md");
    }

    #[test]
    fn test_close_all_tabs_sequentially() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");
        state.open_new_tab("third.md");

        assert!(state.close_tab(0));
        assert!(state.close_tab(0));
        assert!(state.close_tab(0));

        assert!(state.is_empty());
        assert!(state.active_tab_index().is_none());
    }

    #[test]
    fn test_close_last_tab_when_active_is_last() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");

        state.set_active_tab(1);
        state.close_tab(1);

        // Should adjust to previous tab
        assert_eq!(state.active_tab_index(), Some(0));
    }
}

// ============================================
// === Tabs Iterator Tests ===
// ============================================

mod tabs_access_tests {
    use super::*;

    #[test]
    fn test_tabs_slice() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");

        let tabs = state.tabs();
        assert_eq!(tabs.len(), 2);
        assert_eq!(tabs[0].file_name, "first.md");
        assert_eq!(tabs[1].file_name, "second.md");
    }

    #[test]
    fn test_tabs_iteration() {
        let mut state = AppState::new();
        state.open_new_tab("a.md");
        state.open_new_tab("b.md");
        state.open_new_tab("c.md");

        let names: Vec<_> = state.tabs().iter().map(|t| t.file_name.as_str()).collect();
        assert_eq!(names, vec!["a.md", "b.md", "c.md"]);
    }

    #[test]
    fn test_tabs_find() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("target.md");
        state.open_new_tab("third.md");

        let target = state.tabs().iter().find(|t| t.file_name == "target.md");
        assert!(target.is_some());
        assert_eq!(target.unwrap().id, 1);
    }
}

// ============================================
// === State Consistency Tests ===
// ============================================

mod consistency_tests {
    use super::*;

    #[test]
    fn test_tab_count_matches_tabs_len() {
        let mut state = AppState::new();

        for i in 0..5 {
            state.open_new_tab(format!("file{}.md", i));
            assert_eq!(state.tab_count(), state.tabs().len());
        }
    }

    #[test]
    fn test_is_empty_matches_count() {
        let mut state = AppState::new();

        assert!(state.is_empty() == (state.tab_count() == 0));

        state.open_new_tab("test.md");
        assert!(state.is_empty() == (state.tab_count() == 0));
    }

    #[test]
    fn test_active_index_within_bounds() {
        let mut state = AppState::new();

        for i in 0..5 {
            state.open_new_tab(format!("file{}.md", i));

            if let Some(idx) = state.active_tab_index() {
                assert!(idx < state.tab_count());
            }
        }
    }

    #[test]
    fn test_ids_unique() {
        let mut state = AppState::new();

        for i in 0..10 {
            state.open_new_tab(format!("file{}.md", i));
        }

        let ids: Vec<_> = state.tabs().iter().map(|t| t.id).collect();
        let unique_ids: std::collections::HashSet<_> = ids.iter().collect();

        assert_eq!(ids.len(), unique_ids.len());
    }

    #[test]
    fn test_state_after_reopen() {
        let mut state = AppState::new();
        state.open_new_tab("first.md");
        state.open_new_tab("second.md");

        state.close_tab(0);

        let new_id = state.open_new_tab("third.md");

        // New tab should have unique ID
        assert_ne!(new_id, 0);
        assert_ne!(new_id, 1);
    }
}

// ============================================
// === Edge Cases and Stress Tests ===
// ============================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_many_tabs() {
        let mut state = AppState::new();

        for i in 0..100 {
            state.open_new_tab(format!("file{}.md", i));
        }

        assert_eq!(state.tab_count(), 100);
    }

    #[test]
    fn test_close_many_tabs() {
        let mut state = AppState::new();

        for i in 0..50 {
            state.open_new_tab(format!("file{}.md", i));
        }

        // Close from end
        for _ in 0..50 {
            state.close_tab(state.tab_count() - 1);
        }

        assert!(state.is_empty());
    }

    #[test]
    fn test_alternating_open_close() {
        let mut state = AppState::new();

        for i in 0..20 {
            state.open_new_tab(format!("file{}.md", i));
            if i % 2 == 0 && state.tab_count() > 0 {
                state.close_tab(0);
            }
        }

        // Should have some tabs
        assert!(state.tab_count() > 0);
    }

    #[test]
    fn test_rapid_active_switching() {
        let mut state = AppState::new();

        for i in 0..10 {
            state.open_new_tab(format!("file{}.md", i));
        }

        for i in 0..100 {
            let idx = i % state.tab_count();
            state.set_active_tab(idx);
            assert_eq!(state.active_tab_index(), Some(idx));
        }
    }

    #[test]
    fn test_very_long_filename() {
        let mut state = AppState::new();
        let long_name = "a".repeat(1000) + ".md";
        state.open_new_tab(&long_name);

        assert_eq!(state.tabs()[0].file_name, long_name);
    }
}

// ============================================
// === Debug Trait Tests ===
// ============================================

mod debug_tests {
    use super::*;

    #[test]
    fn test_app_state_debug() {
        let state = AppState::new();
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("AppState"));
    }
}
