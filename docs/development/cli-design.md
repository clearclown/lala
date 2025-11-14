# Lala Editor - CLI Design Document

## 概要
lalaエディタを完全にコマンドラインから操作可能にするための包括的なCLI設計

---

## 設計方針

### 1. ハイブリッドアプローチ
- **GUIモード**: デフォルトの動作（現状）
- **CLIモード**: `--cli` フラグで有効化
- **各種操作コマンド**: サブコマンド形式

### 2. Unix哲学に基づく設計
- Do one thing and do it well
- パイプで連携可能
- 標準入出力対応

---

## コマンド構造

```
lala [OPTIONS] [SUBCOMMAND]
```

---

## サブコマンド一覧

### 1. `lala` (デフォルト - GUIモード)
```bash
# GUI起動（既存の動作）
lala                          # 空のエディタ
lala file.txt                 # ファイルを開く
lala /path/to/directory       # ディレクトリを開く
```

### 2. `lala view` - ファイル閲覧
```bash
# 構文
lala view [OPTIONS] <FILE>

# オプション
-n, --line-numbers          # 行番号を表示
-s, --syntax <LANG>         # シンタックスハイライト言語を指定
-p, --pager                 # lessのようなページャーモード
--no-color                  # カラー出力を無効化
-l, --lines <START>..<END>  # 特定の行範囲を表示

# 例
lala view README.md                    # Markdownファイルを表示
lala view --syntax rust main.rs        # Rustファイルを強調表示
lala view -n -l 1..50 large_file.txt   # 行番号付きで1-50行目を表示
```

### 3. `lala edit` - ファイル編集（CLIモード）
```bash
# 構文
lala edit [OPTIONS] <FILE>

# オプション
-i, --interactive           # 対話モード（TUI）
-c, --command <COMMANDS>    # エディタコマンドを実行
-o, --output <FILE>         # 別ファイルに保存
--backup                    # バックアップを作成

# 例
lala edit file.txt                             # TUIエディタで編集
lala edit -c ":%s/old/new/g" -o new.txt in.txt # 置換して保存
lala edit -c ":10,20d" file.txt                # 10-20行目を削除
```

### 4. `lala search` - ファイル検索
```bash
# 構文
lala search [OPTIONS] <PATTERN> [PATH]

# オプション
-r, --regex                 # 正規表現モード
-i, --ignore-case           # 大文字小文字を区別しない
-w, --whole-word            # 単語全体マッチ
-c, --count                 # マッチ数のみ表示
-l, --files-with-matches    # ファイル名のみ表示
-n, --line-number           # 行番号を表示
-A, --after <NUM>           # マッチ後のN行を表示
-B, --before <NUM>          # マッチ前のN行を表示
-C, --context <NUM>         # マッチ前後のN行を表示
--include <PATTERN>         # ファイルパターンを含む
--exclude <PATTERN>         # ファイルパターンを除外

# 例
lala search "TODO" .                           # TODO を検索
lala search -r "fn\s+\w+" src/                 # 関数定義を検索
lala search -i -n "error" --include "*.rs" .   # Rustファイルからerrorを検索
lala search -c "FIXME" src/                    # FIXME の数を数える
```

### 5. `lala replace` - 検索と置換
```bash
# 構文
lala replace [OPTIONS] <PATTERN> <REPLACEMENT> [PATH]

# オプション
-r, --regex                 # 正規表現モード
-i, --ignore-case           # 大文字小文字を区別しない
-d, --dry-run               # 実際に置換せず、プレビューのみ
-b, --backup                # 置換前にバックアップ作成
--include <PATTERN>         # ファイルパターンを含む
--exclude <PATTERN>         # ファイルパターンを除外
-y, --yes                   # 確認なしで実行

# 例
lala replace "old" "new" file.txt               # 置換
lala replace -r "v\d+\.\d+" "v2.0" --dry-run .  # プレビュー
lala replace -i "TODO" "DONE" -b src/           # バックアップ付き置換
```

### 6. `lala markdown` - Markdown処理
```bash
# 構文
lala markdown [OPTIONS] <SUBCOMMAND> <FILE>

# サブコマンド
preview   # プレビュー表示
render    # HTML変換
toc       # 目次生成

# オプション（preview）
-s, --style <STYLE>         # スタイルテーマ (github, minimal, dark)
-w, --watch                 # ファイル変更を監視
-p, --port <PORT>           # プレビューサーバーのポート

# オプション（render）
-o, --output <FILE>         # 出力ファイル
-t, --template <FILE>       # HTMLテンプレート
--standalone                # スタンドアロンHTML

# オプション（toc）
--max-depth <NUM>           # 目次の最大階層
--no-links                  # リンクなしの目次

# 例
lala markdown preview README.md                 # プレビュー表示
lala markdown preview -w -p 8080 docs/          # ライブプレビュー
lala markdown render -o output.html README.md   # HTML変換
lala markdown render --standalone README.md     # 完全なHTML生成
lala markdown toc README.md                     # 目次生成
```

### 7. `lala diff` - ファイル比較
```bash
# 構文
lala diff [OPTIONS] <FILE1> <FILE2>

# オプション
-u, --unified               # unified diff形式
-s, --side-by-side          # 並列表示
-c, --context <NUM>         # コンテキスト行数
--color                     # カラー表示

# 例
lala diff old.txt new.txt                       # 差分表示
lala diff -s file1.rs file2.rs                  # 並列比較
lala diff -u -c 5 v1.txt v2.txt                 # unified diff
```

### 8. `lala format` - コード整形
```bash
# 構文
lala format [OPTIONS] <FILE>

# オプション
-l, --language <LANG>       # 言語を指定
-c, --config <FILE>         # 設定ファイル
-i, --in-place              # 元ファイルを上書き
-d, --diff                  # 差分のみ表示

# 例
lala format -l rust src/main.rs                 # Rustコード整形
lala format -i -c .prettier.json app.js         # 設定ファイル使用
lala format -d code.py                          # 差分プレビュー
```

### 9. `lala stats` - ファイル統計
```bash
# 構文
lala stats [OPTIONS] [PATH]

# オプション
-l, --lines                 # 行数のみ
-w, --words                 # 単語数のみ
-c, --chars                 # 文字数のみ
--include <PATTERN>         # ファイルパターン
--by-language               # 言語別に集計

# 例
lala stats .                                    # プロジェクト全体の統計
lala stats --by-language src/                   # 言語別行数
lala stats -l --include "*.rs" src/             # Rustファイルの行数
```

### 10. `lala tree` - ファイルツリー表示
```bash
# 構文
lala tree [OPTIONS] [PATH]

# オプション
-d, --max-depth <NUM>       # 最大階層
-a, --all                   # 隠しファイルも表示
-I, --ignore <PATTERN>      # パターンを除外
-L, --follow-links          # シンボリックリンクを追跡
--no-color                  # カラー無効化

# 例
lala tree .                                     # ツリー表示
lala tree -d 2 src/                             # 2階層まで
lala tree -a -I "node_modules" .                # 隠しファイル表示
```

---

## グローバルオプション

```bash
# すべてのコマンドで使用可能
-h, --help                  # ヘルプ表示
-V, --version               # バージョン表示
-v, --verbose               # 詳細出力
-q, --quiet                 # エラーのみ出力
--no-color                  # カラー出力無効
--config <FILE>             # 設定ファイル指定
```

---

## パイプ連携

```bash
# 例1: 検索結果を編集
lala search "TODO" src/ | lala edit -

# 例2: 複数ファイルの統計
find . -name "*.rs" | xargs lala stats --lines

# 例3: MarkdownをHTMLに変換してブラウザで開く
lala markdown render README.md | browser

# 例4: 差分をgrepで抽出
lala diff old.txt new.txt | grep "^+"

# 例5: ファイルツリーをファイルに保存
lala tree . > project_structure.txt
```

---

## 設定ファイル

### 場所
- `~/.config/lala/config.toml` (Linux/Mac)
- `%APPDATA%\lala\config.toml` (Windows)

### 設定例
```toml
[general]
default_mode = "gui"  # "gui" | "cli"
color_output = true

[editor]
tab_size = 4
insert_spaces = true
line_numbers = true

[search]
ignore_case = false
use_regex = false
max_results = 1000

[markdown]
default_style = "github"
syntax_highlighting = true

[theme]
syntax_theme = "Monokai"
```

---

## TUIモード（将来実装）

```bash
# 対話型エディタ起動
lala edit --interactive file.txt

# または
lala tui file.txt
```

**TUI機能:**
- Vimライクなキーバインド
- 複数バッファ
- 分割ウィンドウ
- コマンドパレット

---

## 実装優先度

### Phase 1（必須）
- [ ] `lala view` - ファイル閲覧
- [ ] `lala search` - 基本検索
- [ ] `lala markdown preview` - Markdownプレビュー（CLIで表示）
- [ ] `lala markdown render` - HTML変換
- [ ] `lala stats` - 基本統計

### Phase 2（重要）
- [ ] `lala edit --command` - 非対話的編集
- [ ] `lala replace` - 置換機能
- [ ] `lala tree` - ツリー表示
- [ ] `lala diff` - 差分表示

### Phase 3（拡張）
- [ ] `lala format` - コード整形
- [ ] `lala edit --interactive` - TUIエディタ
- [ ] `lala markdown watch` - ライブプレビュー
- [ ] 設定ファイルサポート

---

## 技術的実装

### 使用ライブラリ
- **clap 4.x** - CLIパース（既存）
- **colored** - カラー出力
- **termion** / **crossterm** - TUI
- **syntect** - シンタックスハイライト（既存）
- **pulldown-cmark** - Markdown処理（既存）
- **comrak** - 高度なMarkdown機能
- **similar** - 差分計算

### アーキテクチャ
```
src/
├── cli/
│   ├── mod.rs           # CLIエントリーポイント
│   ├── commands/        # 各サブコマンド
│   │   ├── view.rs
│   │   ├── search.rs
│   │   ├── markdown.rs
│   │   └── ...
│   ├── output.rs        # 出力フォーマッティング
│   └── config.rs        # 設定ファイル処理
├── gui/                 # 既存のGUI（変更なし）
└── core/                # 共通コア機能
```

---

## Markdown プレビューの実装状態

### 現状
- `src/gui/markdown_preview.rs` に実装済み
- pulldown-cmark使用
- eguiで直接レンダリング
- **ただし、GUIにまだ統合されていない**

### CLI対応案

#### オプション1: ターミナルでのMarkdown表示
```bash
lala markdown preview README.md

# 出力例（カラー付き）
## Heading
This is **bold** and *italic*

- List item 1
- List item 2

\`\`\`rust
fn main() {
    println!("Hello");
}
\`\`\`
```

#### オプション2: HTML生成
```bash
lala markdown render README.md > output.html
```

#### オプション3: ブラウザプレビュー
```bash
lala markdown preview --browser README.md
# ローカルサーバーを起動してブラウザで開く
```

---

## 使用例

### 1. プロジェクト内のTODOを検索
```bash
lala search -n "TODO\|FIXME" src/
```

### 2. README をMarkdownでプレビュー
```bash
lala markdown preview README.md | less -R
```

### 3. 全Rustファイルの行数を集計
```bash
lala stats --lines --include "*.rs" .
```

### 4. ドキュメントをHTMLに変換
```bash
lala markdown render --standalone docs/guide.md -o guide.html
```

### 5. ファイル内の文字列を一括置換
```bash
lala replace -r "v\d+\.\d+\.\d+" "v2.0.0" --backup .
```

---

## まとめ

この設計により、lalaは：
1. ✅ 完全なCLI操作が可能
2. ✅ Unixツールチェインと統合可能
3. ✅ GUIとCLI両方をサポート
4. ✅ スクリプトから使いやすい
5. ✅ 段階的な実装が可能

次のステップ: Phase 1の実装から開始
