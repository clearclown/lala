# 🚀 Lala リリース準備ガイド

このドキュメントは、Lalaを各プラットフォームにリリースするための完全ガイドです。
初心者でも理解できるよう、各ステップを詳しく説明します。

---

## 📋 リリースチェックリスト

### ✅ 準備段階（完了済み）
- [x] release-1ブランチの作成
- [x] リリースドキュメント構造の作成
- [x] バージョン0.1.0の確認

### 📦 パッケージング準備
- [x] Cargo.tomlのメタデータ確認
- [x] LICENSEファイルの確認
- [x] READMEの最終確認
- [x] CHANGELOGの作成

### 🏗️ ビルド設定
- [x] クロスプラットフォームビルドの設定
- [x] GitHub Actions CI/CDの設定
- [x] リリースアセットの自動生成

### 🎯 プラットフォーム別リリース

#### 1. Cargo (crates.io) - 最優先 ⭐
**難易度**: ★☆☆☆☆ (簡単)
**ステータス**: ✅ 完了 (2025-11-17)
**必要な作業**:
- [x] crates.ioアカウントの作成
- [x] `cargo publish --dry-run` でチェック
- [x] `cargo publish` で公開

**リリースURL**: https://crates.io/crates/lala
**詳細**: [cargo-release.md](./platforms/cargo-release.md)

---

#### 2. GitHub Releases - 最優先 ⭐
**難易度**: ★★☆☆☆ (簡単)
**ステータス**: ✅ 完了 (2025-11-17)
**必要な作業**:
- [x] バイナリのクロスコンパイル設定
- [x] GitHub Actionsワークフロー作成
- [x] タグの作成とリリースノート

**リリースURL**: https://github.com/clearclown/lala/releases/tag/v0.1.0
**プラットフォーム**: Linux (x86_64, ARM64), macOS (Intel, Apple Silicon), Windows (x86_64)
**詳細**: [github-releases.md](./platforms/github-releases.md)

---

#### 3. Homebrew (macOS/Linux)
**難易度**: ★★★☆☆ (中級)
**ステータス**: 準備中
**必要な作業**:
- [ ] Homebrewフォーミュラの作成
- [ ] Tap リポジトリの作成
- [ ] テストとレビュー

**詳細**: [homebrew.md](./platforms/homebrew.md)

---

#### 4. Debian/Ubuntu (.deb)
**難易度**: ★★★☆☆ (中級)
**ステータス**: 準備中
**必要な作業**:
- [ ] debianパッケージ設定ファイル作成
- [ ] `cargo-deb` の設定
- [ ] PPAリポジトリの作成（オプション）

**詳細**: [debian.md](./platforms/debian.md)

---

#### 5. Arch Linux (AUR)
**難易度**: ★★★☆☆ (中級)
**ステータス**: 準備中
**必要な作業**:
- [ ] PKGBUILDファイルの作成
- [ ] AURアカウントの作成
- [ ] AURへのアップロード

**詳細**: [arch.md](./platforms/arch.md)

---

#### 6. Fedora/RHEL (.rpm)
**難易度**: ★★★☆☆ (中級)
**ステータス**: 準備中
**必要な作業**:
- [ ] RPM specファイルの作成
- [ ] `cargo-generate-rpm` の設定
- [ ] COPRリポジトリの作成（オプション）

**詳細**: [rpm.md](./platforms/rpm.md)

---

#### 7. Flatpak
**難易度**: ★★★★☆ (上級)
**ステータス**: 準備中
**必要な作業**:
- [ ] Flatpakマニフェストの作成
- [ ] Flathubへの申請
- [ ] サンドボックス権限の設定

**詳細**: [flatpak.md](./platforms/flatpak.md)

---

#### 8. Snap Store
**難易度**: ★★★★☆ (上級)
**ステータス**: 準備中
**必要な作業**:
- [ ] snapcraft.yamlの作成
- [ ] Snap Storeアカウントの作成
- [ ] 公開とレビュー

**詳細**: [snap.md](./platforms/snap.md)

---

#### 9. Windows (.exe)
**難易度**: ★★★☆☆ (中級)
**ステータス**: 準備中
**必要な作業**:
- [ ] Windows向けクロスコンパイル
- [ ] インストーラーの作成（NSIS/WiX）
- [ ] コード署名（オプション）

**詳細**: [windows.md](./platforms/windows.md)

---

## 🎯 推奨リリース順序

初心者向けに、簡単なものから順番にリリースすることをお勧めします：

### Phase 1: 基本リリース ✅ 完了 (2025-11-17)
1. ✅ **Cargo (crates.io)** - Rustユーザー向け、最も簡単
2. ✅ **GitHub Releases** - バイナリ配布、比較的簡単

### Phase 2: パッケージマネージャー（3-5日）
3. **Homebrew** - macOS/Linuxユーザー向け
4. **Debian (.deb)** - Ubuntu/Debian系ユーザー向け
5. **Arch (AUR)** - Arch Linuxユーザー向け
6. **RPM** - Fedora/RHELユーザー向け

### Phase 3: アプリストア（1-2週間）
7. **Flatpak** - Linux統一パッケージ
8. **Snap Store** - Ubuntu公式ストア
9. **Windows** - Windows向けインストーラー

---

## 📚 事前準備

### 1. バージョン管理

現在のバージョンを確認：
```bash
grep "^version" Cargo.toml
# 出力: version = "0.1.0"
```

### 2. ビルドテスト

すべてのプラットフォームでビルドが通ることを確認：
```bash
# Linux
cargo build --release

# Windows (クロスコンパイル)
cargo build --release --target x86_64-pc-windows-gnu

# macOS (クロスコンパイル - macOS上で実行)
cargo build --release --target x86_64-apple-darwin
```

### 3. テスト実行

すべてのテストが通ることを確認：
```bash
cargo test --all-features
# 期待: 171 tests passed
```

---

## 🔧 必要なツールのインストール

リリース作業に必要なツールをインストールします：

### すべてのプラットフォーム共通
```bash
# クロスコンパイルツール
cargo install cross

# リリース自動化ツール
cargo install cargo-release

# パッケージングツール
cargo install cargo-deb          # Debian
cargo install cargo-generate-rpm # RPM
```

### GitHub CLI
```bash
# すでにインストール済み
gh --version
```

---

## 🚨 トラブルシューティング

### ビルドエラー

**問題**: クロスコンパイルが失敗する
**解決策**:
```bash
# Dockerベースのcrossツールを使用
cross build --release --target x86_64-pc-windows-gnu
```

### 依存関係エラー

**問題**: システム依存のライブラリが見つからない
**解決策**:
- eguiは純粋なRustなので基本的に問題なし
- OpenSSLではなくrustls-tlsを使用（すでに設定済み）

---

## 📞 サポート

リリース作業で困ったときは：

1. **各プラットフォームの詳細ガイド**を確認
   - `docs/release/platforms/` 以下のファイル

2. **GitHub Issues**で質問
   - タグ: `release`, `packaging`

3. **コミュニティ**
   - Rust Japan Users Group
   - egui Discord

---

## 🎉 リリース後のタスク

リリースが完了したら：

- [x] リリースノートの作成 (CHANGELOG.md)
- [ ] ブログ投稿/SNS告知
- [ ] Reddit/HackerNewsへの投稿
- [ ] パッケージマネージャーのドキュメント更新
- [ ] ユーザーフィードバックの収集

### ✅ Phase 1 完了状況

**crates.io**:
- パッケージ: https://crates.io/crates/lala
- バージョン: 0.1.0
- サイズ: 411.1 KiB (97.6 KiB 圧縮)
- インストール: `cargo install lala`

**GitHub Releases**:
- リリースページ: https://github.com/clearclown/lala/releases/tag/v0.1.0
- バイナリ数: 5プラットフォーム
  - lala-linux-x86_64
  - lala-linux-aarch64
  - lala-macos-x86_64
  - lala-macos-aarch64
  - lala-windows-x86_64.exe

---

## 📝 次のステップ

1. **Phase 1から開始**: [cargo-release.md](./platforms/cargo-release.md)を読む
2. **必要なツールをインストール**（上記参照）
3. **各プラットフォームのガイドに従う**

頑張ってください！🚀
