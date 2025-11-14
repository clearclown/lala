# Lala Editor - Packaging and Distribution Strategy

## Overview

This document outlines the comprehensive strategy for making Lala Editor easily installable across different platforms and package managers.

## Target Platforms

### 1. Rust Ecosystem (Priority 1) ✅ EASIEST
- **crates.io** - Official Rust package registry
- **Installation**: `cargo install lala`
- **Benefits**:
  - Native Rust distribution
  - Automatic dependency management
  - Cross-platform support
  - Easy updates via `cargo install --force`

### 2. Debian/Ubuntu (Priority 2) 📦
- **Package Manager**: apt/dpkg
- **Installation**: `sudo apt install lala`
- **Distribution**: PPA or .deb file
- **Target Systems**: Debian, Ubuntu, Linux Mint, Pop!_OS, etc.

### 3. Arch Linux (Priority 2) 🔷
- **Package Manager**: pacman
- **Installation**: `pacman -S lala` or `yay -S lala`
- **Distribution**: AUR (Arch User Repository)
- **Target Systems**: Arch Linux, Manjaro, EndeavourOS, etc.

### 4. Homebrew (Priority 3) 🍺
- **Package Manager**: brew
- **Installation**: `brew install lala`
- **Target Systems**: macOS, Linux
- **Benefits**: Popular on macOS, growing Linux adoption

### 5. Binary Releases (Priority 1) 📦
- **Platform**: GitHub Releases
- **Installation**: Download + extract + add to PATH
- **Target Systems**: All platforms (Linux, macOS, Windows)
- **Formats**: tar.gz, zip

### 6. Additional Options (Future)
- **Flatpak** - Universal Linux package
- **Snap** - Ubuntu's universal package format
- **AppImage** - Single-file Linux application
- **Scoop** - Windows package manager
- **Chocolatey** - Another Windows package manager
- **Nix** - NixOS package manager

---

## Implementation Plan

## Phase 1: Rust Ecosystem (Week 1)

### Step 1: Prepare for crates.io

**Prerequisites:**
1. Create account on crates.io
2. Generate API token: `cargo login`
3. Verify package metadata in Cargo.toml

**Cargo.toml Requirements:**
```toml
[package]
name = "lala"
version = "0.1.0"
edition = "2021"
authors = ["lala contributors"]
description = "A modern, lightweight text editor with GUI and CLI support for Markdown, HTML, Mermaid, and LaTeX"
license = "MIT OR Apache-2.0"
repository = "https://github.com/USER/lala"
homepage = "https://github.com/USER/lala"
documentation = "https://github.com/USER/lala/blob/main/README.md"
keywords = ["editor", "markdown", "latex", "mermaid", "html"]
categories = ["command-line-utilities", "text-editors", "visualization"]
readme = "README.md"
exclude = [
    "tests/fixtures/*",
    ".git*",
    "target/",
]
```

**Publishing Steps:**
```bash
# 1. Update version in Cargo.toml
cargo build --release

# 2. Run tests
cargo test --all

# 3. Generate documentation
cargo doc --no-deps

# 4. Dry run
cargo publish --dry-run

# 5. Publish
cargo publish
```

**Post-Publication:**
- Users can install with: `cargo install lala`
- Updates: `cargo install --force lala`

---

## Phase 2: Binary Releases (Week 1-2)

### Step 1: Setup GitHub Releases

**Create Release Workflow** (`.github/workflows/release.yml`):

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: lala
            asset_name: lala-linux-x86_64.tar.gz
          - os: ubuntu-latest
            target: x86_64-unknown-linux-musl
            artifact_name: lala
            asset_name: lala-linux-x86_64-musl.tar.gz
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact_name: lala
            asset_name: lala-macos-x86_64.tar.gz
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact_name: lala
            asset_name: lala-macos-aarch64.tar.gz
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact_name: lala.exe
            asset_name: lala-windows-x86_64.zip

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: ${{ matrix.target }}
          override: true

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Strip binary (Linux/macOS)
        if: matrix.os != 'windows-latest'
        run: strip target/${{ matrix.target }}/release/${{ matrix.artifact_name }}

      - name: Package (Linux/macOS)
        if: matrix.os != 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          tar czf ${{ matrix.asset_name }} ${{ matrix.artifact_name }}
          mv ${{ matrix.asset_name }} $GITHUB_WORKSPACE/

      - name: Package (Windows)
        if: matrix.os == 'windows-latest'
        run: |
          cd target/${{ matrix.target }}/release
          7z a ${{ matrix.asset_name }} ${{ matrix.artifact_name }}
          mv ${{ matrix.asset_name }} $env:GITHUB_WORKSPACE/

      - name: Upload Release Asset
        uses: softprops/action-gh-release@v1
        with:
          files: ${{ matrix.asset_name }}
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

**Creating a Release:**
```bash
# 1. Update version in Cargo.toml
# 2. Commit changes
git add Cargo.toml
git commit -m "Bump version to 0.1.0"

# 3. Create and push tag
git tag v0.1.0
git push origin v0.1.0

# GitHub Actions will automatically build and create release
```

**Installation Instructions:**
```bash
# Linux
wget https://github.com/USER/lala/releases/download/v0.1.0/lala-linux-x86_64.tar.gz
tar xzf lala-linux-x86_64.tar.gz
sudo mv lala /usr/local/bin/
```

---

## Phase 3: Debian/Ubuntu Package (Week 2-3)

### Step 1: Create Debian Package Structure

**Directory Structure:**
```
debian/
├── changelog
├── control
├── copyright
├── rules
├── lala.install
└── compat
```

**debian/control:**
```
Source: lala
Section: editors
Priority: optional
Maintainer: Lala Contributors <your-email@example.com>
Build-Depends: debhelper (>= 11), cargo, rustc, libxcb1-dev, libxcb-render0-dev, libxcb-shape0-dev, libxcb-xfixes0-dev
Standards-Version: 4.5.0
Homepage: https://github.com/USER/lala
Vcs-Git: https://github.com/USER/lala.git
Vcs-Browser: https://github.com/USER/lala

Package: lala
Architecture: any
Depends: ${shlibs:Depends}, ${misc:Depends}
Description: Modern text editor with multi-format preview support
 Lala is a lightweight text editor with both GUI and CLI interfaces.
 It supports previewing Markdown, HTML, Mermaid diagrams, and LaTeX
 documents with beautiful terminal formatting.
 .
 Features:
  - File tree view with async loading
  - Syntax highlighting for multiple languages
  - Multi-format preview (Markdown, HTML, Mermaid, LaTeX)
  - Advanced search and replace with regex support
  - CLI and GUI modes
```

**debian/rules:**
```makefile
#!/usr/bin/make -f

%:
	dh $@

override_dh_auto_build:
	cargo build --release

override_dh_auto_install:
	install -D -m755 target/release/lala debian/lala/usr/bin/lala

override_dh_auto_test:
	cargo test --all
```

**debian/changelog:**
```
lala (0.1.0-1) unstable; urgency=low

  * Initial release
  * Multi-format preview support (Markdown, HTML, Mermaid, LaTeX)
  * GUI and CLI modes
  * Syntax highlighting

 -- Lala Contributors <your-email@example.com>  Mon, 14 Nov 2025 00:00:00 +0000
```

**Building .deb Package:**
```bash
# Install build dependencies
sudo apt install debhelper cargo rustc libxcb1-dev libxcb-render0-dev

# Build package
dpkg-buildpackage -us -uc -b

# Result: lala_0.1.0-1_amd64.deb
```

**Installation:**
```bash
sudo dpkg -i lala_0.1.0-1_amd64.deb
sudo apt-get install -f  # Fix dependencies if needed
```

### Step 2: Setup PPA (Personal Package Archive)

**On Launchpad:**
1. Create Launchpad account
2. Create PPA: `ppa:username/lala`
3. Upload source package:
```bash
debuild -S
dput ppa:username/lala lala_0.1.0-1_source.changes
```

**Users Install:**
```bash
sudo add-apt-repository ppa:username/lala
sudo apt update
sudo apt install lala
```

---

## Phase 4: Arch Linux Package (Week 3-4)

### Step 1: Create PKGBUILD

**PKGBUILD:**
```bash
# Maintainer: Lala Contributors <your-email@example.com>
pkgname=lala
pkgver=0.1.0
pkgrel=1
pkgdesc="Modern text editor with multi-format preview support"
arch=('x86_64')
url="https://github.com/USER/lala"
license=('MIT' 'Apache')
depends=('gcc-libs' 'libxcb')
makedepends=('cargo' 'rust')
source=("$pkgname-$pkgver.tar.gz::https://github.com/USER/lala/archive/v$pkgver.tar.gz")
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
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 LICENSE-MIT "$pkgdir/usr/share/licenses/$pkgname/LICENSE-MIT"
    install -Dm644 LICENSE-APACHE "$pkgdir/usr/share/licenses/$pkgname/LICENSE-APACHE"
}
```

**Testing PKGBUILD:**
```bash
makepkg -si
```

### Step 2: Submit to AUR

**AUR Repository Structure:**
```
lala/
├── PKGBUILD
└── .SRCINFO
```

**Generate .SRCINFO:**
```bash
makepkg --printsrcinfo > .SRCINFO
```

**Submit to AUR:**
```bash
# 1. Create SSH key for AUR
ssh-keygen -f ~/.ssh/aur

# 2. Add key to AUR account
cat ~/.ssh/aur.pub
# Upload to https://aur.archlinux.org/account/

# 3. Clone AUR repo
git clone ssh://aur@aur.archlinux.org/lala.git aur-lala
cd aur-lala

# 4. Add files
cp ../PKGBUILD .
makepkg --printsrcinfo > .SRCINFO

# 5. Commit and push
git add PKGBUILD .SRCINFO
git commit -m "Initial commit: lala 0.1.0"
git push
```

**Users Install:**
```bash
# Using yay
yay -S lala

# Using paru
paru -S lala

# Manual
git clone https://aur.archlinux.org/lala.git
cd lala
makepkg -si
```

---

## Phase 5: Homebrew Formula (Week 4-5)

### Step 1: Create Formula

**lala.rb:**
```ruby
class Lala < Formula
  desc "Modern text editor with multi-format preview support"
  homepage "https://github.com/USER/lala"
  url "https://github.com/USER/lala/archive/v0.1.0.tar.gz"
  sha256 "CALCULATE_HASH"
  license "MIT OR Apache-2.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/lala", "--version"
    system "#{bin}/lala", "--help"
  end
end
```

**Calculate SHA256:**
```bash
wget https://github.com/USER/lala/archive/v0.1.0.tar.gz
sha256sum v0.1.0.tar.gz
```

### Step 2: Submit to Homebrew

**Option 1: Personal Tap**
```bash
# Create tap repository
mkdir homebrew-lala
cd homebrew-lala
cp ../lala.rb Formula/lala.rb
git init
git add Formula/lala.rb
git commit -m "Add lala formula"
git remote add origin https://github.com/USER/homebrew-lala.git
git push -u origin main
```

**Users Install:**
```bash
brew tap username/lala
brew install lala
```

**Option 2: Submit to homebrew-core** (after project matures)
```bash
# Fork homebrew-core
# Add formula to Formula/
# Submit pull request
```

---

## Installation Documentation

### Create INSTALL.md

```markdown
# Installation Guide

## Quick Install

### Via Cargo (Recommended)
```bash
cargo install lala
```

### Via Package Managers

#### Debian/Ubuntu
```bash
sudo add-apt-repository ppa:username/lala
sudo apt update
sudo apt install lala
```

#### Arch Linux
```bash
yay -S lala
```

#### Homebrew (macOS/Linux)
```bash
brew install lala
```

### From Binary

#### Linux
```bash
wget https://github.com/USER/lala/releases/latest/download/lala-linux-x86_64.tar.gz
tar xzf lala-linux-x86_64.tar.gz
sudo mv lala /usr/local/bin/
```

#### macOS
```bash
wget https://github.com/USER/lala/releases/latest/download/lala-macos-x86_64.tar.gz
tar xzf lala-macos-x86_64.tar.gz
sudo mv lala /usr/local/bin/
```

### From Source
```bash
git clone https://github.com/USER/lala.git
cd lala
cargo build --release
sudo cp target/release/lala /usr/local/bin/
```
```

---

## Automation with CI/CD

### Complete GitHub Actions Workflow

**`.github/workflows/ci.yml`** - Test on every push
**`.github/workflows/release.yml`** - Build binaries on tag
**`.github/workflows/publish.yml`** - Publish to crates.io on tag

---

## Versioning Strategy

### Semantic Versioning (SemVer)

```
MAJOR.MINOR.PATCH

Examples:
0.1.0 - Initial release
0.1.1 - Bug fix
0.2.0 - New features
1.0.0 - Stable release
```

### Release Checklist

- [ ] Update version in Cargo.toml
- [ ] Update CHANGELOG.md
- [ ] Run full test suite
- [ ] Build release binary
- [ ] Test installation
- [ ] Create git tag
- [ ] Push tag (triggers CI/CD)
- [ ] Update package repositories
- [ ] Announce release

---

## Maintenance

### Regular Updates

1. **Monitor Dependencies**: `cargo outdated`
2. **Security Audits**: `cargo audit`
3. **Update Packages**: Keep all package repos updated
4. **Test Installations**: Verify all package managers work

### Support Matrix

| Platform | Support Level | Update Frequency |
|----------|--------------|------------------|
| crates.io | Primary | Every release |
| GitHub Releases | Primary | Every release |
| AUR | Secondary | Every release |
| Debian PPA | Secondary | Major releases |
| Homebrew | Secondary | Major releases |

---

## Summary

**Priority Order:**
1. ✅ **cargo install** - Easiest, immediate
2. 📦 **GitHub Releases** - Cross-platform binaries
3. 🔷 **AUR** - Arch Linux users
4. 📦 **Debian PPA** - Ubuntu/Debian users
5. 🍺 **Homebrew** - macOS users

**Timeline:**
- Week 1: crates.io + GitHub Releases
- Week 2-3: Debian package + PPA
- Week 3-4: AUR submission
- Week 4-5: Homebrew formula

**Next Steps:**
1. Add LICENSE files (MIT and Apache-2.0)
2. Update Cargo.toml with all metadata
3. Create GitHub Actions workflows
4. Test local packaging
5. Publish to crates.io
