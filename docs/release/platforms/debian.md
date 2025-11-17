# 📦 Debian/Ubuntu (.deb) リリースガイド

**難易度**: ★★★☆☆ (中級)
**推定時間**: 2-3時間
**優先度**: Phase 2

---

## 🖱️ GUI操作が必須の部分

以下の操作は**ブラウザでの手動操作が必要**です：

1. **Launchpadアカウント作成**（PPAを使う場合のみ、5分）
   - https://launchpad.net/ にアクセス
   - Ubuntu Oneアカウントで登録
   - GPGキーを登録（`gpg --gen-key` で生成後、Webで登録）
   - PPAを作成

**基本的な.deb配布（GitHub Releases経由）は全てCLIで完結します！**
**PPAは高度な機能でオプションです。**

---

## 🎯 概要

.debパッケージを作成すると、Debian/Ubuntu系のLinuxディストリビューションで`apt install`できるようになります。

### メリット
- Ubuntu/Debianユーザーに馴染みがある
- 依存関係の自動管理
- システム統合（アイコン、デスクトップエントリ）
- アップデート管理が簡単

---

## 📋 前提条件

### 1. cargo-debのインストール

```bash
cargo install cargo-deb
```

### 2. 必要なツールのインストール

```bash
sudo apt update
sudo apt install dpkg-dev debhelper
```

---

## 🏗️ Cargo.tomlの設定

### Step 1: debメタデータの追加

Cargo.tomlに以下のセクションを追加：

```toml
[package.metadata.deb]
# パッケージの基本情報
maintainer = "Your Name <your.email@example.com>"
copyright = "2025, Your Name <your.email@example.com>"
license-file = ["LICENSE-MIT", "2"]
extended-description = """
Lala is a modern, lightweight text editor written in Rust.
It supports multiple formats including Markdown, HTML, LaTeX, and Mermaid.
Features include syntax highlighting, AI integration, and IME support."""

# 依存関係（Debian/Ubuntuのパッケージ名）
depends = "$auto"

# セクション（カテゴリ）
section = "editors"
priority = "optional"

# インストールするファイル
assets = [
    ["target/release/lala", "usr/bin/", "755"],
    ["README.md", "usr/share/doc/lala/", "644"],
    ["docs/readmeLangs/README_en.md", "usr/share/doc/lala/", "644"],
]

# デスクトップ統合（GUIアプリの場合）
[package.metadata.deb.variants.gui]
assets = [
    ["target/release/lala", "usr/bin/", "755"],
    ["README.md", "usr/share/doc/lala/", "644"],
    ["lala.desktop", "usr/share/applications/", "644"],
    ["assets/icon.png", "usr/share/pixmaps/lala.png", "644"],
]
```

### Step 2: デスクトップエントリの作成

```bash
cat > lala.desktop << 'EOF'
[Desktop Entry]
Name=Lala
GenericName=Text Editor
Comment=Modern multi-format text editor
Exec=lala %F
Icon=lala
Terminal=false
Type=Application
Categories=Utility;TextEditor;
MimeType=text/plain;text/markdown;text/html;
Keywords=editor;text;markdown;
EOF
```

### Step 3: アイコンの準備

```bash
# アイコン用ディレクトリを作成
mkdir -p assets

# アイコンファイルを配置（PNG形式、推奨サイズ: 128x128）
# assets/icon.png に配置
```

---

## 🚀 .debパッケージのビルド

### Step 1: リリースビルド

```bash
# リリースビルドを実行
cargo build --release
```

### Step 2: .debパッケージの生成

```bash
# パッケージを生成
cargo deb

# 生成されたパッケージの確認
ls -lh target/debian/*.deb
# 出力例: lala_0.1.0_amd64.deb
```

### Step 3: パッケージ内容の確認

```bash
# パッケージの詳細情報を確認
dpkg-deb --info target/debian/lala_0.1.0_amd64.deb

# パッケージに含まれるファイル一覧
dpkg-deb --contents target/debian/lala_0.1.0_amd64.deb
```

---

## 🧪 ローカルテスト

### Step 1: インストールテスト

```bash
# .debパッケージをインストール
sudo dpkg -i target/debian/lala_0.1.0_amd64.deb

# 依存関係のエラーがある場合
sudo apt --fix-broken install
```

### Step 2: 動作確認

```bash
# バージョン確認
lala --version

# 実行テスト
lala

# デスクトップエントリの確認（GUI版の場合）
gtk-launch lala
```

### Step 3: アンインストールテスト

```bash
# アンインストール
sudo apt remove lala

# 完全削除（設定ファイルも含む）
sudo apt purge lala
```

---

## 📤 配布方法

### 方法1: GitHub Releasesで配布

```bash
# GitHub Releasesにアップロード
gh release upload v0.1.0 target/debian/lala_0.1.0_amd64.deb
```

**ユーザーへの案内**:
```markdown
## Installation on Debian/Ubuntu

### From .deb package

```bash
# Download the .deb package
wget https://github.com/clearclown/lala/releases/download/v0.1.0/lala_0.1.0_amd64.deb

# Install
sudo dpkg -i lala_0.1.0_amd64.deb

# Fix dependencies if needed
sudo apt --fix-broken install
```
```

### 方法2: PPA (Personal Package Archive) の作成

より高度な方法として、Ubuntu用のPPAを作成できます。

#### Step 1: Launchpadアカウントの作成

1. [Launchpad](https://launchpad.net/)でアカウント作成
2. GPGキーを登録
3. PPAを作成

#### Step 2: ソースパッケージの作成

```bash
# debianディレクトリを作成
mkdir -p debian

# debian/control を作成
cat > debian/control << 'EOF'
Source: lala
Section: editors
Priority: optional
Maintainer: Your Name <your.email@example.com>
Build-Depends: debhelper (>= 11), cargo, rustc
Standards-Version: 4.5.0
Homepage: https://github.com/clearclown/lala

Package: lala
Architecture: any
Depends: ${shlibs:Depends}, ${misc:Depends}
Description: Modern multi-format text editor
 Lala is a modern, lightweight text editor written in Rust.
 It supports multiple formats including Markdown, HTML, LaTeX, and Mermaid.
EOF

# debian/rules を作成
cat > debian/rules << 'EOF'
#!/usr/bin/make -f

%:
	dh $@

override_dh_auto_build:
	cargo build --release

override_dh_auto_install:
	install -D -m 755 target/release/lala $(CURDIR)/debian/lala/usr/bin/lala
EOF

chmod +x debian/rules

# debian/changelog を作成
cat > debian/changelog << 'EOF'
lala (0.1.0-1) unstable; urgency=low

  * Initial release

 -- Your Name <your.email@example.com>  Mon, 01 Jan 2025 00:00:00 +0000
EOF
```

#### Step 3: PPAにアップロード

```bash
# ソースパッケージをビルド
debuild -S -sa

# PPAにアップロード
dput ppa:yourname/lala ../lala_0.1.0-1_source.changes
```

---

## 🔄 バージョンアップデート

新しいバージョンをリリースする場合：

### Step 1: バージョン更新

```bash
# Cargo.toml のバージョンを更新
cargo set-version 0.1.1
```

### Step 2: 再ビルド

```bash
# リリースビルド
cargo build --release

# .debパッケージ生成
cargo deb
```

### Step 3: GitHub Releasesにアップロード

```bash
gh release upload v0.1.1 target/debian/lala_0.1.1_amd64.deb
```

---

## 📊 高度な設定

### カスタムスクリプトの追加

パッケージのインストール/アンインストール時にスクリプトを実行できます：

```bash
# debian/postinst - インストール後のスクリプト
cat > debian/postinst << 'EOF'
#!/bin/sh
set -e

# デスクトップデータベースの更新
if [ -x /usr/bin/update-desktop-database ]; then
    update-desktop-database -q /usr/share/applications
fi

exit 0
EOF

chmod +x debian/postinst

# debian/postrm - アンインストール後のスクリプト
cat > debian/postrm << 'EOF'
#!/bin/sh
set -e

# デスクトップデータベースの更新
if [ -x /usr/bin/update-desktop-database ]; then
    update-desktop-database -q /usr/share/applications
fi

exit 0
EOF

chmod +x debian/postrm
```

Cargo.tomlに追加：
```toml
[package.metadata.deb]
maintainer-scripts = "debian/"
```

---

## 🚨 トラブルシューティング

### エラー: "dpkg-deb: error: failed to open package info file"

**原因**: debianディレクトリの設定が不完全

**解決策**:
```bash
# cargo-debを使用する（自動的に生成）
cargo deb

# 手動でdebianディレクトリを作成する場合は上記の設定を参照
```

### エラー: "dependency problems prevent configuration"

**原因**: 依存関係が満たされていない

**解決策**:
```bash
# 依存関係を自動修正
sudo apt --fix-broken install

# または、Cargo.tomlのdependsを確認
```

### 警告: "W: lala: binary-without-manpage"

**原因**: manページが含まれていない

**解決策**:
```bash
# manページを作成（オプション）
mkdir -p docs/man
# man/lala.1 を作成

# Cargo.tomlに追加
[package.metadata.deb]
assets = [
    # ...
    ["docs/man/lala.1", "usr/share/man/man1/", "644"],
]
```

### エラー: "Architecture is set to 'any' but no binaries were found"

**原因**: リリースビルドが実行されていない

**解決策**:
```bash
cargo build --release
cargo deb
```

---

## 📚 参考リンク

- [cargo-deb Documentation](https://github.com/kornelski/cargo-deb)
- [Debian Policy Manual](https://www.debian.org/doc/debian-policy/)
- [Ubuntu Packaging Guide](https://packaging.ubuntu.com/html/)
- [Launchpad PPA Guide](https://help.launchpad.net/Packaging/PPA)

---

## ✅ 完了チェック

Debianパッケージのリリースが完了したら:

- [ ] Cargo.tomlにdebメタデータが設定されている
- [ ] lala.desktopファイルが作成されている
- [ ] アイコンファイルが配置されている
- [ ] `cargo deb` でパッケージが生成できる
- [ ] `sudo dpkg -i` でインストールできる
- [ ] インストールしたバイナリが動作する
- [ ] GitHub Releasesに.debファイルがアップロードされている
- [ ] READMEにインストール手順が記載されている
- [ ] todo.md の「Debian (.deb)」にチェックを入れる

---

**次のステップ**: [arch.md](./arch.md) でArch Linuxパッケージを作成してください。
