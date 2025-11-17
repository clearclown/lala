# 🎯 GitHub Releases リリースガイド

**難易度**: ★★☆☆☆ (簡単)
**推定時間**: 1-2時間（初回は設定に時間がかかる）
**優先度**: ⭐ 最優先

---

## 🎯 概要

GitHub Releasesを使用すると、コンパイル済みのバイナリを複数のプラットフォーム向けに配布できます。
ユーザーはRustツールチェーンなしで直接ダウンロードして実行できます。

### メリット
- Rustをインストールしていないユーザーでも使える
- 複数プラットフォーム対応（Windows, Linux, macOS）
- ダウンロード数の統計が見られる
- リリースノートで変更内容を明確に伝えられる

---

## 📋 前提条件

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

### 2. 認証

```bash
gh auth login
# GitHub.com を選択
# HTTPS を選択
# ブラウザで認証
```

### 3. クロスコンパイルツールのインストール

```bash
# cross - Dockerベースのクロスコンパイルツール
cargo install cross

# または cargo-zigbuild（より高速）
cargo install cargo-zigbuild
```

---

## 🏗️ GitHub Actions自動ビルドの設定

GitHub Actionsを使用して、タグをプッシュするだけで自動的に複数プラットフォーム向けのバイナリをビルド・リリースできます。

### Step 1: ワークフローファイルの作成

```bash
mkdir -p .github/workflows
cat > .github/workflows/release.yml << 'EOF'
name: Release

on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write

jobs:
  create-release:
    name: Create Release
    runs-on: ubuntu-latest
    outputs:
      upload_url: ${{ steps.create_release.outputs.upload_url }}
    steps:
      - name: Create Release
        id: create_release
        uses: actions/create-release@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tag_name: ${{ github.ref_name }}
          release_name: Release ${{ github.ref_name }}
          draft: false
          prerelease: false

  build-release:
    name: Build Release
    needs: create-release
    strategy:
      matrix:
        include:
          # Linux x86_64
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: lala
            asset_name: lala-linux-x86_64

          # Linux ARM64
          - os: ubuntu-latest
            target: aarch64-unknown-linux-gnu
            artifact_name: lala
            asset_name: lala-linux-aarch64

          # macOS x86_64
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact_name: lala
            asset_name: lala-macos-x86_64

          # macOS ARM64 (Apple Silicon)
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact_name: lala
            asset_name: lala-macos-aarch64

          # Windows x86_64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: lala.exe
            asset_name: lala-windows-x86_64.exe

    runs-on: ${{ matrix.os }}

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Strip binary (Linux/macOS)
        if: matrix.os != 'windows-latest'
        run: strip target/${{ matrix.target }}/release/${{ matrix.artifact_name }}

      - name: Upload Release Asset
        uses: actions/upload-release-asset@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          upload_url: ${{ needs.create-release.outputs.upload_url }}
          asset_path: target/${{ matrix.target }}/release/${{ matrix.artifact_name }}
          asset_name: ${{ matrix.asset_name }}
          asset_content_type: application/octet-stream
EOF
```

### Step 2: ワークフローのコミット

```bash
git add .github/workflows/release.yml
git commit -m "ci: Add GitHub Actions release workflow"
git push origin release-1
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
# 初回の場合、CHANGELOGを作成
cat > CHANGELOG.md << 'EOF'
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-XX

### Added
- Modern text editor with GUI
- Multi-format support (Markdown, HTML, LaTeX, Mermaid)
- Syntax highlighting for code blocks
- AI integration with Gemini API (optional)
- IME support for Japanese/CJK input
- Light/Dark theme support
- Search and replace functionality
- Multiple tab support

### Features
- Fast startup (<100ms)
- Lightweight (no Electron)
- Cross-platform (Windows, Linux, macOS)

[0.1.0]: https://github.com/clearclown/lala/releases/tag/v0.1.0
EOF
```

### Step 3: コミットとプッシュ

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: Bump version to 0.1.0"
git push origin release-1
```

### Step 4: タグの作成とプッシュ

```bash
# アノテーション付きタグを作成
git tag -a v0.1.0 -m "Release version 0.1.0"

# タグをプッシュ（これがGitHub Actionsをトリガーします）
git push origin v0.1.0
```

**GitHub Actionsが自動的に**:
1. 複数プラットフォーム向けにビルド
2. バイナリを圧縮
3. リリースを作成
4. アセットをアップロード

### Step 5: ビルド状況の確認

```bash
# ブラウザでActions画面を開く
gh run list --workflow=release.yml
gh run watch

# または
xdg-open https://github.com/clearclown/lala/actions
```

### Step 6: リリースノートの編集

```bash
# リリースが作成されたら、ノートを追加
gh release edit v0.1.0 --notes-file CHANGELOG.md
```

---

## 📝 手動リリース（GitHub Actions使わない場合）

### Step 1: 各プラットフォーム向けにビルド

```bash
# Linux x86_64（現在のシステム）
cargo build --release
cp target/release/lala lala-linux-x86_64

# Windows (クロスコンパイル)
cross build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/lala.exe lala-windows-x86_64.exe

# macOS x86_64 (macOS上で実行する必要あり)
# cargo build --release --target x86_64-apple-darwin
# cp target/x86_64-apple-darwin/release/lala lala-macos-x86_64

# macOS ARM64 (Apple Silicon, macOS上で実行)
# cargo build --release --target aarch64-apple-darwin
# cp target/aarch64-apple-darwin/release/lala lala-macos-aarch64
```

**注意**: macOSバイナリはmacOS上でビルドする必要があります。

### Step 2: バイナリの圧縮

```bash
# Linux/macOS
strip lala-linux-x86_64
gzip lala-linux-x86_64

# Windows
strip lala-windows-x86_64.exe
zip lala-windows-x86_64.zip lala-windows-x86_64.exe
```

### Step 3: GitHub Releaseの作成

```bash
# リリースを作成
gh release create v0.1.0 \
  --title "Lala v0.1.0" \
  --notes-file CHANGELOG.md \
  lala-linux-x86_64.gz \
  lala-windows-x86_64.zip
```

---

## 📊 リリース後の確認

### 1. リリースページの確認

```bash
# ブラウザで確認
gh release view v0.1.0 --web

# コマンドラインで確認
gh release view v0.1.0
```

### 2. ダウンロードテスト

```bash
# 別のマシンまたはディレクトリで
cd /tmp
gh release download v0.1.0 --repo clearclown/lala
gunzip lala-linux-x86_64.gz
chmod +x lala-linux-x86_64
./lala-linux-x86_64 --version
```

### 3. インストール手順の確認

READMEに以下のようなインストール手順を追加：

```markdown
## Installation

### From Binary (Recommended for non-Rust users)

#### Linux
```bash
# Download latest release
curl -L https://github.com/clearclown/lala/releases/latest/download/lala-linux-x86_64.gz -o lala.gz
gunzip lala.gz
chmod +x lala
sudo mv lala /usr/local/bin/
```

#### Windows
1. Download [lala-windows-x86_64.exe](https://github.com/clearclown/lala/releases/latest/download/lala-windows-x86_64.exe)
2. Rename to `lala.exe`
3. Move to a directory in your PATH

#### macOS
```bash
# Intel Macs
curl -L https://github.com/clearclown/lala/releases/latest/download/lala-macos-x86_64 -o lala
chmod +x lala
sudo mv lala /usr/local/bin/

# Apple Silicon (M1/M2)
curl -L https://github.com/clearclown/lala/releases/latest/download/lala-macos-aarch64 -o lala
chmod +x lala
sudo mv lala /usr/local/bin/
```
```

---

## 🔄 アップデート手順

新しいバージョンをリリースする場合：

### 1. バージョンアップ

```bash
# バージョン番号を更新
cargo set-version 0.1.1

# CHANGELOGに変更内容を追加
```

### 2. コミットとタグ

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: Bump version to 0.1.1"
git push origin release-1

git tag -a v0.1.1 -m "Release version 0.1.1"
git push origin v0.1.1
```

GitHub Actionsが自動的にビルドとリリースを行います。

---

## 🚨 トラブルシューティング

### エラー: "cross-compilation requires Docker"

**原因**: crossツールがDockerを必要とする

**解決策**:
```bash
# Dockerをインストール
sudo apt install docker.io
sudo usermod -aG docker $USER
# ログアウト/ログインして再試行
```

### エラー: GitHub Actions でビルドが失敗

**確認項目**:
```bash
# ローカルでビルドテスト
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

**ログ確認**:
```bash
gh run list --workflow=release.yml
gh run view <run-id> --log-failed
```

### エラー: "insufficient permission for adding an asset to a release"

**原因**: GITHUB_TOKENの権限不足

**解決策**:
1. リポジトリの Settings → Actions → General
2. "Workflow permissions" で "Read and write permissions" を選択
3. Save

### バイナリサイズが大きすぎる

**解決策**: Cargo.tomlに最適化設定を追加

```toml
[profile.release]
strip = true        # シンボル情報を削除
lto = true          # Link Time Optimization
codegen-units = 1   # より良い最適化
opt-level = "z"     # サイズ優先の最適化
```

---

## 📚 参考リンク

- [GitHub Actions - Workflow syntax](https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions)
- [cross - Zero setup cross compilation](https://github.com/cross-rs/cross)
- [GitHub CLI - Release commands](https://cli.github.com/manual/gh_release)
- [Rust Cross Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)

---

## ✅ 完了チェック

リリースが完了したら、以下を確認：

- [ ] GitHub Actionsワークフローが作成されている
- [ ] タグをプッシュするとビルドが自動実行される
- [ ] リリースページが作成される
- [ ] すべてのプラットフォーム向けバイナリがアップロードされる
- [ ] ダウンロードして実行できる
- [ ] READMEにインストール手順が記載されている
- [ ] todo.md の「GitHub Releases」にチェックを入れる

---

**次のステップ**: Phase 1が完了したら、Phase 2として [homebrew.md](./homebrew.md) に進んでください。
