//! テストモジュール

use super::*;
use tokio;

// ========================================
// ユニットテスト（関数/メソッドレベル）
// ========================================

#[test]
fn test_new_editor_is_empty() {
    let editor = EditorCore::new();
    assert!(editor.is_empty());
    assert_eq!(editor.len(), 0);
}

#[test]
fn test_from_text() {
    let editor = EditorCore::from_text("Hello, World!");
    assert!(!editor.is_empty());
    assert_eq!(editor.len(), 13);
    assert_eq!(editor.to_string(), "Hello, World!");
}

#[test]
fn test_insert_at_beginning() {
    let mut editor = EditorCore::from_text("World!");
    editor.insert(0, "Hello, ").unwrap();
    assert_eq!(editor.to_string(), "Hello, World!");
}

#[test]
fn test_insert_at_middle() {
    let mut editor = EditorCore::from_text("Hello World!");
    editor.insert(5, ",").unwrap();
    assert_eq!(editor.to_string(), "Hello, World!");
}

#[test]
fn test_insert_at_end() {
    let mut editor = EditorCore::from_text("Hello");
    editor.insert(5, ", World!").unwrap();
    assert_eq!(editor.to_string(), "Hello, World!");
}

#[test]
fn test_insert_out_of_bounds() {
    let mut editor = EditorCore::from_text("Hello");
    let result = editor.insert(10, "!");
    assert!(result.is_err());
    match result {
        Err(CoreError::OutOfBounds { index, buffer_len }) => {
            assert_eq!(index, 10);
            assert_eq!(buffer_len, 5);
        }
        _ => panic!("Expected OutOfBounds error"),
    }
}

#[test]
fn test_delete_range() {
    let mut editor = EditorCore::from_text("Hello, World!");
    editor.delete(5, 7).unwrap();
    assert_eq!(editor.to_string(), "HelloWorld!");
}

#[test]
fn test_delete_at_beginning() {
    let mut editor = EditorCore::from_text("Hello, World!");
    editor.delete(0, 7).unwrap();
    assert_eq!(editor.to_string(), "World!");
}

#[test]
fn test_delete_at_end() {
    let mut editor = EditorCore::from_text("Hello, World!");
    editor.delete(7, 13).unwrap();
    assert_eq!(editor.to_string(), "Hello, ");
}

#[test]
fn test_delete_all() {
    let mut editor = EditorCore::from_text("Hello, World!");
    editor.delete(0, 13).unwrap();
    assert_eq!(editor.to_string(), "");
    assert!(editor.is_empty());
}

#[test]
fn test_delete_out_of_bounds() {
    let mut editor = EditorCore::from_text("Hello");
    let result = editor.delete(3, 10);
    assert!(result.is_err());
}

#[test]
fn test_delete_invalid_range() {
    let mut editor = EditorCore::from_text("Hello");
    let result = editor.delete(3, 1);
    assert!(result.is_err());
}

#[test]
fn test_undo_insert() {
    let mut editor = EditorCore::from_text("Hello");
    editor.insert(5, ", World!").unwrap();
    assert_eq!(editor.to_string(), "Hello, World!");

    editor.undo().unwrap();
    assert_eq!(editor.to_string(), "Hello");
}

#[test]
fn test_undo_delete() {
    let mut editor = EditorCore::from_text("Hello, World!");
    editor.delete(5, 7).unwrap();
    assert_eq!(editor.to_string(), "HelloWorld!");

    editor.undo().unwrap();
    assert_eq!(editor.to_string(), "Hello, World!");
}

#[test]
fn test_redo_insert() {
    let mut editor = EditorCore::from_text("Hello");
    editor.insert(5, ", World!").unwrap();
    editor.undo().unwrap();
    assert_eq!(editor.to_string(), "Hello");

    editor.redo().unwrap();
    assert_eq!(editor.to_string(), "Hello, World!");
}

#[test]
fn test_redo_delete() {
    let mut editor = EditorCore::from_text("Hello, World!");
    editor.delete(5, 7).unwrap();
    editor.undo().unwrap();
    assert_eq!(editor.to_string(), "Hello, World!");

    editor.redo().unwrap();
    assert_eq!(editor.to_string(), "HelloWorld!");
}

#[test]
fn test_multiple_undo_redo() {
    let mut editor = EditorCore::from_text("Hello");

    // 操作1: 挿入
    editor.insert(5, " World").unwrap();
    assert_eq!(editor.to_string(), "Hello World");

    // 操作2: 挿入
    editor.insert(11, "!").unwrap();
    assert_eq!(editor.to_string(), "Hello World!");

    // 操作3: 削除
    editor.delete(5, 6).unwrap();
    assert_eq!(editor.to_string(), "HelloWorld!");

    // Undo x3
    editor.undo().unwrap();
    assert_eq!(editor.to_string(), "Hello World!");

    editor.undo().unwrap();
    assert_eq!(editor.to_string(), "Hello World");

    editor.undo().unwrap();
    assert_eq!(editor.to_string(), "Hello");

    // Redo x3
    editor.redo().unwrap();
    assert_eq!(editor.to_string(), "Hello World");

    editor.redo().unwrap();
    assert_eq!(editor.to_string(), "Hello World!");

    editor.redo().unwrap();
    assert_eq!(editor.to_string(), "HelloWorld!");
}

#[test]
fn test_undo_without_history() {
    let mut editor = EditorCore::new();
    let result = editor.undo();
    assert!(result.is_err());
    match result {
        Err(CoreError::HistoryError(_)) => {},
        _ => panic!("Expected HistoryError"),
    }
}

#[test]
fn test_redo_without_history() {
    let mut editor = EditorCore::new();
    let result = editor.redo();
    assert!(result.is_err());
    match result {
        Err(CoreError::HistoryError(_)) => {},
        _ => panic!("Expected HistoryError"),
    }
}

#[test]
fn test_new_operation_clears_redo_stack() {
    let mut editor = EditorCore::from_text("Hello");

    // 操作1
    editor.insert(5, " World").unwrap();
    // Undo
    editor.undo().unwrap();
    assert_eq!(editor.to_string(), "Hello");

    // 新しい操作
    editor.insert(5, "!").unwrap();
    assert_eq!(editor.to_string(), "Hello!");

    // Redoは失敗するはず（Redoスタックがクリアされている）
    let result = editor.redo();
    assert!(result.is_err());
}

// ========================================
// エッジケースのテスト
// ========================================

#[test]
fn test_insert_into_empty_buffer() {
    let mut editor = EditorCore::new();
    editor.insert(0, "First text").unwrap();
    assert_eq!(editor.to_string(), "First text");
}

#[test]
fn test_delete_empty_range() {
    let mut editor = EditorCore::from_text("Hello");
    editor.delete(2, 2).unwrap();
    assert_eq!(editor.to_string(), "Hello");
}

#[test]
fn test_multibyte_characters() {
    let mut editor = EditorCore::from_text("こんにちは");
    assert_eq!(editor.len(), 5);

    editor.insert(5, "、世界！").unwrap();
    assert_eq!(editor.to_string(), "こんにちは、世界！");
    assert_eq!(editor.len(), 9);

    editor.delete(5, 6).unwrap();
    assert_eq!(editor.to_string(), "こんにちは世界！");
    assert_eq!(editor.len(), 8);
}

// ========================================
// 統合テスト（非同期ファイルI/O）
// ========================================

#[tokio::test]
async fn test_load_and_save_file() {
    use std::fs;
    use std::io::Write;

    let test_file = "/tmp/test_load_save.txt";
    let test_content = "Hello, Rust!";

    // テストファイルを作成
    {
        let mut file = fs::File::create(test_file).unwrap();
        file.write_all(test_content.as_bytes()).unwrap();
    }

    // ファイルを読み込む
    let mut editor = EditorCore::new();
    editor.load_file(test_file).await.unwrap();
    assert_eq!(editor.to_string(), test_content);

    // 編集する
    editor.insert(0, "Test: ").unwrap();

    // 保存する
    let output_file = "/tmp/test_output.txt";
    editor.save_file(output_file).await.unwrap();

    // 保存されたファイルを確認
    let saved_content = fs::read_to_string(output_file).unwrap();
    assert_eq!(saved_content, "Test: Hello, Rust!");

    // クリーンアップ
    fs::remove_file(test_file).ok();
    fs::remove_file(output_file).ok();
}

#[tokio::test]
async fn test_load_nonexistent_file() {
    let mut editor = EditorCore::new();
    let result = editor.load_file("/tmp/nonexistent_file_xyz.txt").await;
    assert!(result.is_err());
    match result {
        Err(CoreError::IoError(_)) => {},
        _ => panic!("Expected IoError"),
    }
}

#[tokio::test]
async fn test_save_to_invalid_path() {
    let editor = EditorCore::from_text("Test");
    // 書き込み権限がないパス（存在しないディレクトリ）
    let result = editor.save_file("/invalid/path/that/does/not/exist/file.txt").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_load_file_clears_history() {
    use std::fs;
    use std::io::Write;

    let test_file = "/tmp/test_history_clear.txt";

    // テストファイルを作成
    {
        let mut file = fs::File::create(test_file).unwrap();
        file.write_all(b"Initial content").unwrap();
    }

    let mut editor = EditorCore::new();

    // 何か操作を行う
    editor.insert(0, "Before load").unwrap();

    // ファイルを読み込むと履歴がクリアされる
    editor.load_file(test_file).await.unwrap();

    // Undoは失敗するはず
    let result = editor.undo();
    assert!(result.is_err());

    // クリーンアップ
    fs::remove_file(test_file).ok();
}

#[tokio::test]
async fn test_full_workflow() {
    use std::fs;
    use std::io::Write;

    let input_file = "/tmp/test_workflow_input.txt";
    let output_file = "/tmp/test_workflow_output.txt";

    // テストファイルを作成
    {
        let mut file = fs::File::create(input_file).unwrap();
        file.write_all(b"The quick brown fox").unwrap();
    }

    let mut editor = EditorCore::new();

    // ファイルを読み込む
    editor.load_file(input_file).await.unwrap();
    assert_eq!(editor.to_string(), "The quick brown fox");

    // 編集: 文字を挿入
    editor.insert(19, " jumps over the lazy dog").unwrap();
    assert_eq!(editor.to_string(), "The quick brown fox jumps over the lazy dog");

    // 編集: 範囲を削除
    editor.delete(4, 10).unwrap();
    assert_eq!(editor.to_string(), "The brown fox jumps over the lazy dog");

    // Undo
    editor.undo().unwrap();
    assert_eq!(editor.to_string(), "The quick brown fox jumps over the lazy dog");

    // Redo
    editor.redo().unwrap();
    assert_eq!(editor.to_string(), "The brown fox jumps over the lazy dog");

    // ファイルに保存
    editor.save_file(output_file).await.unwrap();

    // 保存されたファイルを確認
    let saved_content = fs::read_to_string(output_file).unwrap();
    assert_eq!(saved_content, "The brown fox jumps over the lazy dog");

    // クリーンアップ
    fs::remove_file(input_file).ok();
    fs::remove_file(output_file).ok();
}
