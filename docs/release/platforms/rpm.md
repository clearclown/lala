# 🎩 Fedora/RHEL (.rpm) リリースガイド

**難易度**: ★★★☆☆ (中級)
**推定時間**: 2-3時間
**優先度**: Phase 2

---

## 🖱️ GUI操作が必須の部分

以下の操作は**ブラウザでの手動操作が必要**です：

1. **COPRアカウント作成**（COPRを使う場合のみ、3分）
   - https://copr.fedorainfracloud.org/ にアクセス
   - Fedoraアカウントでログイン
   - 新しいプロジェクトを作成
   - ビルド設定（chroot選択）

**基本的な.rpm配布（GitHub Releases経由）は全てCLIで完結します！**
**COPRは高度な機能でオプションです。**

---

## 🎯 概要

.rpmパッケージを作成すると、Fedora/RHEL/CentOS/openSUSEなどで`dnf install`または`yum install`できるようになります。

### メリット
- Red Hat系ディストリビューションユーザーにリーチ
- 企業環境で広く使われている
- 依存関係の自動管理
- システム統合

---

## 📋 前提条件

### 1. cargo-generate-rpmのインストール

```bash
cargo install cargo-generate-rpm
```

### 2. 必要なツールのインストール

```bash
# Fedora/RHEL/CentOS
sudo dnf install rpm-build rpmdevtools

# openSUSE
sudo zypper install rpm-build
```

---

## 🏗️ Cargo.tomlの設定

### Step 1: RPMメタデータの追加

Cargo.tomlに以下のセクションを追加：

```toml
[package.metadata.generate-rpm]
# パッケージの基本情報
name = "lala"
summary = "Modern, lightweight multi-format text editor"
license = "MIT OR Apache-2.0"

# アセット（インストールするファイル）
assets = [
    { source = "target/release/lala", dest = "/usr/bin/lala", mode = "755" },
    { source = "README.md", dest = "/usr/share/doc/lala/README.md", mode = "644" },
    { source = "LICENSE-MIT", dest = "/usr/share/licenses/lala/LICENSE-MIT", mode = "644" },
    { source = "LICENSE-APACHE", dest = "/usr/share/licenses/lala/LICENSE-APACHE", mode = "644" },
]

# 依存関係（オプション）
# requires = { package-name = "version" }

# GUIアプリケーションの場合、デスクトップエントリも追加
# assets = [
#     # ... 上記のアセット
#     { source = "lala.desktop", dest = "/usr/share/applications/lala.desktop", mode = "644" },
#     { source = "assets/icon.png", dest = "/usr/share/pixmaps/lala.png", mode = "644" },
# ]

# 追加のメタデータ
[package.metadata.generate-rpm.metadata]
description = """
Lala is a modern, lightweight text editor written in Rust.
It supports multiple formats including Markdown, HTML, LaTeX, and Mermaid.
Features include syntax highlighting, AI integration, and IME support.
"""

# スクリプトレット（オプション）
[package.metadata.generate-rpm.scripts]
# post_install = "scripts/post-install.sh"
# post_uninstall = "scripts/post-uninstall.sh"
```

### Step 2: デスクトップエントリの作成（GUI版の場合）

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

---

## 🚀 .rpmパッケージのビルド

### Step 1: リリースビルド

```bash
# リリースビルドを実行
cargo build --release
```

### Step 2: .rpmパッケージの生成

```bash
# パッケージを生成
cargo generate-rpm

# 生成されたパッケージの確認
ls -lh target/generate-rpm/*.rpm
# 出力例: lala-0.1.0-1.x86_64.rpm
```

### Step 3: パッケージ内容の確認

```bash
# パッケージの詳細情報を確認
rpm -qip target/generate-rpm/lala-0.1.0-1.x86_64.rpm

# パッケージに含まれるファイル一覧
rpm -qlp target/generate-rpm/lala-0.1.0-1.x86_64.rpm
```

---

## 🧪 ローカルテスト

### Step 1: インストールテスト

```bash
# .rpmパッケージをインストール
sudo rpm -ivh target/generate-rpm/lala-0.1.0-1.x86_64.rpm

# または dnf/yum を使用
sudo dnf install target/generate-rpm/lala-0.1.0-1.x86_64.rpm
```

### Step 2: 動作確認

```bash
# バージョン確認
lala --version

# 実行テスト
lala

# インストールされたファイルの確認
rpm -ql lala
```

### Step 3: アンインストールテスト

```bash
# アンインストール
sudo rpm -e lala

# または dnf/yum を使用
sudo dnf remove lala
```

---

## 📤 配布方法

### 方法1: GitHub Releasesで配布

```bash
# GitHub Releasesにアップロード
gh release upload v0.1.0 target/generate-rpm/lala-0.1.0-1.x86_64.rpm
```

**ユーザーへの案内**:
```markdown
## Installation on Fedora/RHEL/CentOS

### From .rpm package

```bash
# Download the .rpm package
wget https://github.com/clearclown/lala/releases/download/v0.1.0/lala-0.1.0-1.x86_64.rpm

# Install
sudo dnf install lala-0.1.0-1.x86_64.rpm

# Or using rpm directly
sudo rpm -ivh lala-0.1.0-1.x86_64.rpm
```
```

### 方法2: COPR (Community Projects) の使用

Fedora向けにCOPRリポジトリを作成できます（Ubuntu PPAに相当）。

#### Step 1: COPRアカウントの作成

1. [COPR](https://copr.fedorainfracloud.org/)にアクセス
2. Fedoraアカウントでログイン
3. 新しいプロジェクトを作成

#### Step 2: .specファイルの作成

```bash
# specファイル用ディレクトリを作成
mkdir -p ~/rpmbuild/{SPECS,SOURCES}

# lala.specを作成
cat > ~/rpmbuild/SPECS/lala.spec << 'EOF'
Name:           lala
Version:        0.1.0
Release:        1%{?dist}
Summary:        Modern, lightweight multi-format text editor

License:        MIT OR Apache-2.0
URL:            https://github.com/clearclown/lala
Source0:        https://github.com/clearclown/%{name}/archive/refs/tags/v%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo

%description
Lala is a modern, lightweight text editor written in Rust.
It supports multiple formats including Markdown, HTML, LaTeX, and Mermaid.

%prep
%autosetup

%build
cargo build --release --locked

%install
install -Dm755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -Dm644 README.md %{buildroot}%{_docdir}/%{name}/README.md
install -Dm644 LICENSE-MIT %{buildroot}%{_licensedir}/%{name}/LICENSE-MIT
install -Dm644 LICENSE-APACHE %{buildroot}%{_licensedir}/%{name}/LICENSE-APACHE

%files
%{_bindir}/%{name}
%{_docdir}/%{name}/README.md
%{_licensedir}/%{name}/LICENSE-MIT
%{_licensedir}/%{name}/LICENSE-APACHE

%changelog
* Mon Jan 01 2025 Your Name <your.email@example.com> - 0.1.0-1
- Initial package
EOF
```

#### Step 3: ソースRPMのビルド

```bash
# ソースtarballをダウンロード
cd ~/rpmbuild/SOURCES
wget https://github.com/clearclown/lala/archive/refs/tags/v0.1.0.tar.gz

# ソースRPMをビルド
cd ~/rpmbuild/SPECS
rpmbuild -bs lala.spec
```

#### Step 4: COPRにアップロード

```bash
# COPR CLIをインストール
sudo dnf install copr-cli

# COPRトークンを設定
copr-cli create lala --chroot fedora-39-x86_64 --description "Lala text editor"

# ソースRPMをアップロード
copr-cli build lala ~/rpmbuild/SRPMS/lala-0.1.0-1.fc39.src.rpm
```

**ユーザーへの案内（COPR使用時）**:
```markdown
## Installation from COPR

```bash
# Enable the COPR repository
sudo dnf copr enable yourname/lala

# Install
sudo dnf install lala
```
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

# .rpmパッケージ生成
cargo generate-rpm
```

### Step 3: GitHub Releasesにアップロード

```bash
gh release upload v0.1.1 target/generate-rpm/lala-0.1.1-1.x86_64.rpm
```

### Step 4: COPR更新（COPR使用時）

```bash
# .specファイルのバージョンを更新
# Version: 0.1.1

# %changelog にエントリを追加
cat >> ~/rpmbuild/SPECS/lala.spec << 'EOF'
* Mon Jan 15 2025 Your Name <your.email@example.com> - 0.1.1-1
- Update to version 0.1.1
- Bug fixes and improvements
EOF

# ソースRPMを再ビルド
rpmbuild -bs ~/rpmbuild/SPECS/lala.spec

# COPRに再アップロード
copr-cli build lala ~/rpmbuild/SRPMS/lala-0.1.1-1.fc39.src.rpm
```

---

## 📊 高度な設定

### インストールスクリプト

パッケージのインストール/アンインストール時にスクリプトを実行：

```bash
# scripts/post-install.sh
cat > scripts/post-install.sh << 'EOF'
#!/bin/sh
# デスクトップデータベースの更新
if [ -x /usr/bin/update-desktop-database ]; then
    /usr/bin/update-desktop-database -q /usr/share/applications
fi
exit 0
EOF

chmod +x scripts/post-install.sh

# scripts/post-uninstall.sh
cat > scripts/post-uninstall.sh << 'EOF'
#!/bin/sh
# デスクトップデータベースの更新
if [ -x /usr/bin/update-desktop-database ]; then
    /usr/bin/update-desktop-database -q /usr/share/applications
fi
exit 0
EOF

chmod +x scripts/post-uninstall.sh
```

Cargo.tomlに追加：
```toml
[package.metadata.generate-rpm.scripts]
post_install = "scripts/post-install.sh"
post_uninstall = "scripts/post-uninstall.sh"
```

---

## 🚨 トラブルシューティング

### エラー: "failed to read assets"

**原因**: assetsで指定したファイルが存在しない

**解決策**:
```bash
# リリースビルドが完了しているか確認
cargo build --release
ls target/release/lala

# パスが正しいか確認
```

### エラー: "No such file or directory: target/generate-rpm"

**原因**: ディレクトリが作成されていない

**解決策**:
```bash
# ディレクトリを手動で作成
mkdir -p target/generate-rpm

# 再度生成
cargo generate-rpm
```

### エラー: "package lala is already installed"

**原因**: すでにインストールされている

**解決策**:
```bash
# アップグレードモードでインストール
sudo rpm -Uvh target/generate-rpm/lala-0.1.0-1.x86_64.rpm

# または先にアンインストール
sudo rpm -e lala
sudo rpm -ivh target/generate-rpm/lala-0.1.0-1.x86_64.rpm
```

### エラー: "Failed dependencies"

**原因**: 依存関係が満たされていない

**解決策**:
```bash
# dnf/yumは依存関係を自動解決
sudo dnf install target/generate-rpm/lala-0.1.0-1.x86_64.rpm

# Cargo.tomlのrequiresセクションを確認
```

---

## 📚 参考リンク

- [cargo-generate-rpm Documentation](https://github.com/cat-in-136/cargo-generate-rpm)
- [RPM Packaging Guide](https://rpm-packaging-guide.github.io/)
- [Fedora Packaging Guidelines](https://docs.fedoraproject.org/en-US/packaging-guidelines/)
- [COPR User Documentation](https://docs.pagure.org/copr.copr/user_documentation.html)

---

## ✅ 完了チェック

RPMパッケージのリリースが完了したら：

- [ ] Cargo.tomlにRPMメタデータが設定されている
- [ ] `cargo generate-rpm` でパッケージが生成できる
- [ ] `sudo rpm -ivh` でインストールできる
- [ ] インストールしたバイナリが動作する
- [ ] GitHub Releasesに.rpmファイルがアップロードされている
- [ ] READMEにインストール手順が記載されている
- [ ] （オプション）COPRリポジトリが作成されている
- [ ] todo.md の「Fedora/RHEL (.rpm)」にチェックを入れる

---

**次のステップ**: Phase 2完了！Phase 3として [flatpak.md](./flatpak.md) に進んでください。
