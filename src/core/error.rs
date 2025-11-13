//! エラー型の定義

use thiserror::Error;

/// コアエンジンのエラー型
#[derive(Error, Debug)]
pub enum CoreError {
    /// ファイルI/Oエラー
    #[error("ファイルI/Oエラー: {0}")]
    IoError(#[from] std::io::Error),

    /// バッファ範囲外エラー
    #[error("バッファ範囲外アクセス: index={index}, buffer_len={buffer_len}")]
    OutOfBounds { index: usize, buffer_len: usize },

    /// Undo/Redo操作が不可能
    #[error("Undo/Redo操作が不可能: {0}")]
    HistoryError(String),

    /// UTF-8変換エラー
    #[error("UTF-8変換エラー: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

/// コアエンジンの結果型
pub type CoreResult<T> = Result<T, CoreError>;
