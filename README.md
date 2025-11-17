<div align="center">

# 🎵 Lala

**モダンで軽量なマルチフォーマット対応テキストエディタ**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()
[![Tests](https://img.shields.io/badge/tests-171%20passing-brightgreen.svg)]()

[English](./docs/readmeLangs/README_en.md) | [日本語](README.md) | [简体中文](./docs/readmeLangs/README_zh-CN.md) | [繁體中文](./docs/readmeLangs/README_zh-TW.md) | [Русский](./docs/readmeLangs/README_ru.md) | [فارسی](./docs/readmeLangs/README_fa.md) | [العربية](./docs/readmeLangs/README_ar.md)

</div>

---

## 📸 スクリーンショット

### ダークテーマでのMarkdown編集
*コードブロックの構文ハイライト、リアルタイムプレビュー*

### ライトテーマでの執筆
*目に優しい配色、日本語IME完全対応*

### マルチフォーマットプレビュー
*Markdown、HTML、LaTeX、Mermaid図を即座にプレビュー*

---

## 💡 Lalaとは？

Lalaは**Rust**で書かれた次世代テキストエディタです。純粋なRust GUI フレームワーク **egui** を使用し、軽量かつ高速な動作を実現しています。

### なぜLalaが必要なのか？

既存のテキストエディタには以下の課題があります：

- 🐌 **重量級**: ElectronベースのエディタはメモリとCPUを大量消費
- 🌐 **ネイティブ感の欠如**: Webベースエディタは応答性に欠ける
- 📝 **プレビューの不便さ**: Markdownやその他形式のリアルタイムプレビューが不十分
- 🌏 **IME対応の不完全さ**: 日本語・中国語など複雑な入力メソッドのサポートが不十分

Lalaはこれらの問題を解決します。

---

## ✨ 主な機能

### 🎨 マルチフォーマット対応
- **Markdown**: リアルタイムプレビュー、構文ハイライト
- **HTML**: パース&レンダリング
- **LaTeX**: 数式・記号のUnicode変換プレビュー
- **Mermaid**: フローチャート・シーケンス図の可視化

### 🚀 高性能
- **軽量**: Rustのゼロコスト抽象化による高速起動（<100ms）
- **効率的**: Rope データ構造による大容量ファイルの高速編集
- **ネイティブ**: Electron不要、システムリソースへの負荷が最小限

### 🌏 日本語完全対応
- **IME対応**: 日本語・中国語・韓国語の入力メソッドをネイティブサポート
- **Unicodeサポート**: 絵文字・記号を含む全Unicode文字に対応

### 🤖 AI統合（オプション）
- **Gemini API統合**: テキストの自動改善
- **文法修正**: スペル・文法エラーの自動検出と修正
- **要約機能**: 長文の自動要約

### 🔍 強力な検索機能
- **バッファ内検索**: 正規表現対応の高速検索・置換
- **Grep機能**: プロジェクト全体を対象とした高速検索
- **.gitignore対応**: 不要なファイルを自動除外

### 🎨 テーマ
- **ダークテーマ**: 長時間の作業に最適
- **ライトテーマ**: カスタマイズされた視認性の高い配色

---

## 📦 インストール

### 方法1: Cargoからインストール（最も簡単・推奨）⭐

Rustツールチェーンがインストールされている場合：

```bash
# crates.ioから最新版をインストール
cargo install lala

# AI機能も使いたい場合
cargo install lala --features llm
```

**Rustのインストール**: https://rustup.rs/

### 方法2: GitHub Releasesからバイナリをダウンロード

Rustをインストールしたくない場合は、プリコンパイル済みバイナリを使用：

```bash
# Linux x86_64
curl -L https://github.com/clearclown/lala/releases/latest/download/lala-linux-x86_64 -o lala
chmod +x lala
sudo mv lala /usr/local/bin/

# macOS (Intel)
curl -L https://github.com/clearclown/lala/releases/latest/download/lala-macos-x86_64 -o lala
chmod +x lala
sudo mv lala /usr/local/bin/

# macOS (Apple Silicon)
curl -L https://github.com/clearclown/lala/releases/latest/download/lala-macos-aarch64 -o lala
chmod +x lala
sudo mv lala /usr/local/bin/

# Windows
# https://github.com/clearclown/lala/releases/latest から
# lala-windows-x86_64.exe をダウンロード
```

### 方法3: ソースからビルド

開発者向け：

```bash
# リポジトリをクローン
git clone https://github.com/clearclown/lala.git
cd lala

# リリースビルド
cargo build --release

# バイナリは target/release/lala に生成されます

# システムにインストール（オプション）
cargo install --path .

# AI機能を有効にしてインストール
cargo install --path . --features llm
```

### アンインストール

```bash
# Cargoでインストールした場合
cargo uninstall lala
```

### AI機能を有効化（オプション）

```bash
# llm機能を有効にしてビルド
cargo build --release --features llm

# または
cargo install --path . --features llm
```

---

## 🚀 使い方

### GUIモードで起動

```bash
# 空のエディタを起動
lala

# 特定のファイルを開く
lala README.md

# ディレクトリを開く
lala ./docs
```

### CLIモード（プレビュー）

```bash
# Markdownをターミナルでプレビュー
lala README.md --preview

# HTMLをプレビュー
lala index.html --preview

# LaTeXをプレビュー
lala document.tex --preview
```

### キーボードショートカット

| ショートカット | 機能 |
|---------------|------|
| `Ctrl+N` | 新規ファイル |
| `Ctrl+O` | ファイルを開く |
| `Ctrl+S` | 保存 |
| `Ctrl+Shift+S` | 名前を付けて保存 |
| `Ctrl+F` | 検索 |
| `Ctrl+H` | 置換 |
| `Ctrl+Shift+F` | プロジェクト全体を検索（Grep） |
| `Ctrl+P` | プレビュー表示切替 |
| `Esc` | パネルを閉じる |

---

## ⚙️ 設定

### AI機能の設定

1. **Gemini APIキーの取得**
   - [Google AI Studio](https://ai.google.dev/tutorials/setup)でAPIキーを取得

2. **環境変数の設定**
   ```bash
   export GEMINI_API_KEY="your-api-key-here"
   ```

3. **または、GUI設定から**
   - `Tools > Settings` メニューを開く
   - APIキーを入力
   - "AI機能を有効化"をチェック

### AI機能の使い方
- **🤖 Improve Markdown**: Markdownの構造・書式を改善
- **✨ Fix Grammar**: 文法・スペルミスを修正
- **📝 Summarize**: テキストを要約

---

## 🗑️ アンインストール

```bash
# Cargoでインストールした場合
cargo uninstall lala

# または手動で削除
rm ~/.cargo/bin/lala  # Linux/macOS
# Windowsの場合: %USERPROFILE%\.cargo\bin\lala.exe を削除
```

---

## 📚 ドキュメント

### アーキテクチャ

```
lala/
├── src/
│   ├── main.rs              # エントリポイント
│   ├── cli/                 # CLIインターフェース
│   │   ├── markdown_view.rs # Markdownプレビュー
│   │   ├── html_view.rs     # HTMLプレビュー
│   │   ├── latex_view.rs    # LaTeXプレビュー
│   │   └── mermaid_view.rs  # Mermaid図プレビュー
│   ├── gui/                 # GUIインターフェース
│   │   ├── app.rs          # メインアプリケーション
│   │   ├── theme.rs        # テーマ設定
│   │   ├── dialogs.rs      # ダイアログUI
│   │   ├── menu.rs         # メニューバー
│   │   ├── previews.rs     # プレビュー機能
│   │   ├── markdown_preview.rs  # Markdownレンダラー
│   │   └── search_panel.rs # 検索パネル
│   ├── core_engine/        # コアエンジン
│   │   └── buffer.rs       # テキストバッファ管理
│   ├── search/             # 検索機能
│   │   ├── buffer_search.rs # バッファ内検索
│   │   └── grep.rs         # Grep検索
│   ├── llm/                # LLM統合（オプション）
│   │   └── mod.rs          # Gemini APIクライアント
│   └── file_tree/          # ファイルツリー
└── tests/                  # 171個のテスト
```

### 技術スタック

- **言語**: Rust 2021 Edition
- **GUIフレームワーク**: egui 0.33 + eframe
- **テキスト処理**: ropey (Rope構造)
- **Markdownパーサー**: pulldown-cmark
- **構文ハイライト**: syntect
- **非同期処理**: Tokio
- **正規表現**: regex
- **AI統合**: reqwest + Gemini API

### テスト

```bash
# すべてのテストを実行
cargo test --all-features

# 特定のテストスイートを実行
cargo test --test core_engine_test
cargo test --test llm_test
cargo test --test preview_test

# 統合テスト
cargo test --test end_to_end_test
```


---

## 🤝 コントリビューション

コントリビューションを歓迎します！以下の方法で貢献できます：

### バグ報告・機能リクエスト
1. [Issues](https://github.com/clearclown/lala/issues)で新しいIssueを作成
2. バグの場合は再現手順を記載
3. 機能リクエストの場合はユースケースを説明

### プルリクエスト
1. このリポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m '✨ Add amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

### 開発ガイドライン
- コードスタイル: `cargo fmt` を使用
- Linter: `cargo clippy` でチェック
- テスト: 新機能には必ずテストを追加
- ドキュメント: パブリックAPIには文書化コメントを記載

---

## 📖 リソース

- [公式ドキュメント](https://github.com/clearclown/lala/blob/main/README.md)
- [Issue Tracker](https://github.com/clearclown/lala/issues)
- [変更履歴](https://github.com/clearclown/lala/blob/main/CHANGELOG.md)
- [ロードマップ](https://github.com/clearclown/lala/blob/main/ROADMAP.md)

### 関連プロジェクト
- [egui](https://github.com/emilk/egui) - 純粋なRust GUIフレームワーク
- [ropey](https://github.com/cessen/ropey) - 高速テキストRopeライブラリ
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdownパーサー
- [syntect](https://github.com/trishume/syntect) - 構文ハイライトライブラリ

---

## 📄 ライセンス

このプロジェクトはデュアルライセンスです：

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) または http://opensource.org/licenses/MIT)
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0)

いずれかのライセンスを選択できます。

### コントリビューション

特に断りのない限り、Apache License 2.0で定義される通り、あなたが意図的にこのプロジェクトに提出した貢献は、追加の条項や条件なしに、上記のようにデュアルライセンスとなります。

---

## 🙏 謝辞

以下のオープンソースプロジェクトに感謝します：
- [Rust プログラミング言語](https://www.rust-lang.org/)
- [egui コミュニティ](https://github.com/emilk/egui)
- すべてのコントリビューター
---

<div align="center">

**[⬆ トップに戻る](#-lala)**

Made with ❤️ by the Lala contributors

</div>
