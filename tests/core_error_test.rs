/// Comprehensive tests for CoreError
///
/// Tests cover:
/// - Error creation
/// - Error display messages
/// - Error conversion (From traits)
/// - Error type matching
use lala::core::CoreError;
use std::io;

// ============================================
// === IO Error Tests ===
// ============================================

mod io_error_tests {
    use super::*;

    #[test]
    fn test_io_error_from() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let core_err: CoreError = io_err.into();

        let display = format!("{}", core_err);
        assert!(display.contains("ファイルI/Oエラー"));
    }

    #[test]
    fn test_io_error_permission_denied() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let core_err: CoreError = io_err.into();

        let display = format!("{}", core_err);
        assert!(display.contains("ファイルI/Oエラー"));
    }

    #[test]
    fn test_io_error_various_kinds() {
        let error_kinds = vec![
            io::ErrorKind::NotFound,
            io::ErrorKind::PermissionDenied,
            io::ErrorKind::ConnectionRefused,
            io::ErrorKind::ConnectionReset,
            io::ErrorKind::ConnectionAborted,
            io::ErrorKind::NotConnected,
            io::ErrorKind::AddrInUse,
            io::ErrorKind::BrokenPipe,
            io::ErrorKind::AlreadyExists,
            io::ErrorKind::WouldBlock,
            io::ErrorKind::InvalidInput,
            io::ErrorKind::InvalidData,
            io::ErrorKind::TimedOut,
            io::ErrorKind::WriteZero,
            io::ErrorKind::Interrupted,
            io::ErrorKind::UnexpectedEof,
        ];

        for kind in error_kinds {
            let io_err = io::Error::new(kind, "test error");
            let core_err: CoreError = io_err.into();
            let display = format!("{}", core_err);
            assert!(display.contains("ファイルI/Oエラー"));
        }
    }
}

// ============================================
// === Out Of Bounds Error Tests ===
// ============================================

mod out_of_bounds_tests {
    use super::*;

    #[test]
    fn test_out_of_bounds_basic() {
        let err = CoreError::OutOfBounds {
            index: 10,
            buffer_len: 5,
        };

        let display = format!("{}", err);
        assert!(display.contains("バッファ範囲外アクセス"));
        assert!(display.contains("index=10"));
        assert!(display.contains("buffer_len=5"));
    }

    #[test]
    fn test_out_of_bounds_zero_length() {
        let err = CoreError::OutOfBounds {
            index: 0,
            buffer_len: 0,
        };

        let display = format!("{}", err);
        assert!(display.contains("index=0"));
        assert!(display.contains("buffer_len=0"));
    }

    #[test]
    fn test_out_of_bounds_large_values() {
        let err = CoreError::OutOfBounds {
            index: usize::MAX,
            buffer_len: usize::MAX - 1,
        };

        let display = format!("{}", err);
        assert!(display.contains("バッファ範囲外アクセス"));
    }

    #[test]
    fn test_out_of_bounds_debug() {
        let err = CoreError::OutOfBounds {
            index: 5,
            buffer_len: 3,
        };

        let debug = format!("{:?}", err);
        assert!(debug.contains("OutOfBounds"));
    }
}

// ============================================
// === History Error Tests ===
// ============================================

mod history_error_tests {
    use super::*;

    #[test]
    fn test_history_error_undo() {
        let err = CoreError::HistoryError("Undo操作が不可能".to_string());

        let display = format!("{}", err);
        assert!(display.contains("Undo/Redo操作が不可能"));
        assert!(display.contains("Undo操作が不可能"));
    }

    #[test]
    fn test_history_error_redo() {
        let err = CoreError::HistoryError("Redo操作が不可能".to_string());

        let display = format!("{}", err);
        assert!(display.contains("Redo操作が不可能"));
    }

    #[test]
    fn test_history_error_empty_message() {
        let err = CoreError::HistoryError(String::new());

        let display = format!("{}", err);
        assert!(display.contains("Undo/Redo操作が不可能"));
    }

    #[test]
    fn test_history_error_unicode_message() {
        let err = CoreError::HistoryError("履歴が空です".to_string());

        let display = format!("{}", err);
        assert!(display.contains("履歴が空です"));
    }

    #[test]
    fn test_history_error_debug() {
        let err = CoreError::HistoryError("test message".to_string());

        let debug = format!("{:?}", err);
        assert!(debug.contains("HistoryError"));
        assert!(debug.contains("test message"));
    }
}

// ============================================
// === UTF-8 Error Tests ===
// ============================================

mod utf8_error_tests {
    use super::*;

    #[test]
    fn test_utf8_error_from() {
        // Create an invalid UTF-8 sequence
        let invalid_utf8 = vec![0xff, 0xfe, 0xfd];
        let utf8_err = String::from_utf8(invalid_utf8).unwrap_err();
        let core_err: CoreError = utf8_err.into();

        let display = format!("{}", core_err);
        assert!(display.contains("UTF-8変換エラー"));
    }

    #[test]
    fn test_utf8_error_debug() {
        let invalid_utf8 = vec![0x80, 0x81, 0x82];
        let utf8_err = String::from_utf8(invalid_utf8).unwrap_err();
        let core_err: CoreError = utf8_err.into();

        let debug = format!("{:?}", core_err);
        assert!(debug.contains("Utf8Error"));
    }
}

// ============================================
// === Error Trait Tests ===
// ============================================

mod error_trait_tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_io_error_is_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "test");
        let core_err: CoreError = io_err.into();

        // CoreError implements std::error::Error
        let _: &dyn Error = &core_err;
    }

    #[test]
    fn test_out_of_bounds_is_error() {
        let err = CoreError::OutOfBounds {
            index: 10,
            buffer_len: 5,
        };

        let _: &dyn Error = &err;
    }

    #[test]
    fn test_history_error_is_error() {
        let err = CoreError::HistoryError("test".to_string());

        let _: &dyn Error = &err;
    }

    #[test]
    fn test_error_source_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "original error");
        let core_err: CoreError = io_err.into();

        // The source should be the original io::Error
        assert!(core_err.source().is_some());
    }

    #[test]
    fn test_error_source_out_of_bounds() {
        let err = CoreError::OutOfBounds {
            index: 10,
            buffer_len: 5,
        };

        // OutOfBounds has no source
        assert!(err.source().is_none());
    }

    #[test]
    fn test_error_source_history() {
        let err = CoreError::HistoryError("test".to_string());

        // HistoryError has no source
        assert!(err.source().is_none());
    }
}

// ============================================
// === Pattern Matching Tests ===
// ============================================

mod pattern_matching_tests {
    use super::*;

    #[test]
    fn test_match_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "test");
        let core_err: CoreError = io_err.into();

        match core_err {
            CoreError::IoError(_) => (),
            _ => panic!("Expected IoError variant"),
        }
    }

    #[test]
    fn test_match_out_of_bounds() {
        let err = CoreError::OutOfBounds {
            index: 10,
            buffer_len: 5,
        };

        match err {
            CoreError::OutOfBounds { index, buffer_len } => {
                assert_eq!(index, 10);
                assert_eq!(buffer_len, 5);
            }
            _ => panic!("Expected OutOfBounds variant"),
        }
    }

    #[test]
    fn test_match_history_error() {
        let err = CoreError::HistoryError("test message".to_string());

        match err {
            CoreError::HistoryError(msg) => {
                assert_eq!(msg, "test message");
            }
            _ => panic!("Expected HistoryError variant"),
        }
    }

    #[test]
    fn test_match_utf8_error() {
        let invalid_utf8 = vec![0xff, 0xfe];
        let utf8_err = String::from_utf8(invalid_utf8).unwrap_err();
        let core_err: CoreError = utf8_err.into();

        match core_err {
            CoreError::Utf8Error(_) => (),
            _ => panic!("Expected Utf8Error variant"),
        }
    }
}

// ============================================
// === Edge Cases ===
// ============================================

mod edge_cases {
    use super::*;

    #[test]
    fn test_out_of_bounds_index_equals_len() {
        let err = CoreError::OutOfBounds {
            index: 5,
            buffer_len: 5,
        };

        let display = format!("{}", err);
        assert!(display.contains("index=5"));
        assert!(display.contains("buffer_len=5"));
    }

    #[test]
    fn test_history_error_with_newlines() {
        let err = CoreError::HistoryError("Error\nwith\nnewlines".to_string());

        let display = format!("{}", err);
        assert!(display.contains("newlines"));
    }

    #[test]
    fn test_history_error_with_special_chars() {
        let err = CoreError::HistoryError("Error: <test> & \"quotes\"".to_string());

        let display = format!("{}", err);
        assert!(display.contains("<test>"));
        assert!(display.contains("&"));
    }
}
