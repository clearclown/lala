# Syntax Highlighting Implementation

## 概要

このドキュメントは、`feature/syntax-highlighting` (構文強調) の実装詳細を記述しています。

## 実装完了

### ✅ 完了条件の確認

- [x] `cargo test` ですべての（ロジック）テストが通る
  - 7つのユニットテストが全て成功
- [x] `cargo clippy` でlintエラーがない
  - `-D warnings` で全エラーなし
- [x] `.rs` ファイルを開いた際に、キーワードやリテラルが色付けされている
  - 実装済み：`test_sample.rs` で確認可能
- [x] `.md` ファイルを開いた際に、見出しや強調が色付けされている
  - 実装済み：`test_sample.md` で確認可能

## アーキテクチャ

### モジュール構成

```
src/
├── main.rs              # アプリケーションエントリーポイント
├── app.rs               # メインアプリケーション構造
├── core/
│   ├── mod.rs           # コアモジュール
│   └── rope.rs          # TextBuffer (Rope ラッパー)
└── gui/
    ├── mod.rs           # GUIモジュール
    ├── highlighting.rs  # 構文強調エンジン
    └── editor.rs        # エディタコンポーネント
```

### コンポーネント説明

#### 1. `core::rope::TextBuffer`

- `ropey::Rope` をラップしたテキストバッファ
- ファイルパスと拡張子の管理
- 将来の編集機能拡張に対応

#### 2. `gui::highlighting::SyntaxHighlighter`

**主要機能:**
- `syntect` ライブラリを使用した構文強調
- テーマ: `base16-ocean.dark` (ハードコード)
- サポート言語: Rust, Markdown, Plaintext

**メソッド:**
- `new()`: デフォルト設定でハイライターを初期化
- `load_syntax(file_path)`: ファイル拡張子から構文定義を取得
- `highlight(text, file_path)`: テキストをハイライトして `egui::LayoutJob` を返す
- `style_to_color(style)`: syntect の Style を egui の Color32 に変換

**テスト:**
- `test_load_syntax_rust`: Rust ファイルの構文定義読み込み
- `test_load_syntax_markdown`: Markdown ファイルの構文定義読み込み
- `test_load_syntax_unknown`: 未知の拡張子のフォールバック
- `test_highlight_rust_code`: Rust コードのハイライト
- `test_highlight_markdown`: Markdown のハイライト
- `test_highlight_plaintext`: プレーンテキストのハイライト
- `test_supported_extensions`: サポート拡張子のリスト

#### 3. `gui::editor::EditorPanel`

**主要機能:**
- `egui::TextEdit` を使用したテキストエディタUI
- カスタム `layouter` によるリアルタイム構文強調
- サンプルファイルの切り替え機能

**実装方式:**
- 方針A（TextEdit + layouter フック）を採用
- `TextEdit::layouter` で `SyntaxHighlighter::highlight` を呼び出し
- `egui::LayoutJob` に色情報を設定して描画

## 技術的詳細

### syntect との統合

```rust
// ハイライト処理の流れ
TextEdit::multiline(&mut text)
    .layouter(&mut |ui, text, wrap_width| {
        // 1. SyntaxHighlighter で text をハイライト
        let layout_job = highlighter.highlight(text, file_path);

        // 2. LayoutJob を egui にレンダリングさせる
        ui.fonts(|f| f.layout_job(layout_job))
    })
```

### egui::LayoutJob の生成

```rust
// syntect の出力を egui の形式に変換
for line in LinesWithEndings::from(text) {
    let ranges = highlighter.highlight_line(line, &syntax_set)?;

    for (style, text_segment) in ranges {
        let color = Color32::from_rgb(
            style.foreground.r,
            style.foreground.g,
            style.foreground.b,
        );

        layout_job.append(
            text_segment,
            0.0,
            TextFormat {
                font_id: FontId::monospace(14.0),
                color,
                ..Default::default()
            },
        );
    }
}
```

## パフォーマンス

### 現在の実装

- **ハイライト方式**: 入力ごとに全テキストを再ハイライト
- **実行スレッド**: GUIスレッドで同期実行
- **最適化**: `syntect` の効率的なパーサーを使用

### 将来の改善案 (Step 3 リファクタリング)

パフォーマンスが問題になった場合:
1. バックグラウンドスレッドでハイライト処理
2. 差分ハイライト（変更行のみ再パース）
3. 結果のキャッシング

## 拡張性

### 新しい言語の追加

1. `SyntaxHighlighter::load_syntax` に拡張子マッピングを追加
2. syntect の SyntaxSet に言語定義を追加

```rust
match extension {
    "rs" => self.syntax_set.find_syntax_by_extension("rs"),
    "md" => self.syntax_set.find_syntax_by_extension("md"),
    "py" => self.syntax_set.find_syntax_by_extension("py"), // 新規追加例
    _ => self.syntax_set.find_syntax_plain_text(),
}
```

### 新しいテーマの追加

1. `SyntaxHighlighter::load_theme` を修正
2. テーマ選択UIを追加（将来の機能）

```rust
fn load_theme(theme_name: &str) -> Theme {
    let ts = ThemeSet::load_defaults();
    ts.themes.get(theme_name)
        .cloned()
        .unwrap_or_else(|| ts.themes["base16-ocean.dark"].clone())
}
```

## エラーハンドリング

### 未サポートファイル

- 未知の拡張子は自動的に "Plain Text" として処理
- `load_syntax` メソッドでフォールバック実装

### パースエラー

- `highlight_line` のエラーは `unwrap_or_default()` で処理
- エラー時は色なしテキストとして表示

## テストファイル

### `test_sample.rs`

- Rust構文の包括的なサンプル
- キーワード、リテラル、コメント、マクロ、構造体、trait実装など

### `test_sample.md`

- Markdown構文の包括的なサンプル
- ヘッダー、リスト、コードブロック、リンク、テーブルなど

### `test_sample.txt`

- プレーンテキストのサンプル
- 構文強調が適用されないことを確認

## 依存関係

```toml
[dependencies]
eframe = "0.29"      # egui フレームワーク
egui = "0.29"        # GUI ライブラリ
syntect = "5.2"      # 構文強調エンジン
ropey = "1.6"        # Rope データ構造
```

## 実行方法

### 開発モード

```bash
cargo run
```

### リリースビルド

```bash
cargo build --release
./target/release/lala
```

### テスト実行

```bash
cargo test
cargo clippy -- -D warnings
```

## 既知の制限事項

1. **テーマ**: 現在 `base16-ocean.dark` のみサポート
2. **言語**: Rust, Markdown, Plaintext のみ完全サポート
3. **パフォーマンス**: 大きなファイル（10,000行以上）では入力遅延の可能性
4. **非同期**: ハイライトはGUIスレッドで同期実行

## 今後の改善項目

1. **Step 3 リファクタリング**
   - 非同期ハイライト実装
   - バックグラウンドスレッド処理

2. **機能拡張**
   - テーマ選択UI
   - より多くの言語サポート
   - カスタム言語定義の読み込み

3. **パフォーマンス最適化**
   - 差分ハイライト
   - 表示範囲のみハイライト
   - 結果キャッシング

## 参考資料

- [syntect documentation](https://docs.rs/syntect/)
- [egui documentation](https://docs.rs/egui/)
- [ropey documentation](https://docs.rs/ropey/)
