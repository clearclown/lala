#!/usr/bin/env bash
#
# Lala Editor Universal Installer for Linux/macOS
#
# Usage: curl -sSL https://raw.githubusercontent.com/yourusername/lala/main/packaging/scripts/install.sh | bash

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="yourusername/lala"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
VERSION="${VERSION:-latest}"

# Functions
print_info() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}==>${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}==>${NC} $1"
}

print_error() {
    echo -e "${RED}Error:${NC} $1" >&2
}

detect_platform() {
    local os arch

    # Detect OS
    case "$(uname -s)" in
        Linux*)     os="linux" ;;
        Darwin*)    os="macos" ;;
        *)
            print_error "Unsupported operating system: $(uname -s)"
            exit 1
            ;;
    esac

    # Detect architecture
    case "$(uname -m)" in
        x86_64)     arch="x86_64" ;;
        arm64|aarch64) arch="aarch64" ;;
        *)
            print_error "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac

    echo "${os}-${arch}"
}

get_download_url() {
    local platform=$1
    local version=$2

    if [ "$version" = "latest" ]; then
        version=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    fi

    local asset_name
    case "$platform" in
        linux-x86_64)
            asset_name="lala-linux-x86_64.tar.gz"
            ;;
        linux-aarch64)
            asset_name="lala-linux-aarch64.tar.gz"
            ;;
        macos-x86_64)
            asset_name="lala-macos-x86_64.tar.gz"
            ;;
        macos-aarch64)
            asset_name="lala-macos-aarch64.tar.gz"
            ;;
        *)
            print_error "No binary available for platform: $platform"
            exit 1
            ;;
    esac

    echo "https://github.com/$REPO/releases/download/$version/$asset_name"
}

install_lala() {
    print_info "Installing Lala Editor..."

    # Detect platform
    local platform
    platform=$(detect_platform)
    print_info "Detected platform: $platform"

    # Get download URL
    local download_url
    download_url=$(get_download_url "$platform" "$VERSION")
    print_info "Download URL: $download_url"

    # Create temporary directory
    local tmp_dir
    tmp_dir=$(mktemp -d)
    trap "rm -rf $tmp_dir" EXIT

    # Download
    print_info "Downloading..."
    if command -v curl > /dev/null; then
        curl -sSL "$download_url" -o "$tmp_dir/lala.tar.gz"
    elif command -v wget > /dev/null; then
        wget -q "$download_url" -O "$tmp_dir/lala.tar.gz"
    else
        print_error "Neither curl nor wget found. Please install one of them."
        exit 1
    fi

    # Extract
    print_info "Extracting..."
    tar xzf "$tmp_dir/lala.tar.gz" -C "$tmp_dir"

    # Install
    print_info "Installing to $INSTALL_DIR..."
    mkdir -p "$INSTALL_DIR"
    mv "$tmp_dir/lala" "$INSTALL_DIR/lala"
    chmod +x "$INSTALL_DIR/lala"

    # Check if install directory is in PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        print_warning "$INSTALL_DIR is not in your PATH"
        print_warning "Add the following line to your ~/.bashrc or ~/.zshrc:"
        echo ""
        echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
        echo ""
    fi

    print_success "Lala Editor installed successfully!"
    print_success "Run 'lala --help' to get started"
}

# Check for package managers
check_package_manager() {
    if command -v cargo > /dev/null; then
        print_info "Detected Rust/Cargo. You can also install with:"
        echo "    cargo install lala"
        echo ""
    fi

    case "$(uname -s)" in
        Linux*)
            if command -v apt > /dev/null; then
                print_info "For Debian/Ubuntu, you can use APT:"
                echo "    sudo apt install lala"
                echo ""
            elif command -v pacman > /dev/null; then
                print_info "For Arch Linux, you can use AUR:"
                echo "    yay -S lala"
                echo ""
            elif command -v dnf > /dev/null; then
                print_info "For Fedora/RHEL, you can use DNF:"
                echo "    sudo dnf install lala"
                echo ""
            fi
            ;;
        Darwin*)
            if command -v brew > /dev/null; then
                print_info "For macOS, you can use Homebrew:"
                echo "    brew install lala"
                echo ""
            fi
            ;;
    esac
}

# Main
main() {
    echo "Lala Editor Installer"
    echo "===================="
    echo ""

    check_package_manager
    install_lala
}

main "$@"
