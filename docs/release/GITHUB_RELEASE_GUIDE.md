# GitHub Release 手順まとめ

## 📋 概要

GitHub Releaseを使用して、コンパイル済みのバイナリを複数のプラットフォーム向けに配布する方法です。
タグをプッシュするだけで、GitHub Actionsが自動的にビルド・リリースを行います。

---

## ⚠️ GUI操作が必須の部分（初回設定）

以下の操作は**ブラウザでの手動操作が必要**です：

### 1. GitHub Actions権限の設定（初回のみ、30秒）

**重要**: これを設定しないと自動リリースが失敗します！

**手順**:
1. リポジトリページにアクセス
2. Settings → Actions → General に移動
3. "Workflow permissions" セクションで **"Read and write permissions"** を選択
4. Save をクリック

**設定場所**: `https://github.com/[ユーザー名]/[リポジトリ名]/settings/actions`

### 2. リリースノートの編集（任意、各リリース時）

- ブラウザで `https://github.com/clearclown/lala/releases` を確認
- 自動生成されたリリースノートを編集したい場合のみ手動で編集

**それ以外は全てCLI（gh + git）で完結します！**

---

## 🛠️ 前提条件

### 1. GitHub CLIのインストール

```bash
# すでにインストール済みか確認
gh --version

# 未インストールの場合
# Debian/Ubuntu:
sudo apt install gh

# Arch:
sudo pacman -S github-cli

# Fedora:
sudo dnf install gh
```

### 2. GitHub CLIの認証

```bash
gh auth login
# GitHub.com を選択
# HTTPS を選択
# ブラウザで認証
```

---

## 🚀 リリース手順（自動化）

### Step 1: バージョンの更新

```bash
# Cargo.toml のバージョンを更新
# version = "0.1.0" → version = "0.1.1"

# 自動更新（cargo-edit使用）
cargo install cargo-edit
cargo set-version 0.1.1
```

### Step 2: CHANGELOGの作成/更新

```bash
# CHANGELOG.mdに変更内容を追加
# フォーマット例:
# ## [0.1.1] - 2025-01-XX
# ### Added
# - 新機能の説明
# ### Changed
# - 変更内容の説明
# ### Fixed
# - バグ修正の説明
```

### Step 3: コミットとプッシュ

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: Bump version to 0.1.1"
git push origin main  # または適切なブランチ名
```

### Step 4: タグの作成とプッシュ

```bash
# アノテーション付きタグを作成
git tag -a v0.1.1 -m "Release version 0.1.1"

# タグをプッシュ（これがGitHub Actionsをトリガーします）
git push origin v0.1.1
```

**GitHub Actionsが自動的に**:
1. 複数プラットフォーム向けにビルド
2. バイナリを圧縮（tar.gz / zip）
3. SHA256チェックサムを生成
4. リリースを作成
5. アセットをアップロード
6. CHANGELOGからリリースノートを自動生成

### Step 5: ビルド状況の確認

```bash
# ブラウザでActions画面を開く
gh run list --workflow=release.yml
gh run watch

# または直接ブラウザで確認
gh run view --web
```

### Step 6: リリースの確認

```bash
# リリースページをブラウザで開く
gh release view v0.1.1 --web

# コマンドラインで確認
gh release view v0.1.1
```

---

## 📦 ビルドされるプラットフォーム

現在のワークフローでは以下のプラットフォーム向けにビルドされます：

- **Linux x86_64 (GNU)**: `lala-linux-x86_64.tar.gz`
- **Linux x86_64 (musl)**: `lala-linux-x86_64-musl.tar.gz`
- **macOS x86_64**: `lala-macos-x86_64.tar.gz`
- **macOS ARM64 (Apple Silicon)**: `lala-macos-aarch64.tar.gz`
- **Windows x86_64**: `lala-windows-x86_64.zip`

各アセットには `.sha256` チェックサムファイルも含まれます。

---

## 🔄 アップデート手順

新しいバージョンをリリースする場合：

```bash
# 1. バージョン番号を更新
cargo set-version 0.1.2

# 2. CHANGELOGに変更内容を追加
# CHANGELOG.mdを編集

# 3. コミットとプッシュ
git add Cargo.toml CHANGELOG.md
git commit -m "chore: Bump version to 0.1.2"
git push origin main

# 4. タグを作成してプッシュ
git tag -a v0.1.2 -m "Release version 0.1.2"
git push origin v0.1.2
```

---

## 🚨 トラブルシューティング

### エラー: "insufficient permission for adding an asset to a release"

**原因**: GitHub Actionsの権限不足

**解決策**:
1. リポジトリの Settings → Actions → General
2. "Workflow permissions" で "Read and write permissions" を選択
3. Save

### エラー: GitHub Actions でビルドが失敗

**確認項目**:
```bash
# ローカルでビルドテスト
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-msvc
```

**ログ確認**:
```bash
gh run list --workflow=release.yml
gh run view <run-id> --log-failed
```

### リリースが作成されない

**確認項目**:
- タグ名が `v*` 形式か確認（例: `v0.1.0`）
- ワークフローファイルが `.github/workflows/release.yml` に存在するか確認
- GitHub Actionsの実行ログを確認

---

## 📚 参考リンク

- [GitHub Actions - Workflow syntax](https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions)
- [GitHub CLI - Release commands](https://cli.github.com/manual/gh_release)
- [Semantic Versioning](https://semver.org/)

---

## ✅ チェックリスト

リリースが完了したら、以下を確認：

- [ ] GitHub Actions権限が設定されている（Read and write permissions）
- [ ] タグをプッシュするとビルドが自動実行される
- [ ] リリースページが作成される
- [ ] すべてのプラットフォーム向けバイナリがアップロードされる
- [ ] SHA256チェックサムファイルが含まれている
- [ ] リリースノートがCHANGELOGから自動生成されている
- [ ] ダウンロードして実行できる
