# 🏔️ Arch Linux (AUR) リリースガイド

**難易度**: ★★★☆☆ (中級)
**推定時間**: 1-2時間
**優先度**: Phase 2

---

## 🎯 概要

AUR (Arch User Repository) はArch Linuxユーザー向けのコミュニティ駆動パッケージリポジトリです。
AURに公開すると、ユーザーは`yay -S lala`や`paru -S lala`で簡単にインストールできます。

### メリット
- Arch/Manjaro/EndeavourOSユーザーにリーチできる
- メンテナンスが比較的簡単
- コミュニティのフィードバックが得られる

---

## 📋 前提条件

### 1. AURアカウントの作成

1. [AUR](https://aur.archlinux.org/)にアクセス
2. 右上の「Register」をクリック
3. アカウントを作成
4. SSH公開鍵を登録

### 2. SSH鍵の設定

```bash
# SSH鍵が無い場合は作成
ssh-keygen -t ed25519 -C "your.email@example.com"

# 公開鍵をコピー
cat ~/.ssh/id_ed25519.pub

# AUR account pageで公開鍵を登録
# https://aur.archlinux.org/account/
```

### 3. 必要なツールのインストール（Arch Linux上で）

```bash
# ビルドツール
sudo pacman -S base-devel git

# AURヘルパー（テスト用）
sudo pacman -S --needed git base-devel
git clone https://aur.archlinux.org/yay.git
cd yay
makepkg -si
```

---

## 🏗️ PKGBUILDファイルの作成

### Step 1: パッケージ情報の収集

```bash
# GitHubリリースのソースコードtarballのURL
# https://github.com/clearclown/lala/archive/refs/tags/v0.1.0.tar.gz

# SHA256ハッシュの計算
curl -L https://github.com/clearclown/lala/archive/refs/tags/v0.1.0.tar.gz | sha256sum
```

### Step 2: PKGBUILDの作成

```bash
# ローカルディレクトリを作成
mkdir -p ~/aur-packages/lala
cd ~/aur-packages/lala

# PKGBUILD ファイルを作成
cat > PKGBUILD << 'EOF'
# Maintainer: Your Name <your.email@example.com>
pkgname=lala
pkgver=0.1.0
pkgrel=1
pkgdesc="Modern, lightweight multi-format text editor written in Rust"
arch=('x86_64' 'aarch64')
url="https://github.com/clearclown/lala"
license=('MIT' 'Apache')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/clearclown/$pkgname/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')  # Step 3で更新

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

check() {
    cd "$pkgname-$pkgver"
    cargo test --release --locked
}

package() {
    cd "$pkgname-$pkgver"

    # バイナリのインストール
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"

    # ライセンスファイル
    install -Dm644 LICENSE-MIT "$pkgdir/usr/share/licenses/$pkgname/LICENSE-MIT"
    install -Dm644 LICENSE-APACHE "$pkgdir/usr/share/licenses/$pkgname/LICENSE-APACHE"

    # ドキュメント
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
EOF
```

### Step 3: SHA256ハッシュの更新

```bash
# ハッシュを計算
updpkgsums

# または手動で
curl -L https://github.com/clearclown/lala/archive/refs/tags/v0.1.0.tar.gz | sha256sum
# PKGBUILDのsha256sumsを更新
```

### Step 4: .SRCINFOの生成

```bash
# .SRCINFOを生成（AURが読み取るメタデータファイル）
makepkg --printsrcinfo > .SRCINFO
```

---

## 🧪 ローカルテスト

### Step 1: ビルドテスト

```bash
cd ~/aur-packages/lala

# パッケージをビルド
makepkg -f

# 生成されたパッケージを確認
ls -lh lala-*.pkg.tar.zst
```

### Step 2: インストールテスト

```bash
# ビルドしたパッケージをインストール
sudo pacman -U lala-*.pkg.tar.zst

# 動作確認
lala --version
```

### Step 3: アンインストールテスト

```bash
sudo pacman -R lala
```

### Step 4: クリーンビルドテスト

```bash
# ビルドファイルを削除
rm -rf src/ pkg/ *.pkg.tar.zst

# 再度ビルド（依存関係も含めてテスト）
makepkg -s
```

---

## 🚀 AURへの公開

### Step 1: Gitリポジトリの初期化

```bash
cd ~/aur-packages/lala

# AURのリポジトリをクローン
git clone ssh://aur@aur.archlinux.org/lala.git lala-aur
cd lala-aur

# PKGBUILDと.SRCINFOをコピー
cp ../PKGBUILD .
cp ../.SRCINFO .
```

### Step 2: コミットとプッシュ

```bash
# ファイルを追加
git add PKGBUILD .SRCINFO

# コミット
git commit -m "Initial upload: lala 0.1.0"

# AURにプッシュ
git push origin master
```

### Step 3: パッケージページの確認

```bash
# ブラウザで確認
xdg-open https://aur.archlinux.org/packages/lala
```

---

## 📊 ユーザーへの案内

READMEに以下のインストール手順を追加：

```markdown
## Installation on Arch Linux

### From AUR

Using `yay`:
```bash
yay -S lala
```

Using `paru`:
```bash
paru -S lala
```

Manual installation:
```bash
git clone https://aur.archlinux.org/lala.git
cd lala
makepkg -si
```

### Upgrade
```bash
yay -Syu lala
```

### Uninstall
```bash
sudo pacman -R lala
```
```

---

## 🔄 バージョンアップデート

新しいバージョンをリリースしたら：

### Step 1: PKGBUILDの更新

```bash
cd ~/aur-packages/lala-aur

# PKGBUILDを編集
# pkgver を新バージョンに更新
# pkgrel を 1 にリセット
cat > PKGBUILD << 'EOF'
# Maintainer: Your Name <your.email@example.com>
pkgname=lala
pkgver=0.1.1  # 更新
pkgrel=1      # リセット
pkgdesc="Modern, lightweight multi-format text editor written in Rust"
arch=('x86_64' 'aarch64')
url="https://github.com/clearclown/lala"
license=('MIT' 'Apache')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/clearclown/$pkgname/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

check() {
    cd "$pkgname-$pkgver"
    cargo test --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 LICENSE-MIT "$pkgdir/usr/share/licenses/$pkgname/LICENSE-MIT"
    install -Dm644 LICENSE-APACHE "$pkgdir/usr/share/licenses/$pkgname/LICENSE-APACHE"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
EOF
```

### Step 2: SHA256の更新と.SRCINFO生成

```bash
# ハッシュを更新
updpkgsums

# .SRCINFOを再生成
makepkg --printsrcinfo > .SRCINFO
```

### Step 3: テストとプッシュ

```bash
# ビルドテスト
makepkg -f

# インストールテスト
sudo pacman -U lala-*.pkg.tar.zst

# 問題なければコミット
git add PKGBUILD .SRCINFO
git commit -m "Update to version 0.1.1"
git push origin master
```

---

## 📋 高度な設定

### バイナリパッケージ（-bin）の提供

コンパイル時間を短縮するため、プリコンパイル版も提供できます：

```bash
# lala-bin パッケージを作成
mkdir -p ~/aur-packages/lala-bin
cd ~/aur-packages/lala-bin

cat > PKGBUILD << 'EOF'
# Maintainer: Your Name <your.email@example.com>
pkgname=lala-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="Modern, lightweight multi-format text editor (binary release)"
arch=('x86_64')
url="https://github.com/clearclown/lala"
license=('MIT' 'Apache')
depends=()
provides=('lala')
conflicts=('lala')
source_x86_64=("https://github.com/clearclown/lala/releases/download/v$pkgver/lala-linux-x86_64")
sha256sums_x86_64=('SKIP')

package() {
    install -Dm755 "$srcdir/lala-linux-x86_64" "$pkgdir/usr/bin/lala"
}
EOF

# .SRCINFOを生成
makepkg --printsrcinfo > .SRCINFO

# AURに公開
git clone ssh://aur@aur.archlinux.org/lala-bin.git
cd lala-bin
cp ../PKGBUILD ../.SRCINFO .
git add PKGBUILD .SRCINFO
git commit -m "Initial upload: lala-bin 0.1.0"
git push origin master
```

### Git版（-git）の提供

開発版も提供できます：

```bash
cat > PKGBUILD << 'EOF'
# Maintainer: Your Name <your.email@example.com>
pkgname=lala-git
pkgver=r123.abc1234  # git rev-listで自動更新
pkgrel=1
pkgdesc="Modern, lightweight multi-format text editor (git version)"
arch=('x86_64' 'aarch64')
url="https://github.com/clearclown/lala"
license=('MIT' 'Apache')
depends=()
makedepends=('rust' 'cargo' 'git')
provides=('lala')
conflicts=('lala')
source=("git+https://github.com/clearclown/lala.git")
sha256sums=('SKIP')

pkgver() {
    cd lala
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
    cd lala
    cargo build --release --locked
}

package() {
    cd lala
    install -Dm755 "target/release/lala" "$pkgdir/usr/bin/lala"
    install -Dm644 LICENSE-MIT "$pkgdir/usr/share/licenses/$pkgname/LICENSE-MIT"
    install -Dm644 LICENSE-APACHE "$pkgdir/usr/share/licenses/$pkgname/LICENSE-APACHE"
}
EOF
```

---

## 🚨 トラブルシューティング

### エラー: "ERROR: One or more PGP signatures could not be verified!"

**原因**: 署名付きのソースを使用している場合

**解決策**:
```bash
# sha256sumsの代わりにPGP署名を使用
source=("$pkgname-$pkgver.tar.gz::https://github.com/..."
        "$pkgname-$pkgver.tar.gz.asc::https://github.com/.../releases/download/v$pkgver/$pkgname-$pkgver.tar.gz.asc")
validpgpkeys=('YOUR_GPG_KEY_ID')

# または署名チェックをスキップ（推奨しない）
sha256sums=('SKIP')
```

### エラー: "ERROR: PKGBUILD contains CRLF characters"

**原因**: Windowsの改行コードが混入

**解決策**:
```bash
dos2unix PKGBUILD
# またはLF改行に変換
```

### 警告: "One or more files did not pass the validity check!"

**原因**: SHA256ハッシュが一致しない

**解決策**:
```bash
# ハッシュを再計算
updpkgsums

# またはキャッシュをクリア
rm -rf ~/.cache/yay/lala
```

### エラー: "Permission denied (publickey)"

**原因**: SSH鍵が正しく設定されていない

**解決策**:
```bash
# SSH鍵がAURに登録されているか確認
ssh -T aur@aur.archlinux.org

# 登録されていない場合、AURアカウントページで登録
```

---

## 📚 参考リンク

- [AUR Submission Guidelines](https://wiki.archlinux.org/title/AUR_submission_guidelines)
- [PKGBUILD Reference](https://wiki.archlinux.org/title/PKGBUILD)
- [Creating packages (Arch Wiki)](https://wiki.archlinux.org/title/Creating_packages)
- [Rust package guidelines](https://wiki.archlinux.org/title/Rust_package_guidelines)

---

## ✅ 完了チェック

AURパッケージのリリースが完了したら：

- [ ] AURアカウントが作成されている
- [ ] SSH鍵が登録されている
- [ ] PKGBUILDファイルが作成されている
- [ ] .SRCINFOファイルが生成されている
- [ ] ローカルで`makepkg`が成功する
- [ ] AURにプッシュできる
- [ ] `yay -S lala`でインストールできる
- [ ] インストールしたバイナリが動作する
- [ ] READMEにインストール手順が記載されている
- [ ] todo.md の「Arch (AUR)」にチェックを入れる

---

**次のステップ**: [rpm.md](./rpm.md) でFedora/RHELパッケージを作成してください。
