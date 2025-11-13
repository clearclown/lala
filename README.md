# lala

Lala Editor - A cross-platform text editor built with Rust and egui.

## Features

### ✅ Implemented: feature/gui-base (GUI Base & Tab System)

GUI基盤とタブシステムを実装しました。

#### 機能

- **ウィンドウ表示**: egui/eframeを使用したクロスプラットフォームのネイティブウィンドウ
- **UI骨格**:
  - トップメニューバー領域（プレースホルダー、将来拡張予定）
  - 左サイドバー領域（将来的にfile-tree機能で使用）
  - メインエディタ領域
  - 下部ステータスバー領域（将来拡張予定）
- **タブシステム**:
  - 複数ドキュメントのタブ管理
  - タブにファイル名を表示
  - 未保存マーク「*」の表示（将来的に実装）
  - タブを閉じる「✕」ボタン
  - 新しいタブの追加

#### プロジェクト構造

```
src/
├── main.rs              # アプリケーションのエントリーポイント
├── core/
│   └── mod.rs          # core-engine のダミー実装（将来置き換え予定）
└── gui/
    ├── mod.rs          # GUIモジュールのメインファイル（EditorApp実装）
    ├── app_state.rs    # アプリケーション状態とタブ管理ロジック
    └── tab.rs          # タブの状態管理
```

#### API

`AppState`は以下のAPIを提供します：

- `open_new_tab(file_name)` - 新しいタブを開く
- `close_tab(index)` - タブを閉じる
- `active_tab()` - アクティブなタブへの参照を取得
- `active_tab_mut()` - アクティブなタブへの可変参照を取得
- `set_active_tab(index)` - アクティブなタブを設定
- `tabs()` - すべてのタブへの参照を取得

#### テスト

13個のユニットテストを実装し、タブ管理ロジックを検証しています：

```bash
cargo test
```

#### ビルドと実行

```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release

# 実行
cargo run
```

#### Lint

```bash
cargo clippy -- -D warnings
```

## 今後の実装予定

- [ ] feature/file-tree: ファイルツリー表示
- [ ] feature/basic-editing: 基本的なテキスト編集機能
- [ ] feature/core-engine: コアエンジンの実装
- [ ] feature/core-cli: CLIインターフェース

## 開発環境

- Rust 2021 edition
- egui 0.31
- eframe 0.31

## ライセンス

TBD
