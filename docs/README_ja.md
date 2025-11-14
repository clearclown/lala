# Lala エディタ

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](../LICENSE-MIT)
[![Crates.io](https://img.shields.io/crates/v/lala.svg)](https://crates.io/crates/lala)
[![CI](https://github.com/yourusername/lala/workflows/CI/badge.svg)](https://github.com/yourusername/lala/actions)

**他の言語で読む**: [English](../README.md) | [فارسی](README_fa.md) | [العربية](README_ar.md) | [简体中文](README_zh-CN.md) | [繁體中文](README_zh-TW.md) | [Русский](README_ru.md)

RustとeguiとeframeでビルドされたGUIとCLI両対応のモダンで軽量なテキストエディタです。

**最新機能**: 包括的なコマンドラインインターフェースを搭載！**Markdown、HTML、Mermaidダイアグラム、LaTeX**ドキュメントを美しいターミナルフォーマットでプレビュー可能。行番号付きファイル表示など、すべてターミナルから利用できます！

## 目次

- [特徴](#特徴)
- [インストール](#インストール)
- [使い方](#使い方)
- [対応フォーマット](#対応フォーマット)
- [開発](#開発)
- [ライセンス](#ライセンス)

## 特徴

### ✨ ファイルツリー表示
- **ツリー表示**: 左サイドバーに階層的なファイルツリーを表示
- **ディレクトリ展開**: フォルダアイコンをクリックしてディレクトリを展開・折りたたみ
- **ファイルを開く**: ファイルをクリックしてエディタタブで開く
- **非同期読み込み**: ノンブロッキングなディレクトリスキャンでUIがフリーズしない
- **スマートフィルタリング**:
  - `.gitignore`ファイルを尊重（gitリポジトリ内）
  - `.git`ディレクトリを自動的にフィルタ
  - シンボリックリンクをたどらない（セキュリティのため）
- **エラーハンドリング**: 制限されたディレクトリへのアクセス拒否メッセージを表示
- **パフォーマンス**: 大きなディレクトリ（`node_modules`など）に最適化

### 📝 テキスト編集
- **テキスト表示**: Ropeバッファから効率的にテキストを表示
- **テキスト入力**: 完全なキーボード入力サポート（文字、バックスペース、エンター、削除）
- **カーソル管理**: スムーズなカーソルナビゲーションと位置決め
- **マルチタブ編集**: 複数のファイルを同時に開く
- **タブ管理**: ×ボタンでタブを閉じる
- **未保存変更インジケータ**: 変更されたファイルの視覚的インジケータ

### 🔍 高度な検索
- **Grep統合**: ripgrepを使用した高速なファイル全体検索
- **複数ファイル検索**: ディレクトリ内の複数ファイルを横断検索
- **置換機能**: ファイル内のテキストを検索して置換
- **正規表現サポート**: 高度なパターンのための完全な正規表現サポート
- **検索パネル**: 検索操作専用のパネル
- **コンテキスト表示**: 一致した行をコンテキスト付きで表示

### 🎨 シンタックスハイライト
- **コードエディタ**: コード編集用の等幅フォント
- **言語検出**: ファイル拡張子に基づく自動シンタックス検出
- **カラーテーマ**: カスタマイズ可能なテーマでシンタックスハイライト
- **複数言語対応**: syntect経由で様々なプログラミング言語をサポート

### 📖 マルチフォーマットプレビュー
- **Markdownプレビュー**: GitHub風のリッチフォーマットレンダリング
- **HTMLプレビュー**: テーブル、リスト、リンクを含む美しいHTML表示
- **Mermaidダイアグラム**: ASCIIアートのフローチャート、シーケンス図など
- **LaTeXドキュメント**: Unicode数式記号（√、∫、∑、α、βなど）
- **ターミナルレンダリング**: すべてのフォーマットがCLIでカラーサポート付きで表示可能
- **Pure Rust**: WebView依存なし、eguiまたはターミナルで直接レンダリング

## インストール

### 方法1: Cargo（推奨）

Rustがインストールされている場合、最も簡単な方法：

```bash
cargo install lala
```

### 方法2: パッケージマネージャー

#### Arch Linux (AUR)
```bash
yay -S lala
# または
paru -S lala
```

#### Debian/Ubuntu
```bash
sudo add-apt-repository ppa:username/lala
sudo apt update
sudo apt install lala
```

#### Fedora/RHEL
```bash
sudo dnf install lala
```

#### Homebrew (macOS/Linux)
```bash
brew install lala
```

#### Chocolatey (Windows)
```bash
choco install lala
```

### 方法3: バイナリダウンロード

[GitHubリリース](https://github.com/yourusername/lala/releases/latest)から事前コンパイル済みバイナリをダウンロード：

#### Linux
```bash
wget https://github.com/yourusername/lala/releases/latest/download/lala-linux-x86_64.tar.gz
tar xzf lala-linux-x86_64.tar.gz
sudo mv lala /usr/local/bin/
```

#### macOS
```bash
# Intel Mac
wget https://github.com/yourusername/lala/releases/latest/download/lala-macos-x86_64.tar.gz

# Apple Silicon (M1/M2/M3)
wget https://github.com/yourusername/lala/releases/latest/download/lala-macos-aarch64.tar.gz

tar xzf lala-macos-*.tar.gz
sudo mv lala /usr/local/bin/
```

#### Windows
1. [リリースページ](https://github.com/yourusername/lala/releases/latest)から`lala-windows-x86_64.zip`をダウンロード
2. ZIPファイルを展開
3. `lala.exe`をPATHのディレクトリに移動

### 方法4: ソースからビルド

```bash
git clone https://github.com/yourusername/lala.git
cd lala
cargo build --release
sudo cp target/release/lala /usr/local/bin/
```

**詳細なインストール手順**: [インストールガイド](ja/インストール.md)を参照

## 使い方

### GUIモード（デフォルト）

```bash
# 空のエディタを起動
lala

# 特定のファイルを開く
lala file.txt

# ディレクトリを開く
lala /path/to/project
```

### CLIモード

#### Markdownプレビュー
```bash
lala markdown README.md
```

#### HTMLプレビュー
```bash
lala html page.html
```

#### Mermaidダイアグラムプレビュー
```bash
lala mermaid diagram.mmd
```

#### LaTeXドキュメントプレビュー
```bash
lala latex document.tex
```

#### ファイル表示（行番号付き）
```bash
lala view -n src/main.rs
```

### ヘルプ

```bash
# 全般的なヘルプ
lala --help

# 特定のコマンドのヘルプ
lala markdown --help
lala html --help
lala mermaid --help
lala latex --help
```

**詳細なCLI使用方法**: [CLI使い方ガイド](ja/CLI使い方.md)を参照

## 対応フォーマット

### Markdown
- GitHub風スタイル
- 見出し、リスト、コードブロック
- 太字、斜体、インラインコード
- リンク、画像
- 水平線

### HTML
- 見出し（H1-H6）
- テーブル
- リスト（順序付き/順序なし）
- リンク
- コードブロック

### Mermaid
- フローチャート
- シーケンス図
- クラス図
- 状態図
- ER図
- ガントチャート
- 円グラフ

### LaTeX
- Unicode数式記号
- ギリシャ文字
- 数学演算子
- 分数、平方根
- 文書構造（セクション、リスト）

**詳細**: [フォーマット対応ガイド](ja/フォーマット対応.md)を参照

## 開発

### 前提条件

- Rust 1.70以降
- Cargo（Rustに含まれる）

### ビルド

```bash
# デバッグビルド
cargo build

# リリースビルド（最適化）
cargo build --release
```

### テスト

```bash
# 全テスト実行
cargo test

# 出力付きで実行
cargo test -- --nocapture
```

### Lint

```bash
# 一般的な間違いをチェック
cargo clippy --all-targets --all-features

# 自動修正を適用
cargo clippy --fix
```

## キーボードショートカット

### エディタ（GUIモード）
- **Ctrl+S**: ファイルを保存
- **Ctrl+Z**: 元に戻す（予定）
- **Ctrl+Y** または **Ctrl+Shift+Z**: やり直し（予定）

### ナビゲーション（GUIモード）
- **矢印キー**: カーソル移動
- **Page Up/Down**: スクロール
- **Home/End**: 行の先頭/末尾にジャンプ

### テキスト編集（GUIモード）
- **Backspace**: カーソル前の文字を削除
- **Delete**: カーソル位置の文字を削除
- **Enter**: 改行を挿入
- **Tab**: タブ/スペースを挿入

## アーキテクチャ

シングルクレートアプリケーションで、モジュール構造：

```
lala/
├── src/
│   ├── main.rs         # エントリーポイント
│   ├── lib.rs          # ライブラリエクスポート
│   ├── app.rs          # アプリケーション状態
│   ├── cli/            # CLI引数パース
│   ├── core/           # コア編集エンジン
│   ├── file_tree/      # ファイルツリー表示
│   ├── gui/            # GUIコンポーネント
│   └── search/         # 検索機能
```

## 依存関係

主な依存関係：
- **eframe** / **egui**: イミディエイトモードGUIフレームワーク
- **ropey**: 効率的なテキスト編集のためのRopeデータ構造
- **tokio**: ノンブロッキング操作のための非同期ランタイム
- **syntect**: シンタックスハイライトエンジン
- **pulldown-cmark**: Markdownパーサー
- **html2text**: HTMLからテキストへの変換
- **scraper**: HTMLパースとDOM操作
- **clap**: コマンドライン引数パース
- **colored**: ターミナルカラーとフォーマット

## 貢献

貢献を歓迎します！お気軽にissueやpull requestを提出してください。

## ライセンス

このプロジェクトはMITライセンスまたはApache License 2.0のデュアルライセンスです。

- MIT License: [LICENSE-MIT](../LICENSE-MIT)
- Apache License 2.0: [LICENSE-APACHE](../LICENSE-APACHE)

## サポート

- 📖 **ドキュメント**: [ドキュメント一覧](ja/)を参照
- 🐛 **問題報告**: [GitHub Issues](https://github.com/yourusername/lala/issues)
- 💬 **質問・議論**: [GitHub Discussions](https://github.com/yourusername/lala/discussions)
- 📧 **お問い合わせ**: your-email@example.com

## クレジット

Lala Contributors によって作成

---

**詳細ドキュメント**:
- [初心者ガイド](ja/初心者ガイド.md)
- [インストールガイド](ja/インストール.md)
- [CLI使い方ガイド](ja/CLI使い方.md)
- [フォーマット対応](ja/フォーマット対応.md)
- [トラブルシューティング](ja/トラブルシューティング.md)
