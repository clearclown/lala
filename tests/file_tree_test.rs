/// Comprehensive tests for FileTree module
///
/// Tests cover:
/// - FileTree creation
/// - Root path management
/// - Default behavior
/// - Path manipulation
use lala::file_tree::FileTree;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

// ============================================
// === Basic Creation Tests ===
// ============================================

mod creation_tests {
    use super::*;

    #[test]
    fn test_new_with_path() {
        let tree = FileTree::new(PathBuf::from("/tmp/test"));
        assert_eq!(tree.root(), Path::new("/tmp/test"));
    }

    #[test]
    fn test_new_with_relative_path() {
        let tree = FileTree::new(PathBuf::from("relative/path"));
        assert_eq!(tree.root(), Path::new("relative/path"));
    }

    #[test]
    fn test_new_with_current_dir() {
        let tree = FileTree::new(PathBuf::from("."));
        assert_eq!(tree.root(), Path::new("."));
    }

    #[test]
    fn test_new_with_parent_dir() {
        let tree = FileTree::new(PathBuf::from(".."));
        assert_eq!(tree.root(), Path::new(".."));
    }

    #[test]
    fn test_default_uses_current_dir() {
        let tree = FileTree::default();
        // Default should use current working directory or fallback to "."
        assert!(!tree.root().as_os_str().is_empty());
    }

    #[test]
    fn test_clone() {
        let tree = FileTree::new(PathBuf::from("/some/path"));
        let cloned = tree.clone();
        assert_eq!(tree.root(), cloned.root());
    }
}

// ============================================
// === Root Path Tests ===
// ============================================

mod root_path_tests {
    use super::*;

    #[test]
    fn test_root_returns_path_reference() {
        let tree = FileTree::new(PathBuf::from("/path/to/root"));
        let root: &Path = tree.root();
        assert_eq!(root, Path::new("/path/to/root"));
    }

    #[test]
    fn test_set_root() {
        let mut tree = FileTree::new(PathBuf::from("/initial"));
        tree.set_root(PathBuf::from("/changed"));
        assert_eq!(tree.root(), Path::new("/changed"));
    }

    #[test]
    fn test_set_root_multiple_times() {
        let mut tree = FileTree::new(PathBuf::from("/first"));
        tree.set_root(PathBuf::from("/second"));
        tree.set_root(PathBuf::from("/third"));
        assert_eq!(tree.root(), Path::new("/third"));
    }

    #[test]
    fn test_root_with_temp_dir() {
        let temp_dir = TempDir::new().unwrap();
        let tree = FileTree::new(temp_dir.path().to_path_buf());
        assert_eq!(tree.root(), temp_dir.path());
    }
}

// ============================================
// === Path Types Tests ===
// ============================================

mod path_type_tests {
    use super::*;

    #[test]
    fn test_absolute_path() {
        let tree = FileTree::new(PathBuf::from("/absolute/path"));
        assert!(tree.root().is_absolute());
    }

    #[test]
    fn test_relative_path() {
        let tree = FileTree::new(PathBuf::from("relative/path"));
        assert!(tree.root().is_relative());
    }

    #[test]
    fn test_root_path() {
        let tree = FileTree::new(PathBuf::from("/"));
        assert_eq!(tree.root(), Path::new("/"));
    }

    #[test]
    fn test_empty_path() {
        let tree = FileTree::new(PathBuf::from(""));
        assert_eq!(tree.root(), Path::new(""));
    }

    #[test]
    fn test_path_with_spaces() {
        let tree = FileTree::new(PathBuf::from("/path with spaces/dir"));
        assert_eq!(tree.root(), Path::new("/path with spaces/dir"));
    }

    #[test]
    fn test_path_with_unicode() {
        let tree = FileTree::new(PathBuf::from("/日本語/パス"));
        assert_eq!(tree.root(), Path::new("/日本語/パス"));
    }
}

// ============================================
// === Debug Trait Tests ===
// ============================================

mod debug_tests {
    use super::*;

    #[test]
    fn test_file_tree_debug() {
        let tree = FileTree::new(PathBuf::from("/test/path"));
        let debug_str = format!("{:?}", tree);
        assert!(debug_str.contains("FileTree"));
        assert!(debug_str.contains("/test/path"));
    }
}

// ============================================
// === Real Directory Tests ===
// ============================================

mod real_directory_tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_with_existing_directory() {
        let temp_dir = TempDir::new().unwrap();
        let tree = FileTree::new(temp_dir.path().to_path_buf());

        assert!(tree.root().exists());
        assert!(tree.root().is_dir());
    }

    #[test]
    fn test_with_nested_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nested = temp_dir.path().join("a").join("b").join("c");
        fs::create_dir_all(&nested).unwrap();

        let tree = FileTree::new(nested.clone());
        assert_eq!(tree.root(), nested.as_path());
    }

    #[test]
    fn test_with_nonexistent_directory() {
        // FileTree doesn't validate existence
        let tree = FileTree::new(PathBuf::from("/nonexistent/path/12345"));
        assert_eq!(tree.root(), Path::new("/nonexistent/path/12345"));
    }
}

// ============================================
// === Path Operations Tests ===
// ============================================

mod path_operations_tests {
    use super::*;

    #[test]
    fn test_root_join() {
        let tree = FileTree::new(PathBuf::from("/base"));
        let joined = tree.root().join("subdir");
        assert_eq!(joined, PathBuf::from("/base/subdir"));
    }

    #[test]
    fn test_root_parent() {
        let tree = FileTree::new(PathBuf::from("/parent/child"));
        assert_eq!(tree.root().parent(), Some(Path::new("/parent")));
    }

    #[test]
    fn test_root_file_name() {
        let tree = FileTree::new(PathBuf::from("/path/to/dirname"));
        assert_eq!(tree.root().file_name().unwrap(), "dirname");
    }

    #[test]
    fn test_root_components() {
        let tree = FileTree::new(PathBuf::from("/a/b/c"));
        let components: Vec<_> = tree.root().components().collect();
        assert!(!components.is_empty());
    }
}

// ============================================
// === Edge Cases ===
// ============================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_very_long_path() {
        let long_path = "/".to_string() + &"a/".repeat(100) + "end";
        let tree = FileTree::new(PathBuf::from(&long_path));
        assert!(tree.root().to_string_lossy().len() > 200);
    }

    #[test]
    fn test_path_with_dots() {
        let tree = FileTree::new(PathBuf::from("/path/./to/../dir"));
        // Path is stored as-is (no normalization)
        assert_eq!(tree.root(), Path::new("/path/./to/../dir"));
    }

    #[test]
    fn test_path_with_special_chars() {
        let tree = FileTree::new(PathBuf::from("/path/with-dashes_and_underscores"));
        assert_eq!(tree.root(), Path::new("/path/with-dashes_and_underscores"));
    }

    #[test]
    fn test_home_tilde_literal() {
        // Tilde is not expanded by PathBuf
        let tree = FileTree::new(PathBuf::from("~/Documents"));
        assert_eq!(tree.root(), Path::new("~/Documents"));
    }
}
