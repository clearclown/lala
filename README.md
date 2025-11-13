# lala - テキストエディタ コアエンジン

高性能なテキストエディタのコアエンジンライブラリ

## 概要

`lala`は、Rustで実装された高性能テキストエディタのコアエンジンです。Ropeデータ構造を使用し、大規模ファイル（100MB以上）でもneovim並みの応答性を実現します。

## 特徴

- **高速なテキストバッファ管理**: Ropeデータ構造（`ropey`クレート）により、大規模ファイルでも高速に動作
- **非同期ファイルI/O**: `tokio`による非ブロッキングなファイル読み書き
- **完全なUndo/Redo**: 複数の編集操作を元に戻したり、やり直したりできる
- **エラーハンドリング**: Rustの`Result`型による堅牢なエラー処理
- **GUI/CUI非依存**: 純粋なライブラリとして、任意のフロントエンドから利用可能
- **マルチバイト文字対応**: UTF-8文字列を正しく扱える

## パフォーマンス

**100MBファイルでの実測値:**

| 操作 | 時間 | 要件 |
|------|------|------|
| ファイル読み込み | 358ms | < 5秒 |
| テキスト挿入 | 22μs | < 16ms |
| テキスト削除 | 6μs | < 16ms |
| Undo | 968ns | < 16ms |
| Redo | 742ns | < 16ms |
| ファイル保存 | 164ms | - |

✅ すべての操作がneovim並みの応答性要件を満たしています

## インストール

`Cargo.toml`に以下を追加:

```toml
[dependencies]
lala = "0.1.0"
ropey = "1.6"
tokio = { version = "1.35", features = ["full"] }
thiserror = "1.0"
```

## 使用例

### 基本的な使用方法

```rust
use lala::core::{EditorCore, CoreResult};

#[tokio::main]
async fn main() -> CoreResult<()> {
    // エディタを作成
    let mut editor = EditorCore::new();

    // ファイルを読み込む
    editor.load_file("sample.txt").await?;

    // テキストを編集
    editor.insert(0, "Hello, ")?;
    editor.delete(7, 12)?;

    // Undo/Redo
    editor.undo()?;
    editor.redo()?;

    // ファイルに保存
    editor.save_file("output.txt").await?;

    Ok(())
}
```

### 空のバッファから開始

```rust
use lala::core::EditorCore;

let mut editor = EditorCore::new();
editor.insert(0, "新しいテキスト")?;
println!("{}", editor); // Display traitが実装されている
```

### 既存のテキストで初期化

```rust
use lala::core::EditorCore;

let editor = EditorCore::from_text("Hello, World!");
assert_eq!(editor.len(), 13);
```

## API リファレンス

### `EditorCore`

主要なメソッド:

- `new()` - 空のエディタを作成
- `from_text(text: &str)` - テキストで初期化
- `load_file(path)` - ファイルを非同期で読み込み
- `save_file(path)` - ファイルに非同期で保存
- `insert(index, text)` - 指定位置にテキストを挿入
- `delete(start, end)` - 指定範囲のテキストを削除
- `undo()` - 最後の操作を元に戻す
- `redo()` - Undoした操作をやり直す
- `len()` - バッファの文字数を取得
- `is_empty()` - バッファが空か判定

詳細なAPIドキュメント:
```bash
cargo doc --open
```

## 開発

### テストの実行

```bash
# 全テストを実行
cargo test

# 特定のテストを実行
cargo test test_insert

# テスト出力を表示
cargo test -- --nocapture
```

### ベンチマークの実行

```bash
cargo bench --bench performance
```

### Lintチェック

```bash
cargo clippy -- -D warnings
```

## テストカバレッジ

- **28個のユニットテスト** (全てパス)
  - 基本操作テスト
  - エッジケーステスト
  - 統合テスト
  - 非同期I/Oテスト

## プロジェクト構造

```
lala/
├── src/
│   ├── lib.rs          # ライブラリエントリポイント
│   └── core/
│       ├── mod.rs      # EditorCore実装
│       ├── error.rs    # カスタムエラー型
│       └── tests.rs    # テストコード
├── benches/
│   └── performance.rs  # パフォーマンステスト
├── Cargo.toml          # 依存関係設定
└── README.md           # このファイル
```

## ライセンス

未定

## 貢献

バグ報告や機能リクエストは、GitHubのIssuesで受け付けています。
