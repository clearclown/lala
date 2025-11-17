# 📦 Cargo (crates.io) リリースガイド

**難易度**: ★☆☆☆☆ (最も簡単)
**推定時間**: 15-30分
**優先度**: ⭐ 最優先

---

## 🖱️ GUI操作が必須の部分

以下の操作は**ブラウザでの手動操作が必要**です：

1. **crates.ioアカウント作成**（初回のみ、1分）
   - https://crates.io/ にアクセス
   - 「Log in with GitHub」をクリック
   - GitHubアカウントで認証

2. **APIトークンの取得**（初回のみ、1分）
   - ログイン後、https://crates.io/settings/tokens にアクセス
   - 「New Token」をクリック
   - トークン名を入力（例: "lala-publishing"）
   - トークンをコピー

**それ以外は全てCLIで完結します！**

---

## 🎯 概要

crates.ioはRustの公式パッケージレジストリです。
ここに公開すると、世界中のRustユーザーが`cargo install lala`でインストールできるようになります。

### メリット
- Rustユーザーに最も簡単にアクセスできる
- 依存関係が自動的に管理される
- クロスプラットフォーム対応（コンパイル時にビルド）
- メンテナンスが簡単

---

## 📋 前提条件

### 1. crates.ioアカウントの作成

1. [crates.io](https://crates.io/)にアクセス
2. 右上の「Log in with GitHub」をクリック
3. GitHubアカウントで認証

### 2. APIトークンの取得

```bash
# crates.ioにログイン後、以下のURLにアクセス
# https://crates.io/settings/tokens

# または、cargo loginコマンドを使用
cargo login
```

トークンを入力すると、`~/.cargo/credentials`に保存されます。

---

## ✅ リリース前チェックリスト

### 1. Cargo.tomlの確認

以下の項目が正しく設定されているか確認：

```toml
[package]
name = "lala"
version = "0.1.0"  # セマンティックバージョニング
authors = ["Your Name <your.email@example.com>"]
edition = "2021"
description = "Modern, lightweight multi-format text editor written in Rust"
homepage = "https://github.com/clearclown/lala"
repository = "https://github.com/clearclown/lala"
license = "MIT OR Apache-2.0"
keywords = ["editor", "text-editor", "gui", "markdown", "rust"]
categories = ["text-editors", "command-line-utilities", "gui"]
readme = "README.md"

# オプションだが推奨
documentation = "https://github.com/clearclown/lala#readme"
```

**確認コマンド**:
```bash
grep -A 15 "^\[package\]" Cargo.toml
```

### 2. README.mdの確認

crates.ioのプロジェクトページに表示されるため、以下を確認：

- [ ] プロジェクトの説明が明確
- [ ] インストール方法が記載されている
- [ ] 基本的な使用例がある
- [ ] ライセンス情報が記載されている

### 3. LICENSEファイルの確認

```bash
# MIT と Apache-2.0 の両方が存在するか確認
ls -la LICENSE-MIT LICENSE-APACHE
```

### 4. 不要なファイルの除外

**重要**: `.cargo-ignore`ではなく、`Cargo.toml`の`exclude`キーを使用します：

```toml
[package]
exclude = [
    ".github/",
    "docs/",
    "tests/",
    "screenshots/",
    "*.sh",
    # README.mdとCHANGELOG.mdは含める
    # （excludeに指定しない）
    "target/",
    "Cargo.lock",
    ".vscode/",
    ".idea/",
]

# または includeで明示的に指定
# include = [
#     "src/**/*",
#     "Cargo.toml",
#     "Cargo.lock",
#     "README.md",
#     "CHANGELOG.md",
#     "LICENSE-MIT",
#     "LICENSE-APACHE",
# ]
```

**注意**: パッケージサイズの上限は10MBです。

---

## 🚀 リリース手順

### Step 1: ドライラン（テスト公開）

実際に公開する前に、問題がないか確認：

```bash
cargo publish --dry-run
```

**チェック項目**:
- ✅ パッケージングが成功する
- ✅ 警告やエラーがない
- ✅ ファイルサイズが適切（通常 < 10MB）
- ✅ 含まれるファイルが正しい

**出力例**:
```
   Packaging lala v0.1.0 (/home/user/lala)
   Verifying lala v0.1.0 (/home/user/lala)
   Compiling lala v0.1.0 (/home/user/lala/target/package/lala-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 45.23s
```

### Step 2: 含まれるファイルの確認

```bash
cargo package --list | head -20
```

不要なファイルが含まれていないか確認します。

### Step 3: ビルドテスト

パッケージ化されたバージョンがビルドできるか確認：

```bash
cargo package
cd target/package/lala-0.1.0
cargo build --release
cd ../../..
```

### Step 4: 公開

すべてのチェックが完了したら、公開：

```bash
cargo publish
```

**成功時の出力**:
```
    Updating crates.io index
   Packaging lala v0.1.0 (/home/user/lala)
   Verifying lala v0.1.0 (/home/user/lala)
   Compiling lala v0.1.0 (/home/user/lala/target/package/lala-0.1.0)
    Finished dev [unoptimized + debuginfo] target(s) in 45.23s
   Uploading lala v0.1.0 (/home/user/lala)
```

---

## 📊 公開後の確認

### 1. crates.ioでの確認

```bash
# ブラウザで確認
xdg-open https://crates.io/crates/lala
```

以下を確認：
- [ ] プロジェクト説明が正しく表示されている
- [ ] READMEが正しくレンダリングされている
- [ ] ライセンス情報が正しい
- [ ] キーワードとカテゴリが適切

### 2. インストールテスト

別のディレクトリで実際にインストールしてみる：

```bash
# 新しいディレクトリで
cd /tmp
cargo install lala
lala --version
```

### 3. ドキュメントの確認

```bash
# docs.rs でのドキュメント生成を確認
xdg-open https://docs.rs/lala
```

**注意**: ドキュメントの生成には数分かかる場合があります。

---

## 🔄 アップデート方法

新しいバージョンをリリースする場合：

### 1. バージョンの更新

```bash
# Cargo.toml のバージョンを更新
# version = "0.1.0" → version = "0.1.1"

# 自動的に更新するには cargo-edit を使用
cargo install cargo-edit
cargo set-version 0.1.1
```

### 2. CHANGELOGの更新

```bash
cat >> CHANGELOG.md << 'EOF'

## [0.1.1] - 2025-01-XX

### Added
- 新機能の説明

### Fixed
- バグ修正の説明

### Changed
- 変更点の説明
EOF
```

### 3. コミットとタグ

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: Bump version to 0.1.1"
git tag -a v0.1.1 -m "Release version 0.1.1"
git push origin main --tags
```

### 4. 再公開

```bash
cargo publish
```

---

## 🚨 トラブルシューティング

### エラー: "the remote server responded with an error: crate name is already taken"

**原因**: パッケージ名が既に使用されている

**解決策**:
```bash
# 名前が使用可能か確認
cargo search lala

# 名前を変更する必要がある場合
# Cargo.toml の name を変更
# 例: name = "lala-editor"
```

### エラー: "failed to verify package tarball"

**原因**: パッケージのビルドが失敗している

**解決策**:
```bash
# 詳細なエラーを確認
cargo publish --dry-run --verbose

# target/package/ を削除して再試行
rm -rf target/package
cargo publish --dry-run
```

### エラー: "some files in the working directory contain changes"

**原因**: 未コミットの変更がある

**解決策**:
```bash
# 変更を確認
git status

# コミットまたはstash
git add .
git commit -m "Prepare for release"
# または
git stash
```

### 警告: "manifest has no documentation, homepage or repository"

**原因**: Cargo.tomlに推奨フィールドが不足

**解決策**:
```toml
[package]
homepage = "https://github.com/clearclown/lala"
repository = "https://github.com/clearclown/lala"
documentation = "https://github.com/clearclown/lala#readme"
```

---

## 📚 参考リンク

- [The Cargo Book - Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Cargo.toml Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [crates.io ポリシー](https://crates.io/policies)
- [セマンティックバージョニング](https://semver.org/lang/ja/)

---

## ✅ 完了チェック

リリースが完了したら、以下を確認：

- [ ] `cargo publish` が成功
- [ ] crates.io でプロジェクトページが表示される
- [ ] `cargo install lala` でインストールできる
- [ ] インストールしたバイナリが正常に動作する
- [ ] docs.rs でドキュメントが生成される（数分後）
- [ ] todo.md の「Cargo (crates.io)」にチェックを入れる

---

**次のステップ**: Phase 1の2番目として [github-releases.md](./github-releases.md) に進んでください。
