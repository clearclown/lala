# 🎁 Snap Store リリースガイド

**難易度**: ★★★★☆ (上級)
**推定時間**: 3-5時間
**優先度**: Phase 3

---

## 🎯 概要

Snapは Canonical が開発したユニバーサルパッケージ形式で、Ubuntu公式ストアで配布できます。
すべてのLinuxディストリビューションで動作しますが、特にUbuntuで人気があります。

### メリット
- Ubuntu公式ストアで配布
- 自動アップデート
- サンドボックス環境
- 複数バージョンの同時インストール（tracks/channels）

---

## 📋 前提条件

### 1. snapdとsnapcraftのインストール

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install snapd snapcraft

# Fedora
sudo dnf install snapd
sudo ln -s /var/lib/snapd/snap /snap

# Arch
yay -S snapd
sudo systemctl enable --now snapd.socket
sudo ln -s /var/lib/snapd/snap /snap
```

### 2. Snap Storeアカウントの作成

1. [Snap Store](https://snapcraft.io/)にアクセス
2. Ubuntu Oneアカウントでログイン
3. Developer dashboard にアクセス

### 3. snapcraftでログイン

```bash
snapcraft login
# Ubuntu Oneアカウントでログイン
```

---

## 🏗️ snapcraft.yamlの作成

### Step 1: プロジェクト初期化

```bash
# プロジェクトルートで
cd /path/to/lala
mkdir -p snap
```

### Step 2: snapcraft.yamlの作成

```bash
cat > snap/snapcraft.yaml << 'EOF'
name: lala
base: core22
version: '0.1.0'
summary: Modern, lightweight multi-format text editor
description: |
  Lala is a modern text editor written in Rust that supports multiple
  formats including Markdown, HTML, LaTeX, and Mermaid diagrams.

  Features:
  - Syntax highlighting for code blocks
  - Real-time preview for Markdown and other formats
  - AI integration with Gemini API (optional)
  - IME support for Japanese and other languages
  - Light and dark themes
  - Fast and lightweight (<100ms startup)

grade: stable
confinement: strict

# アプリケーション
apps:
  lala:
    command: bin/lala
    extensions: [gnome]
    plugs:
      - home
      - network
      - x11
      - wayland
      - opengl
      - desktop
      - desktop-legacy

# ビルド設定
parts:
  lala:
    plugin: rust
    source: .
    build-packages:
      - libssl-dev
      - pkg-config
    stage-packages:
      - libssl3

  # デスクトップ統合
  desktop-file:
    plugin: dump
    source: .
    organize:
      lala.desktop: usr/share/applications/lala.desktop
      assets/icon.png: usr/share/pixmaps/lala.png

# アーキテクチャ
architectures:
  - build-on: amd64
  - build-on: arm64
EOF
```

### Step 3: デスクトップファイルの準備

```bash
# lala.desktop（すでに作成済みの場合はスキップ）
cat > lala.desktop << 'EOF'
[Desktop Entry]
Name=Lala
GenericName=Text Editor
Comment=Modern multi-format text editor
Exec=lala %F
Icon=${SNAP}/usr/share/pixmaps/lala.png
Terminal=false
Type=Application
Categories=Utility;TextEditor;
MimeType=text/plain;text/markdown;text/html;
Keywords=editor;text;markdown;
EOF

# アイコンを assets/icon.png に配置
```

---

## 🧪 ローカルビルドとテスト

### Step 1: ビルド

```bash
# Snapをビルド
snapcraft

# 初回ビルドは依存関係のダウンロードで時間がかかります
# 生成されたファイル: lala_0.1.0_amd64.snap
```

### Step 2: ローカルインストール

```bash
# 危険モードでインストール（署名なし）
sudo snap install lala_0.1.0_amd64.snap --dangerous

# devmodeでインストール（confinement無効）
sudo snap install lala_0.1.0_amd64.snap --dangerous --devmode
```

### Step 3: 実行テスト

```bash
# アプリを実行
lala

# Snapの情報を確認
snap info lala

# ログを確認
snap logs lala
```

### Step 4: アンインストール

```bash
sudo snap remove lala
```

---

## 🚀 Snap Storeへの公開

### Step 1: Snap名の登録

```bash
# Snap名を登録（他のユーザーが使用できなくなる）
snapcraft register lala
```

### Step 2: Snapのアップロード

```bash
# Snapをビルド（リリース版）
snapcraft clean
snapcraft

# Snap Storeにアップロード
snapcraft upload lala_0.1.0_amd64.snap
```

### Step 3: リリースチャンネルの設定

Snapには4つのチャンネルがあります：
- **stable**: 安定版（一般ユーザー向け）
- **candidate**: リリース候補
- **beta**: ベータ版
- **edge**: 開発版

```bash
# stableチャンネルにリリース
snapcraft release lala <revision> stable

# 例: revision 1をstableにリリース
snapcraft release lala 1 stable

# または複数チャンネルに同時リリース
snapcraft release lala 1 stable,candidate,beta
```

### Step 4: ストアリスティングの設定

```bash
# Webブラウザでストアダッシュボードにアクセス
xdg-open https://snapcraft.io/lala/listing

# または snapcraft CLIで設定
snapcraft set-description lala description.txt
snapcraft set-icon lala assets/icon.png
```

**description.txt**:
```
Lala is a modern, lightweight text editor written in Rust.

It supports multiple formats including Markdown, HTML, LaTeX, and Mermaid.

Features:
• Syntax highlighting for code blocks
• Real-time preview for Markdown and other formats
• AI integration with Gemini API (optional)
• IME support for Japanese and other languages
• Light and dark themes
• Fast and lightweight (<100ms startup)
```

---

## 📊 ユーザーへの案内

READMEに以下のインストール手順を追加：

```markdown
## Installation from Snap Store

### Ubuntu and other Linux distributions

```bash
# Install from Snap Store
sudo snap install lala

# Run
lala
```

### Upgrade
```bash
# Automatic updates are enabled by default
# Manual upgrade:
sudo snap refresh lala
```

### Uninstall
```bash
sudo snap remove lala
```

### Different channels
```bash
# Install beta version
sudo snap install lala --channel=beta

# Install edge (development) version
sudo snap install lala --channel=edge
```
```

---

## 🔄 バージョンアップデート

### Step 1: バージョン更新

```bash
# snap/snapcraft.yamlのバージョンを更新
version: '0.1.1'

# Cargo.tomlも更新
cargo set-version 0.1.1
```

### Step 2: ビルドとアップロード

```bash
# クリーンビルド
snapcraft clean
snapcraft

# アップロード
snapcraft upload lala_0.1.1_amd64.snap

# リリース
snapcraft release lala <revision> stable
```

---

## 📊 高度な設定

### 複数アーキテクチャのビルド

```yaml
architectures:
  - build-on: amd64
    build-for: amd64
  - build-on: arm64
    build-for: arm64
  - build-on: armhf
    build-for: armhf
```

### リモートビルド（Launchpad使用）

ローカルで他アーキテクチャをビルドできない場合：

```bash
# Launchpadでリモートビルド
snapcraft remote-build

# すべてのアーキテクチャが自動的にビルドされる
```

### Tracks（複数メジャーバージョン）

異なるメジャーバージョンを同時提供：

```bash
# Track作成リクエスト（Snap Storeチームに連絡）
# 例: v0.1, v0.2 など

# 特定trackにリリース
snapcraft release lala 1 v0.1/stable
snapcraft release lala 5 v0.2/stable
```

### Confinementモード

```yaml
# strict: 完全なサンドボックス（推奨）
confinement: strict

# classic: サンドボックスなし（要申請）
confinement: classic

# devmode: 開発用
confinement: devmode
```

**classic confinementの申請**:
```bash
# Forum投稿が必要
# https://forum.snapcraft.io/t/process-for-reviewing-classic-confinement-snaps/
```

### 環境変数の設定

```yaml
apps:
  lala:
    command: bin/lala
    environment:
      RUST_LOG: info
      LALA_CONFIG: $SNAP_USER_DATA
```

---

## 🚨 トラブルシューティング

### エラー: "The name 'lala' is already taken"

**原因**: 名前がすでに登録されている

**解決策**:
```bash
# 別の名前を使用
name: lala-editor

# または現在の所有者に連絡して譲渡を依頼
```

### エラー: "Build failed: cargo not found"

**原因**: Rust pluginの設定不足

**解決策**:
```yaml
parts:
  lala:
    plugin: rust
    rust-channel: stable  # 追加
```

### エラー: "Permission denied" when running snap

**原因**: confinementの制限

**解決策**:
```bash
# snapの接続状況を確認
snap connections lala

# 必要な権限を手動で接続
sudo snap connect lala:home
sudo snap connect lala:network

# またはsnapcraft.yamlのplugsを確認
```

### 警告: "desktop integration not found"

**原因**: desktop helperが不足

**解決策**:
```yaml
apps:
  lala:
    extensions: [gnome]  # またはkdeまたはgnome-3-38
```

### ビルドが遅い

**解決策**:
```bash
# LXDを使用（Multipass より高速）
sudo snap install lxd
sudo lxd init --auto
snapcraft --use-lxd

# またはDockerを使用
snapcraft --destructive-mode
```

---

## 📚 参考リンク

- [Snapcraft Documentation](https://snapcraft.io/docs)
- [Rust Plugin](https://snapcraft.io/docs/rust-plugin)
- [Snap Store Publishing](https://snapcraft.io/docs/releasing-your-app)
- [Classic Confinement](https://snapcraft.io/docs/classic-confinement)

---

## ✅ 完了チェック

Snapリリースが完了したら：

- [ ] snapcraft.yamlが作成されている
- [ ] デスクトップファイルとアイコンが準備されている
- [ ] ローカルで`snapcraft`が成功する
- [ ] ローカルでインストールして動作確認できる
- [ ] Snap Storeに名前が登録されている
- [ ] Snapがアップロードされている
- [ ] stableチャンネルにリリースされている
- [ ] ストアリスティングが設定されている
- [ ] READMEにインストール手順が記載されている
- [ ] todo.md の「Snap Store」にチェックを入れる

---

**次のステップ**: [windows.md](./windows.md) でWindowsインストーラーを作成してください。
