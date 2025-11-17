# 🍺 Homebrew リリースガイド

**難易度**: ★★★☆☆ (中級)
**推定時間**: 2-3時間
**優先度**: Phase 2

---

## 🖱️ GUI操作が必須の部分

以下の操作は**ブラウザでの手動操作が必要**です：

1. **GitHubリポジトリ作成**（初回のみ、2分）
   - `gh repo create homebrew-lala --public` でCLI作成できるが、
   - ブラウザで https://github.com/new から手動作成も可能

2. **公式Homebrew Coreへの申請**（オプション、人気が出たら）
   - GitHub上でプルリクエストのレビュー対応
   - https://github.com/Homebrew/homebrew-core でPR作成
   - レビュアーとのやり取りはGitHub上で

**基本的な公開（自分のTap）は全てCLIで完結します！**

---

## 🎯 概要

HomebrewはmacOSとLinux用のパッケージマネージャーです。
Homebrewに公開すると、ユーザーは`brew install lala`で簡単にインストールできます。

### メリット
- macOS/Linuxユーザーに人気
- 依存関係の自動管理
- アップデートが簡単（`brew upgrade`）
- 複数バージョンの管理

---

## 📋 前提条件

### 1. GitHub Releasesの準備

Homebrewは GitHub Releases からソースコードまたはバイナリをダウンロードします。
先に [github-releases.md](./github-releases.md) を完了してください。

### 2. Homebrewのインストール（テスト用）

```bash
# macOS/Linux
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

---

## 🏗️ Homebrewフォーミュラの作成

### Step 1: Tap リポジトリの作成

Homebrewでは、カスタムリポジトリを「Tap」と呼びます。

```bash
# GitHubに新しいリポジトリを作成
gh repo create homebrew-lala --public --description "Homebrew tap for Lala text editor"

# ローカルにクローン
cd ~/Projects
git clone https://github.com/clearclown/homebrew-lala.git
cd homebrew-lala
```

### Step 2: フォーミュラファイルの作成

```bash
# Formulaディレクトリを作成
mkdir -p Formula

# lala.rb フォーミュラを作成
cat > Formula/lala.rb << 'EOF'
class Lala < Formula
  desc "Modern, lightweight multi-format text editor"
  homepage "https://github.com/clearclown/lala"
  url "https://github.com/clearclown/lala/archive/refs/tags/v0.1.0.tar.gz"
  sha256 ""  # Step 3で計算
  license "MIT OR Apache-2.0"
  head "https://github.com/clearclown/lala.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "lala", shell_output("#{bin}/lala --version")
  end
end
EOF
```

### Step 3: SHA256ハッシュの計算

```bash
# リリースのtarballをダウンロードしてハッシュを計算
curl -L https://github.com/clearclown/lala/archive/refs/tags/v0.1.0.tar.gz -o lala-0.1.0.tar.gz
shasum -a 256 lala-0.1.0.tar.gz

# 出力例: a1b2c3d4e5f6... lala-0.1.0.tar.gz
# この値を lala.rb の sha256 に設定
```

**lala.rb を更新**:
```ruby
  sha256 "a1b2c3d4e5f6..."  # 実際のハッシュ値に置き換え
```

---

## 🧪 ローカルテスト

### Step 1: フォーミュラの構文チェック

```bash
cd ~/Projects/homebrew-lala
brew audit --strict --online Formula/lala.rb
```

### Step 2: インストールテスト

```bash
# Tapを追加
brew tap clearclown/lala

# インストール
brew install clearclown/lala/lala

# 動作確認
lala --version
```

### Step 3: アンインストールテスト

```bash
brew uninstall lala
brew untap clearclown/lala
```

---

## 🚀 公開手順

### Step 1: フォーミュラをコミット

```bash
cd ~/Projects/homebrew-lala

git add Formula/lala.rb
git commit -m "Add lala formula version 0.1.0"
git push origin main
```

### Step 2: ユーザーへの案内

READMEに以下の手順を追加：

```markdown
## Installation with Homebrew (macOS/Linux)

```bash
# Add the tap
brew tap clearclown/lala

# Install
brew install lala
```

### Upgrade
```bash
brew upgrade lala
```

### Uninstall
```bash
brew uninstall lala
brew untap clearclown/lala
```
```

---

## 🔄 バージョンアップデート

新しいバージョンをリリースしたら：

### Step 1: 新しいリリースのハッシュを取得

```bash
# 新バージョンのtarballをダウンロード
curl -L https://github.com/clearclown/lala/archive/refs/tags/v0.1.1.tar.gz -o lala-0.1.1.tar.gz
shasum -a 256 lala-0.1.1.tar.gz
```

### Step 2: フォーミュラを更新

```bash
cd ~/Projects/homebrew-lala

# lala.rb を編集
cat > Formula/lala.rb << 'EOF'
class Lala < Formula
  desc "Modern, lightweight multi-format text editor"
  homepage "https://github.com/clearclown/lala"
  url "https://github.com/clearclown/lala/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "新しいハッシュ値"
  license "MIT OR Apache-2.0"
  head "https://github.com/clearclown/lala.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "lala", shell_output("#{bin}/lala --version")
  end
end
EOF
```

### Step 3: テストとコミット

```bash
# テスト
brew reinstall clearclown/lala/lala
lala --version  # 新バージョンか確認

# コミット
git add Formula/lala.rb
git commit -m "Update lala to version 0.1.1"
git push origin main
```

---

## 🌟 公式Homebrewへの申請（オプション）

人気が出たら、公式のHomebrew Coreに申請できます。

### 要件
- 75以上のGitHub Stars
- 30日以上の活発な開発
- 安定したリリース
- 明確なライセンス

### 申請方法

```bash
# 1. homebrew-coreをフォーク
gh repo fork Homebrew/homebrew-core

# 2. フォーミュラを追加
cd homebrew-core
cp ~/Projects/homebrew-lala/Formula/lala.rb Formula/

# 3. プルリクエストを作成
git checkout -b add-lala
git add Formula/lala.rb
git commit -m "lala 0.1.0 (new formula)"
git push origin add-lala
gh pr create --repo Homebrew/homebrew-core
```

**注意**: 厳格なレビュープロセスがあります。詳細は[Homebrew Contribution Guide](https://docs.brew.sh/How-To-Open-a-Homebrew-Pull-Request)を参照。

---

## 📊 高度な設定

### バイナリボトル（プリコンパイル版）の提供

ビルド時間を短縮するため、プリコンパイルしたバイナリを提供できます：

```ruby
class Lala < Formula
  desc "Modern, lightweight multi-format text editor"
  homepage "https://github.com/clearclown/lala"
  url "https://github.com/clearclown/lala/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "..."
  license "MIT OR Apache-2.0"

  # macOS向けバイナリボトル
  bottle do
    root_url "https://github.com/clearclown/lala/releases/download/v0.1.0"
    sha256 cellar: :any_skip_relocation, ventura:      "..."
    sha256 cellar: :any_skip_relocation, monterey:     "..."
    sha256 cellar: :any_skip_relocation, big_sur:      "..."
    sha256 cellar: :any_skip_relocation, x86_64_linux: "..."
  end

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "lala", shell_output("#{bin}/lala --version")
  end
end
```

**ボトルの作成**:
```bash
brew install --build-bottle clearclown/lala/lala
brew bottle clearclown/lala/lala
```

---

## 🚨 トラブルシューティング

### エラー: "Failed to download resource"

**原因**: URLが間違っている、またはリリースが存在しない

**解決策**:
```bash
# URLを確認
curl -I https://github.com/clearclown/lala/archive/refs/tags/v0.1.0.tar.gz

# 存在しない場合、GitHubでリリースを作成
```

### エラー: "SHA256 mismatch"

**原因**: ハッシュ値が正しくない

**解決策**:
```bash
# 再計算
curl -L https://github.com/clearclown/lala/archive/refs/tags/v0.1.0.tar.gz | shasum -a 256
```

### 警告: "Formulae should not depend on other formulae at runtime"

**原因**: 不要なruntime依存関係

**解決策**:
```ruby
# ビルド時のみの依存関係には :build を付ける
depends_on "rust" => :build
```

### エラー: "Version detection failed"

**原因**: `lala --version` が正しく動作していない

**解決策**:
```bash
# バイナリのバージョン出力を確認
cargo run -- --version

# Cargo.tomlにバージョン情報があるか確認
```

---

## 📚 参考リンク

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Homebrew Acceptable Formulae](https://docs.brew.sh/Acceptable-Formulae)
- [Creating Taps](https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap)
- [Ruby DSL Reference](https://rubydoc.brew.sh/Formula.html)

---

## ✅ 完了チェック

Homebrewリリースが完了したら：

- [ ] homebrew-lala リポジトリが作成されている
- [ ] Formula/lala.rb が正しく作成されている
- [ ] SHA256ハッシュが正しい
- [ ] `brew install clearclown/lala/lala` でインストールできる
- [ ] インストールしたバイナリが動作する
- [ ] `brew uninstall` でアンインストールできる
- [ ] READMEにインストール手順が記載されている
- [ ] todo.md の「Homebrew」にチェックを入れる

---

**次のステップ**: [debian.md](./debian.md) でDebian/Ubuntuパッケージを作成してください。
