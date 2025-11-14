# Package Publishing Guide

Complete guide for publishing Lala to various package managers.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Crates.io (Cargo)](#cratesio-cargo)
- [AUR (Arch User Repository)](#aur-arch-user-repository)
- [Debian/Ubuntu (APT)](#debianubuntu-apt)
- [Fedora/RHEL (RPM)](#fedorarhel-rpm)
- [Homebrew (macOS/Linux)](#homebrew-macoslinux)
- [Chocolatey (Windows)](#chocolatey-windows)
- [Scoop (Windows)](#scoop-windows)
- [GitHub Releases](#github-releases)
- [Automation](#automation)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### Required Tools

```bash
# Git (for version control)
git --version

# Rust and Cargo (for building)
rustc --version
cargo --version

# GitHub CLI (for releases)
gh --version

# Docker (for cross-platform builds)
docker --version
```

### Required Accounts

- [ ] GitHub account with repository access
- [ ] Crates.io account (link to GitHub)
- [ ] AUR account (for Arch packages)
- [ ] Homebrew tap repository access
- [ ] Chocolatey account (for Windows packages)

### Version Preparation

Before publishing, ensure:

1. **Version bump** in `Cargo.toml`:
   ```toml
   [package]
   version = "0.1.0"  # Update this
   ```

2. **Update CHANGELOG.md**:
   ```bash
   # Add new version entry
   vim docs/CHANGELOG.md
   ```

3. **Update version in packaging files**:
   - `packaging/arch/PKGBUILD`
   - `packaging/rpm/lala.spec`
   - `packaging/windows/chocolatey/lala.nuspec`
   - `packaging/windows/installer.nsi`
   - `packaging/homebrew/lala.rb`

4. **Run tests**:
   ```bash
   cargo test --all
   cargo clippy --all-targets --all-features
   cargo build --release
   ```

## Crates.io (Cargo)

### First-Time Setup

1. **Get API token**:
   ```bash
   # Login to crates.io and get token from
   # https://crates.io/settings/tokens

   # Save token
   cargo login <YOUR_TOKEN>
   ```

2. **Verify package**:
   ```bash
   # Dry run
   cargo publish --dry-run

   # Check what will be included
   cargo package --list
   ```

### Publishing

```bash
# 1. Clean build
cargo clean
cargo build --release

# 2. Run tests
cargo test --all

# 3. Verify package
cargo package

# 4. Publish
cargo publish

# Note: This is PERMANENT and cannot be undone!
# Make sure you're ready before publishing
```

### Post-Publishing

```bash
# Verify it's published
cargo search lala

# Test installation
cargo install lala

# Check on crates.io
open https://crates.io/crates/lala
```

### Version Updates

```bash
# Update version in Cargo.toml
vim Cargo.toml

# Commit changes
git add Cargo.toml docs/CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"

# Tag the release
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0

# Publish to crates.io
cargo publish
```

## AUR (Arch User Repository)

### First-Time Setup

1. **Create AUR account**:
   - Register at https://aur.archlinux.org/register
   - Add SSH key to your account

2. **Clone AUR repository**:
   ```bash
   git clone ssh://aur@aur.archlinux.org/lala.git lala-aur
   cd lala-aur
   ```

### Publishing

1. **Update PKGBUILD**:
   ```bash
   cd packaging/arch/

   # Update pkgver and pkgrel
   vim PKGBUILD

   # Update checksums
   updpkgsums

   # Test build
   makepkg -si
   ```

2. **Generate .SRCINFO**:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```

3. **Commit and push**:
   ```bash
   cd ../../lala-aur/
   cp ../packaging/arch/PKGBUILD .
   cp ../packaging/arch/.SRCINFO .

   git add PKGBUILD .SRCINFO
   git commit -m "Update to version 0.1.0"
   git push
   ```

### Update Process

```bash
# Update version in PKGBUILD
cd packaging/arch/
vim PKGBUILD  # Update pkgver, increment pkgrel

# Update checksums
updpkgsums

# Test build locally
makepkg -si

# Regenerate .SRCINFO
makepkg --printsrcinfo > .SRCINFO

# Push to AUR
cd ../../lala-aur/
cp ../packaging/arch/{PKGBUILD,.SRCINFO} .
git add PKGBUILD .SRCINFO
git commit -m "Update to version X.Y.Z"
git push
```

## Debian/Ubuntu (APT)

### Prerequisites

```bash
# Install packaging tools
sudo apt install devscripts debhelper dh-make build-essential
```

### Building .deb Package

1. **Prepare source**:
   ```bash
   # Create source tarball
   cargo build --release
   tar czf lala_0.1.0.orig.tar.gz \
     --exclude=.git \
     --exclude=target \
     --transform 's,^,lala-0.1.0/,' \
     .
   ```

2. **Build package**:
   ```bash
   # Extract and build
   tar xzf lala_0.1.0.orig.tar.gz
   cd lala-0.1.0

   # Build binary package
   dpkg-buildpackage -us -uc -b

   # Or build source and binary
   dpkg-buildpackage -us -uc
   ```

3. **Test package**:
   ```bash
   # Install locally
   sudo dpkg -i ../lala_0.1.0-1_amd64.deb

   # Test
   lala --version

   # Verify dependencies
   dpkg -I lala_0.1.0-1_amd64.deb

   # Remove
   sudo apt remove lala
   ```

### Publishing to PPA (Personal Package Archive)

1. **Create Launchpad account** at https://launchpad.net

2. **Setup GPG key**:
   ```bash
   # Generate key if needed
   gpg --gen-key

   # Upload to Ubuntu keyserver
   gpg --send-keys YOUR_KEY_ID
   ```

3. **Create source package**:
   ```bash
   # Build source package
   debuild -S -sa
   ```

4. **Upload to PPA**:
   ```bash
   dput ppa:yourusername/lala lala_0.1.0-1_source.changes
   ```

## Fedora/RHEL (RPM)

### Prerequisites

```bash
# Install build tools
sudo dnf install rpm-build rpmdevtools rpmlint

# Setup build environment
rpmdev-setuptree
```

### Building RPM

1. **Prepare spec file**:
   ```bash
   cp packaging/rpm/lala.spec ~/rpmbuild/SPECS/
   ```

2. **Create source tarball**:
   ```bash
   # Create source archive
   cargo build --release
   tar czf ~/rpmbuild/SOURCES/lala-0.1.0.tar.gz \
     --exclude=.git \
     --exclude=target \
     --transform 's,^,lala-0.1.0/,' \
     .
   ```

3. **Build RPM**:
   ```bash
   # Build binary RPM
   rpmbuild -bb ~/rpmbuild/SPECS/lala.spec

   # Build source and binary RPM
   rpmbuild -ba ~/rpmbuild/SPECS/lala.spec
   ```

4. **Test RPM**:
   ```bash
   # Install
   sudo dnf install ~/rpmbuild/RPMS/x86_64/lala-0.1.0-1.fc*.x86_64.rpm

   # Test
   lala --version

   # Check package
   rpm -qi lala

   # Remove
   sudo dnf remove lala
   ```

### Publishing to COPR

1. **Create COPR account** at https://copr.fedorainfracloud.org

2. **Create project**:
   - Go to https://copr.fedorainfracloud.org/coprs/add/
   - Set project name: `lala`
   - Select chroots (Fedora versions)

3. **Upload SRPM**:
   ```bash
   # Build SRPM
   rpmbuild -bs ~/rpmbuild/SPECS/lala.spec

   # Upload to COPR
   copr-cli build yourusername/lala \
     ~/rpmbuild/SRPMS/lala-0.1.0-1.fc*.src.rpm
   ```

## Homebrew (macOS/Linux)

### For Homebrew Core (requires significant user base)

1. **Fork homebrew-core**:
   ```bash
   cd $(brew --repository homebrew/core)
   git remote add yourusername https://github.com/yourusername/homebrew-core
   ```

2. **Create formula**:
   ```bash
   # Use our template
   cp packaging/homebrew/lala.rb \
     $(brew --repository homebrew/core)/Formula/lala.rb
   ```

3. **Test formula**:
   ```bash
   brew install --build-from-source lala
   brew test lala
   brew audit --strict lala
   ```

4. **Submit PR** to https://github.com/Homebrew/homebrew-core

### For Homebrew Tap (easier alternative)

1. **Create tap repository**:
   ```bash
   # Create repo: homebrew-lala
   # at https://github.com/yourusername/homebrew-lala
   ```

2. **Add formula**:
   ```bash
   git clone https://github.com/yourusername/homebrew-lala
   cd homebrew-lala
   mkdir Formula
   cp ../lala/packaging/homebrew/lala.rb Formula/
   git add Formula/lala.rb
   git commit -m "Add lala formula"
   git push
   ```

3. **Users can install with**:
   ```bash
   brew tap yourusername/lala
   brew install lala
   ```

## Chocolatey (Windows)

### Prerequisites

```powershell
# Install Chocolatey if not installed
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Get API key from https://community.chocolatey.org/account
choco apikey --key YOUR_API_KEY --source https://push.chocolatey.org/
```

### Creating Package

1. **Prepare nuspec**:
   ```bash
   cp packaging/windows/chocolatey/lala.nuspec chocolatey/
   ```

2. **Create tools directory**:
   ```powershell
   mkdir chocolatey/tools

   # Create chocolateyinstall.ps1
   @"
   `$ErrorActionPreference = 'Stop'

   `$packageName = 'lala'
   `$toolsDir = Split-Path -Parent `$MyInvocation.MyCommand.Definition
   `$url64 = 'https://github.com/yourusername/lala/releases/download/v0.1.0/lala-windows-x86_64.zip'

   Install-ChocolateyZipPackage `
     -PackageName `$packageName `
     -Url64bit `$url64 `
     -UnzipLocation `$toolsDir `
     -Checksum64 'CHECKSUM_HERE' `
     -ChecksumType64 'sha256'
   "@ | Out-File chocolatey/tools/chocolateyinstall.ps1
   ```

3. **Pack and test**:
   ```powershell
   cd chocolatey
   choco pack

   # Test locally
   choco install lala -s . -y
   lala --version
   choco uninstall lala -y
   ```

4. **Push to Chocolatey**:
   ```powershell
   choco push lala.0.1.0.nupkg --source https://push.chocolatey.org/
   ```

## Scoop (Windows)

### For Scoop Main Bucket

1. **Fork scoop bucket**:
   ```bash
   git clone https://github.com/ScoopInstaller/Main
   ```

2. **Create manifest**:
   ```json
   {
       "version": "0.1.0",
       "description": "Modern text editor with multi-format preview",
       "homepage": "https://github.com/yourusername/lala",
       "license": "MIT OR Apache-2.0",
       "architecture": {
           "64bit": {
               "url": "https://github.com/yourusername/lala/releases/download/v0.1.0/lala-windows-x86_64.zip",
               "hash": "sha256:HASH_HERE"
           }
       },
       "bin": "lala.exe",
       "checkver": "github",
       "autoupdate": {
           "architecture": {
               "64bit": {
                   "url": "https://github.com/yourusername/lala/releases/download/v$version/lala-windows-x86_64.zip"
               }
           }
       }
   }
   ```

3. **Save and test**:
   ```powershell
   # Save as bucket/lala.json
   scoop install bucket/lala.json
   ```

4. **Submit PR** to https://github.com/ScoopInstaller/Main

## GitHub Releases

### Manual Release

1. **Tag version**:
   ```bash
   git tag -a v0.1.0 -m "Release version 0.1.0"
   git push origin v0.1.0
   ```

2. **Build binaries**:
   ```bash
   # Linux x86_64
   cargo build --release --target x86_64-unknown-linux-gnu

   # Linux musl (static)
   cargo build --release --target x86_64-unknown-linux-musl

   # macOS x86_64
   cargo build --release --target x86_64-apple-darwin

   # macOS aarch64
   cargo build --release --target aarch64-apple-darwin

   # Windows
   cargo build --release --target x86_64-pc-windows-msvc
   ```

3. **Create archives**:
   ```bash
   # Linux
   tar czf lala-linux-x86_64.tar.gz -C target/x86_64-unknown-linux-gnu/release lala

   # macOS
   tar czf lala-macos-x86_64.tar.gz -C target/x86_64-apple-darwin/release lala

   # Windows
   cd target/x86_64-pc-windows-msvc/release
   7z a ../../../lala-windows-x86_64.zip lala.exe
   ```

4. **Create checksums**:
   ```bash
   sha256sum lala-*.tar.gz lala-*.zip > SHA256SUMS
   ```

5. **Upload to GitHub**:
   ```bash
   gh release create v0.1.0 \
     --title "Release v0.1.0" \
     --notes-file docs/CHANGELOG.md \
     lala-linux-x86_64.tar.gz \
     lala-macos-x86_64.tar.gz \
     lala-windows-x86_64.zip \
     SHA256SUMS
   ```

### Automated Release (GitHub Actions)

Our `.github/workflows/release.yml` automatically:
- Builds for multiple platforms
- Creates GitHub Release
- Uploads binaries with checksums

**To trigger automated release**:
```bash
# Just push a version tag
git tag -a v0.1.0 -m "Release version 0.1.0"
git push origin v0.1.0

# GitHub Actions will handle the rest
```

## Automation

### Release Checklist Script

Create `scripts/release.sh`:

```bash
#!/bin/bash
set -e

VERSION=$1

if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.0"
    exit 1
fi

echo "🚀 Preparing release v$VERSION"

# 1. Update version in all files
echo "📝 Updating version numbers..."
sed -i "s/^version = .*/version = \"$VERSION\"/" Cargo.toml
sed -i "s/^pkgver=.*/pkgver=$VERSION/" packaging/arch/PKGBUILD
sed -i "s/^Version:.*/Version:        $VERSION/" packaging/rpm/lala.spec

# 2. Update CHANGELOG
echo "📰 Update CHANGELOG manually, then press Enter"
read

# 3. Run tests
echo "🧪 Running tests..."
cargo test --all

# 4. Build release
echo "🔨 Building release..."
cargo build --release

# 5. Commit changes
echo "💾 Committing changes..."
git add Cargo.toml packaging/ docs/CHANGELOG.md
git commit -m "chore: bump version to $VERSION"

# 6. Create tag
echo "🏷️  Creating tag..."
git tag -a "v$VERSION" -m "Release version $VERSION"

# 7. Push
echo "📤 Pushing to remote..."
git push origin main
git push origin "v$VERSION"

echo "✅ Release v$VERSION prepared!"
echo "GitHub Actions will build and publish binaries."
echo ""
echo "Next steps:"
echo "1. Wait for GitHub Actions to complete"
echo "2. Publish to crates.io: cargo publish"
echo "3. Update AUR package"
echo "4. Update other package managers as needed"
```

Make executable:
```bash
chmod +x scripts/release.sh
```

Usage:
```bash
./scripts/release.sh 0.2.0
```

## Troubleshooting

### Crates.io

**Problem**: `error: failed to verify package tarball`
```bash
# Solution: Clean and retry
cargo clean
cargo publish --dry-run
cargo publish
```

**Problem**: `error: manifest has no edition specified`
```toml
# Add to Cargo.toml
[package]
edition = "2021"
```

### AUR

**Problem**: `ERROR: One or more files did not pass the validity check!`
```bash
# Solution: Update checksums
updpkgsums
makepkg --printsrcinfo > .SRCINFO
```

**Problem**: SSH connection refused
```bash
# Solution: Check SSH key
ssh -T aur@aur.archlinux.org
```

### Debian/Ubuntu

**Problem**: `dpkg-buildpackage: error: source package has two conflicting values`
```bash
# Solution: Clean previous builds
dh_clean
dpkg-buildpackage -us -uc -b
```

**Problem**: Missing dependencies during build
```bash
# Solution: Install build dependencies
sudo apt build-dep .
```

### RPM

**Problem**: `error: Failed build dependencies`
```bash
# Solution: Install dependencies
sudo dnf builddep lala.spec
```

**Problem**: File not found in %files
```bash
# Solution: Check %files section in .spec
# Make sure paths match installed files
```

### Homebrew

**Problem**: `Error: SHA256 mismatch`
```bash
# Solution: Update formula with correct SHA256
shasum -a 256 lala-0.1.0.tar.gz
```

**Problem**: Formula audit fails
```bash
# Solution: Fix issues reported by audit
brew audit --strict --online lala
```

---

For questions or issues with publishing, open an issue on GitHub or contact the maintainers.
