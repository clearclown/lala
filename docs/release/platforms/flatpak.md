# 📦 Flatpak リリースガイド

**難易度**: ★★★★☆ (上級)
**推定時間**: 4-6時間
**優先度**: Phase 3

---

## 🎯 概要

Flatpakは、すべてのLinuxディストリビューション向けの統一されたアプリケーション配布形式です。
Flathubに公開すると、すべての主要Linuxディストリビューションのユーザーに簡単にリーチできます。

### メリット
- ディストリビューションに依存しない
- サンドボックス環境で安全
- 依存関係が完全に含まれる
- Flathubストアで統一的に配布

---

## 📋 前提条件

### 1. Flatpak開発環境のセットアップ

```bash
# Flatpakとflatpak-builderをインストール
# Debian/Ubuntu
sudo apt install flatpak flatpak-builder

# Fedora
sudo dnf install flatpak flatpak-builder

# Arch
sudo pacman -S flatpak flatpak-builder

# Flathubリポジトリを追加
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
```

### 2. SDKのインストール

```bash
# Freedesktop SDKをインストール（Rustアプリ向け）
flatpak install flathub org.freedesktop.Platform//23.08
flatpak install flathub org.freedesktop.Sdk//23.08
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//23.08
```

---

## 🏗️ Flatpakマニフェストの作成

### Step 1: アプリIDの決定

Flatpakでは逆ドメイン形式のIDを使用：
- 例: `com.github.clearclown.Lala`

### Step 2: マニフェストファイルの作成

```bash
# プロジェクトルートに作成
cat > com.github.clearclown.Lala.yml << 'EOF'
app-id: com.github.clearclown.Lala
runtime: org.freedesktop.Platform
runtime-version: '23.08'
sdk: org.freedesktop.Sdk
sdk-extensions:
  - org.freedesktop.Sdk.Extension.rust-stable

command: lala

finish-args:
  # ファイルシステムアクセス
  - --filesystem=home
  - --filesystem=xdg-documents

  # X11とWaylandアクセス（GUI用）
  - --socket=x11
  - --socket=wayland

  # GPUアクセス（レンダリング用）
  - --device=dri

  # ネットワークアクセス（AI機能用、オプション）
  - --share=network

modules:
  - name: lala
    buildsystem: simple
    build-options:
      append-path: /usr/lib/sdk/rust-stable/bin
      env:
        CARGO_HOME: /run/build/lala/cargo
    build-commands:
      # Rustでビルド
      - cargo --offline fetch --manifest-path Cargo.toml --verbose
      - cargo --offline build --release --verbose
      - install -Dm755 ./target/release/lala -t /app/bin/
      - install -Dm644 ./lala.desktop -t /app/share/applications/
      - install -Dm644 ./assets/icon.png /app/share/icons/hicolor/128x128/apps/com.github.clearclown.Lala.png
    sources:
      - type: archive
        url: https://github.com/clearclown/lala/archive/refs/tags/v0.1.0.tar.gz
        sha256: YOUR_SHA256_HERE

      # Cargo依存関係のキャッシュ
      - generated-sources.json
EOF
```

### Step 3: Cargo依存関係の生成

Flatpakのオフラインビルドのため、すべての依存関係を事前にダウンロード：

```bash
# flatpak-cargo-generatorをダウンロード
curl -O https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py

# generated-sources.jsonを生成
python3 flatpak-cargo-generator.py ./Cargo.lock -o generated-sources.json
```

### Step 4: デスクトップファイルとアイコンの準備

```bash
# デスクトップファイル（すでに作成済みの場合はスキップ）
cat > lala.desktop << 'EOF'
[Desktop Entry]
Name=Lala
GenericName=Text Editor
Comment=Modern multi-format text editor
Exec=lala %F
Icon=com.github.clearclown.Lala
Terminal=false
Type=Application
Categories=Utility;TextEditor;
MimeType=text/plain;text/markdown;text/html;
Keywords=editor;text;markdown;
EOF

# アイコン（128x128 PNG）を assets/icon.png に配置
```

### Step 5: メタデータファイルの作成（Flathub申請用）

```bash
# AppStream metadataを作成
mkdir -p assets
cat > assets/com.github.clearclown.Lala.metainfo.xml << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>com.github.clearclown.Lala</id>
  <metadata_license>CC0-1.0</metadata_license>
  <project_license>MIT OR Apache-2.0</project_license>
  <name>Lala</name>
  <summary>Modern, lightweight multi-format text editor</summary>

  <description>
    <p>
      Lala is a modern text editor written in Rust that supports multiple formats
      including Markdown, HTML, LaTeX, and Mermaid diagrams.
    </p>
    <p>Features:</p>
    <ul>
      <li>Syntax highlighting for code blocks</li>
      <li>Real-time preview for Markdown and other formats</li>
      <li>AI integration with Gemini API (optional)</li>
      <li>IME support for Japanese and other languages</li>
      <li>Light and dark themes</li>
      <li>Fast and lightweight (<100ms startup)</li>
    </ul>
  </description>

  <launchable type="desktop-id">lala.desktop</launchable>

  <screenshots>
    <screenshot type="default">
      <image>https://raw.githubusercontent.com/clearclown/lala/main/screenshots/main.png</image>
      <caption>Main editor window</caption>
    </screenshot>
  </screenshots>

  <url type="homepage">https://github.com/clearclown/lala</url>
  <url type="bugtracker">https://github.com/clearclown/lala/issues</url>

  <developer_name>Your Name</developer_name>

  <releases>
    <release version="0.1.0" date="2025-01-01">
      <description>
        <p>Initial release</p>
        <ul>
          <li>Multi-format text editor support</li>
          <li>Syntax highlighting</li>
          <li>AI integration</li>
        </ul>
      </description>
    </release>
  </releases>

  <content_rating type="oars-1.1" />
</component>
EOF
```

マニフェストに追加：
```yaml
    build-commands:
      # ... 既存のコマンド
      - install -Dm644 ./assets/com.github.clearclown.Lala.metainfo.xml -t /app/share/metainfo/
```

---

## 🧪 ローカルビルドとテスト

### Step 1: ビルド

```bash
# Flatpakをビルド
flatpak-builder --force-clean build-dir com.github.clearclown.Lala.yml

# リポジトリにエクスポート
flatpak-builder --repo=repo --force-clean build-dir com.github.clearclown.Lala.yml
```

### Step 2: ローカルインストール

```bash
# リポジトリからインストール
flatpak --user remote-add --no-gpg-verify lala-repo repo
flatpak --user install lala-repo com.github.clearclown.Lala
```

### Step 3: 実行テスト

```bash
# アプリを実行
flatpak run com.github.clearclown.Lala

# デバッグモードで実行
flatpak run --command=sh com.github.clearclown.Lala
# シェル内で: /app/bin/lala
```

### Step 4: アンインストール

```bash
flatpak --user uninstall com.github.clearclown.Lala
flatpak --user remote-delete lala-repo
```

---

## 🚀 Flathubへの申請

### Step 1: Flathub GitHubリポジトリをフォーク

```bash
# Flathubのメインリポジトリをフォーク
# https://github.com/flathub/flathub

# 新しいアプリ用のリポジトリを作成リクエスト
# https://github.com/flathub/flathub/new/master
```

### Step 2: アプリリポジトリの作成

```bash
# Flathubから新しいリポジトリが作成されたらクローン
git clone https://github.com/flathub/com.github.clearclown.Lala.git
cd com.github.clearclown.Lala

# マニフェストとファイルをコピー
cp /path/to/lala/com.github.clearclown.Lala.yml .
cp /path/to/lala/generated-sources.json .
cp /path/to/lala/lala.desktop .
cp /path/to/lala/assets/com.github.clearclown.Lala.metainfo.xml .
cp /path/to/lala/assets/icon.png .

# コミット
git add .
git commit -m "Initial commit for Lala"
git push origin master
```

### Step 3: プルリクエストを作成

```bash
# Flathubリポジトリにプルリクエストを作成
# https://github.com/flathub/flathub/compare

# レビューを待つ（通常数日～数週間）
```

---

## 📊 高度な設定

### サンドボックス権限の調整

Flatpakはデフォルトでサンドボックス化されています。必要に応じて権限を調整：

```yaml
finish-args:
  # ファイルシステムアクセス
  - --filesystem=home                # ホームディレクトリ全体
  - --filesystem=xdg-documents       # ドキュメントフォルダのみ
  - --filesystem=xdg-download:ro     # ダウンロードフォルダ（読み取り専用）

  # GUI
  - --socket=x11                     # X11サポート
  - --socket=wayland                 # Waylandサポート
  - --device=dri                     # GPUアクセス

  # ネットワーク
  - --share=network                  # ネットワークアクセス

  # その他
  - --share=ipc                      # プロセス間通信
  - --socket=pulseaudio              # オーディオ（必要な場合）
```

### ビルド最適化

```yaml
build-options:
  env:
    CARGO_PROFILE_RELEASE_LTO: 'true'
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS: '1'
    CARGO_PROFILE_RELEASE_OPT_LEVEL: 'z'
```

---

## 🔄 バージョンアップデート

### Step 1: マニフェストの更新

```yaml
sources:
  - type: archive
    url: https://github.com/clearclown/lala/archive/refs/tags/v0.1.1.tar.gz
    sha256: NEW_SHA256_HERE
```

### Step 2: メタデータの更新

```xml
<releases>
  <release version="0.1.1" date="2025-01-15">
    <description>
      <p>Bug fixes and improvements</p>
    </description>
  </release>
  <!-- 古いリリース -->
</releases>
```

### Step 3: generated-sources.jsonの再生成

```bash
python3 flatpak-cargo-generator.py ./Cargo.lock -o generated-sources.json
```

### Step 4: プッシュ

```bash
git add com.github.clearclown.Lala.yml generated-sources.json assets/com.github.clearclown.Lala.metainfo.xml
git commit -m "Update to version 0.1.1"
git push origin master
```

---

## 🚨 トラブルシューティング

### エラー: "Module lala: Child process exited with code 101"

**原因**: Cargoビルドが失敗している

**解決策**:
```bash
# ログを確認
flatpak-builder --force-clean build-dir com.github.clearclown.Lala.yml 2>&1 | tee build.log

# デバッグビルド
flatpak-builder --keep-build-dirs --force-clean build-dir com.github.clearclown.Lala.yml
cd .flatpak-builder/build/lala-1/
# ビルドディレクトリで問題を調査
```

### エラー: "ERROR: Icon validation failed"

**原因**: アイコンサイズまたは形式が不正

**解決策**:
```bash
# アイコンを128x128 PNGに変換
convert icon.png -resize 128x128 assets/icon.png

# または複数サイズを提供
install -Dm644 ./assets/icon-48.png /app/share/icons/hicolor/48x48/apps/com.github.clearclown.Lala.png
install -Dm644 ./assets/icon-128.png /app/share/icons/hicolor/128x128/apps/com.github.clearclown.Lala.png
```

### エラー: "Failed to load generated-sources.json"

**原因**: generated-sources.jsonが正しく生成されていない

**解決策**:
```bash
# Cargo.lockが最新か確認
cargo update
cargo build

# 再生成
python3 flatpak-cargo-generator.py ./Cargo.lock -o generated-sources.json
```

---

## 📚 参考リンク

- [Flatpak Documentation](https://docs.flatpak.org/)
- [Flathub Submission Guidelines](https://github.com/flathub/flathub/wiki/App-Submission)
- [Flatpak Rust Example](https://github.com/flatpak/flatpak-builder-tools/tree/master/cargo)
- [AppStream Metadata Guidelines](https://www.freedesktop.org/software/appstream/docs/)

---

## ✅ 完了チェック

Flatpakリリースが完了したら：

- [ ] マニフェストファイルが作成されている
- [ ] generated-sources.jsonが生成されている
- [ ] AppStreamメタデータが作成されている
- [ ] デスクトップファイルとアイコンが準備されている
- [ ] ローカルで`flatpak-builder`が成功する
- [ ] ローカルでインストールして動作確認できる
- [ ] Flathubにプルリクエストを作成（オプション）
- [ ] READMEにインストール手順が記載されている
- [ ] todo.md の「Flatpak」にチェックを入れる

---

**次のステップ**: [snap.md](./snap.md) でSnap Storeパッケージを作成してください。
