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
sed -i "s/<version>.*<\/version>/<version>$VERSION<\/version>/" packaging/windows/chocolatey/lala.nuspec

# Update NSIS installer version
MAJOR=$(echo $VERSION | cut -d. -f1)
MINOR=$(echo $VERSION | cut -d. -f2)
BUILD=$(echo $VERSION | cut -d. -f3)
sed -i "s/^!define VERSIONMAJOR.*/!define VERSIONMAJOR $MAJOR/" packaging/windows/installer.nsi
sed -i "s/^!define VERSIONMINOR.*/!define VERSIONMINOR $MINOR/" packaging/windows/installer.nsi
sed -i "s/^!define VERSIONBUILD.*/!define VERSIONBUILD $BUILD/" packaging/windows/installer.nsi

# Update Homebrew formula
sed -i "s/version \".*\"/version \"$VERSION\"/" packaging/homebrew/lala.rb

# 2. Update CHANGELOG
echo "📰 Please update docs/CHANGELOG.md with release notes"
echo "Press Enter when done..."
read

# 3. Run tests
echo "🧪 Running tests..."
cargo test --all

# 4. Run clippy
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# 5. Build release
echo "🔨 Building release..."
cargo build --release

# 6. Verify binary
echo "✅ Testing binary..."
./target/release/lala --version

# 7. Show changes
echo "📋 Changes to be committed:"
git status

echo ""
echo "Do you want to commit these changes? (y/n)"
read -r response

if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    # Commit changes
    echo "💾 Committing changes..."
    git add Cargo.toml Cargo.lock packaging/ docs/CHANGELOG.md
    git commit -m "chore: bump version to $VERSION"

    # Create tag
    echo "🏷️  Creating tag..."
    git tag -a "v$VERSION" -m "Release version $VERSION"

    # Show next steps
    echo ""
    echo "✅ Release v$VERSION prepared!"
    echo ""
    echo "📌 Next steps:"
    echo "1. Review the commit: git show"
    echo "2. Push to remote: git push origin main && git push origin v$VERSION"
    echo "3. GitHub Actions will automatically:"
    echo "   - Build binaries for all platforms"
    echo "   - Create GitHub Release"
    echo "   - Upload binaries and checksums"
    echo ""
    echo "4. Manual steps after GitHub Actions completes:"
    echo "   - Publish to crates.io: cargo publish"
    echo "   - Update AUR package (see docs/development/publishing-guide.md)"
    echo "   - Update other package managers as needed"
else
    echo "❌ Commit cancelled. Changes are staged but not committed."
    echo "You can review and commit manually."
fi
