//! テキストエディタのコアエンジン
//!
//! このモジュールは、テキストエディタの中核となるデータ構造と編集操作を提供します。
//!
//! # 使用例
//!
//! ```no_run
//! use lala::core::{EditorCore, CoreResult};
//!
//! #[tokio::main]
//! async fn main() -> CoreResult<()> {
//!     // 新しいエディタコアを作成
//!     let mut editor = EditorCore::new();
//!
//!     // ファイルを読み込む
//!     editor.load_file("sample.txt").await?;
//!
//!     // 文字を挿入
//!     editor.insert(0, "Hello, ")?;
//!
//!     // 範囲を削除
//!     editor.delete(0, 7)?;
//!
//!     // Undo/Redo
//!     editor.undo()?;
//!     editor.redo()?;
//!
//!     // ファイルに保存
//!     editor.save_file("output.txt").await?;
//!
//!     Ok(())
//! }
//! ```

mod error;
#[cfg(test)]
mod tests;

pub use error::{CoreError, CoreResult};

use ropey::Rope;
use std::path::Path;
use tokio::fs;

/// 編集操作の種類
#[derive(Debug, Clone)]
enum Operation {
    /// 挿入操作 (位置, 挿入されたテキスト)
    Insert(usize, String),
    /// 削除操作 (位置, 削除されたテキスト)
    Delete(usize, String),
}

/// テキストエディタのコアエンジン
///
/// このエンジンは、テキストバッファの管理、編集操作、履歴管理を行います。
pub struct EditorCore {
    /// テキストバッファ（Ropeデータ構造）
    buffer: Rope,
    /// Undo用の操作履歴
    undo_stack: Vec<Operation>,
    /// Redo用の操作履歴
    redo_stack: Vec<Operation>,
}

impl EditorCore {
    /// 新しいエディタコアを作成する
    ///
    /// # 例
    ///
    /// ```
    /// use lala::core::EditorCore;
    ///
    /// let editor = EditorCore::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            buffer: Rope::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// 指定されたテキストで初期化する
    ///
    /// # 例
    ///
    /// ```
    /// use lala::core::EditorCore;
    ///
    /// let editor = EditorCore::from_text("Hello, World!");
    /// ```
    #[must_use]
    pub fn from_text(text: &str) -> Self {
        Self {
            buffer: Rope::from_str(text),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// ファイルを非同期で読み込む
    ///
    /// # Errors
    ///
    /// ファイルが存在しない、または読み込み権限がない場合は`CoreError::IoError`を返す
    pub async fn load_file<P: AsRef<Path>>(&mut self, path: P) -> CoreResult<()> {
        let content = fs::read_to_string(path).await?;
        self.buffer = Rope::from_str(&content);
        // ファイルを読み込んだ時点で履歴をクリア
        self.undo_stack.clear();
        self.redo_stack.clear();
        Ok(())
    }

    /// ファイルに非同期で保存する
    ///
    /// # Errors
    ///
    /// ファイルへの書き込み権限がない場合は`CoreError::IoError`を返す
    pub async fn save_file<P: AsRef<Path>>(&self, path: P) -> CoreResult<()> {
        let content = self.buffer.to_string();
        fs::write(path, content).await?;
        Ok(())
    }

    /// 指定位置に文字列を挿入する
    ///
    /// # 引数
    ///
    /// * `index` - 挿入位置（文字インデックス）
    /// * `text` - 挿入するテキスト
    ///
    /// # Errors
    ///
    /// インデックスがバッファの範囲外の場合は`CoreError::OutOfBounds`を返す
    pub fn insert(&mut self, index: usize, text: &str) -> CoreResult<()> {
        // 範囲チェック
        let buffer_len = self.buffer.len_chars();
        if index > buffer_len {
            return Err(CoreError::OutOfBounds { index, buffer_len });
        }

        // バッファに挿入
        self.buffer.insert(index, text);

        // 履歴に記録
        self.undo_stack.push(Operation::Insert(index, text.to_string()));
        // 新しい操作を行ったのでRedoスタックをクリア
        self.redo_stack.clear();

        Ok(())
    }

    /// 指定範囲の文字列を削除する
    ///
    /// # 引数
    ///
    /// * `start` - 削除開始位置（文字インデックス）
    /// * `end` - 削除終了位置（文字インデックス、この位置は含まれない）
    ///
    /// # Errors
    ///
    /// インデックスがバッファの範囲外の場合、または start > end の場合は`CoreError::OutOfBounds`を返す
    pub fn delete(&mut self, start: usize, end: usize) -> CoreResult<()> {
        let buffer_len = self.buffer.len_chars();

        // 範囲チェック
        if start > buffer_len || end > buffer_len {
            return Err(CoreError::OutOfBounds {
                index: start.max(end),
                buffer_len,
            });
        }

        if start > end {
            return Err(CoreError::OutOfBounds {
                index: start,
                buffer_len: end,
            });
        }

        // 削除するテキストを保存（Undo用）
        let deleted_text = self.buffer.slice(start..end).to_string();

        // バッファから削除
        self.buffer.remove(start..end);

        // 履歴に記録
        self.undo_stack.push(Operation::Delete(start, deleted_text));
        // 新しい操作を行ったのでRedoスタックをクリア
        self.redo_stack.clear();

        Ok(())
    }

    /// 最後の操作を元に戻す
    ///
    /// # Errors
    ///
    /// Undo履歴がない場合は`CoreError::HistoryError`を返す
    pub fn undo(&mut self) -> CoreResult<()> {
        let operation = self.undo_stack.pop().ok_or_else(|| {
            CoreError::HistoryError("Undo履歴がありません".to_string())
        })?;

        match operation {
            Operation::Insert(index, text) => {
                // 挿入を元に戻す = 削除する
                let end = index + text.chars().count();
                self.buffer.remove(index..end);
                // Redoスタックに記録
                self.redo_stack.push(Operation::Insert(index, text));
            }
            Operation::Delete(index, text) => {
                // 削除を元に戻す = 挿入する
                self.buffer.insert(index, &text);
                // Redoスタックに記録
                self.redo_stack.push(Operation::Delete(index, text));
            }
        }

        Ok(())
    }

    /// Undoした操作をやり直す
    ///
    /// # Errors
    ///
    /// Redo履歴がない場合は`CoreError::HistoryError`を返す
    pub fn redo(&mut self) -> CoreResult<()> {
        let operation = self.redo_stack.pop().ok_or_else(|| {
            CoreError::HistoryError("Redo履歴がありません".to_string())
        })?;

        match operation {
            Operation::Insert(index, text) => {
                // 挿入をやり直す
                self.buffer.insert(index, &text);
                // Undoスタックに記録
                self.undo_stack.push(Operation::Insert(index, text));
            }
            Operation::Delete(index, text) => {
                // 削除をやり直す
                let end = index + text.chars().count();
                self.buffer.remove(index..end);
                // Undoスタックに記録
                self.undo_stack.push(Operation::Delete(index, text));
            }
        }

        Ok(())
    }

    /// バッファの文字数を取得する
    #[must_use]
    pub fn len(&self) -> usize {
        self.buffer.len_chars()
    }

    /// バッファが空かどうかを判定する
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buffer.len_chars() == 0
    }

    /// バッファの内容を文字列として取得する
    ///
    /// # 注意
    ///
    /// 大きなバッファの場合、メモリコピーが発生するため、パフォーマンスに影響する可能性があります。
    /// 可能な限り、このメソッドの使用を避け、イテレータを使用することを推奨します。
    #[must_use]
    pub fn as_str(&self) -> String {
        self.buffer.to_string()
    }
}

impl Default for EditorCore {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EditorCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buffer)
    }
}
