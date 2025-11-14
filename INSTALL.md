# Installation Guide

## Quick Install

Choose your preferred installation method below.

---

## Method 1: Cargo (Recommended for Rust users)

The easiest way to install if you have Rust installed:

```bash
cargo install lala
```

**Update:**
```bash
cargo install --force lala
```

**Uninstall:**
```bash
cargo uninstall lala
```

---

## Method 2: Pre-built Binaries

Download pre-compiled binaries from [GitHub Releases](https://github.com/yourusername/lala/releases/latest).

### Linux

```bash
# Download latest release
wget https://github.com/yourusername/lala/releases/latest/download/lala-linux-x86_64.tar.gz

# Extract
tar xzf lala-linux-x86_64.tar.gz

# Install (requires sudo)
sudo mv lala /usr/local/bin/

# Verify installation
lala --version
```

**Alternative: User-local installation (no sudo needed)**
```bash
mkdir -p ~/.local/bin
mv lala ~/.local/bin/
# Add to PATH in ~/.bashrc or ~/.zshrc:
export PATH="$HOME/.local/bin:$PATH"
```

### macOS

```bash
# Intel Macs
wget https://github.com/yourusername/lala/releases/latest/download/lala-macos-x86_64.tar.gz
tar xzf lala-macos-x86_64.tar.gz

# Apple Silicon (M1/M2/M3)
wget https://github.com/yourusername/lala/releases/latest/download/lala-macos-aarch64.tar.gz
tar xzf lala-macos-aarch64.tar.gz

# Install
sudo mv lala /usr/local/bin/

# Verify
lala --version
```

### Windows

1. Download `lala-windows-x86_64.zip` from [Releases](https://github.com/yourusername/lala/releases/latest)
2. Extract the zip file
3. Move `lala.exe` to a directory in your PATH

Or use PowerShell:
```powershell
# Download
Invoke-WebRequest -Uri "https://github.com/yourusername/lala/releases/latest/download/lala-windows-x86_64.zip" -OutFile "lala.zip"

# Extract
Expand-Archive -Path lala.zip -DestinationPath .

# Move to a directory in PATH (e.g., C:\Program Files\lala)
```

---

## Method 3: Package Managers

### Arch Linux (AUR)

```bash
# Using yay
yay -S lala

# Using paru
paru -S lala

# Manual installation
git clone https://aur.archlinux.org/lala.git
cd lala
makepkg -si
```

### Debian/Ubuntu (PPA)

```bash
# Add PPA
sudo add-apt-repository ppa:username/lala
sudo apt update

# Install
sudo apt install lala
```

**Alternative: Install .deb file**
```bash
# Download .deb file
wget https://github.com/yourusername/lala/releases/latest/download/lala_0.1.0-1_amd64.deb

# Install
sudo dpkg -i lala_0.1.0-1_amd64.deb

# Fix dependencies if needed
sudo apt-get install -f
```

### Homebrew (macOS/Linux)

```bash
# Add tap
brew tap yourusername/lala

# Install
brew install lala
```

---

## Method 4: Build from Source

### Prerequisites

- **Rust**: 1.70 or later
- **Cargo**: Included with Rust
- **System libraries** (Linux only):
  - libxcb1-dev
  - libxcb-render0-dev
  - libxcb-shape0-dev
  - libxcb-xfixes0-dev

### Installation Steps

```bash
# Clone repository
git clone https://github.com/yourusername/lala.git
cd lala

# Build release binary
cargo build --release

# Run tests (optional)
cargo test --all

# Install to system
sudo cp target/release/lala /usr/local/bin/

# Or install for current user only
mkdir -p ~/.local/bin
cp target/release/lala ~/.local/bin/
```

---

## Verify Installation

After installation, verify that lala is working:

```bash
# Check version
lala --version

# Show help
lala --help

# Test Markdown preview
echo "# Test" > test.md
lala markdown test.md
```

---

## Platform-Specific Notes

### Linux

**GUI Mode Requirements:**
- X11 or Wayland display server
- OpenGL support

**Dependencies (Debian/Ubuntu):**
```bash
sudo apt install libxcb1 libxcb-render0 libxcb-shape0 libxcb-xfixes0
```

**Dependencies (Arch Linux):**
```bash
sudo pacman -S libxcb
```

### macOS

**Security Notice:**
On first run, you may see "lala cannot be opened because it is from an unidentified developer."

**Solution:**
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine /usr/local/bin/lala

# Or open via System Preferences > Security & Privacy
```

### Windows

**GUI Mode:**
- Requires Windows 10 or later
- OpenGL support recommended

**Terminal:**
- Windows Terminal recommended for best color support
- CMD.exe works but with limited colors

---

## Update

### Cargo
```bash
cargo install --force lala
```

### Package Managers
```bash
# Arch Linux
yay -Syu lala

# Debian/Ubuntu
sudo apt update && sudo apt upgrade lala

# Homebrew
brew upgrade lala
```

### Binary
Download the latest release and replace the existing binary.

---

## Uninstall

### Cargo
```bash
cargo uninstall lala
```

### Package Managers
```bash
# Arch Linux
sudo pacman -R lala

# Debian/Ubuntu
sudo apt remove lala

# Homebrew
brew uninstall lala
```

### Manual Binary
```bash
# If installed to /usr/local/bin
sudo rm /usr/local/bin/lala

# If installed to ~/.local/bin
rm ~/.local/bin/lala
```

---

## Troubleshooting

### "Command not found"

**Check if lala is in PATH:**
```bash
which lala
echo $PATH
```

**Add to PATH (if needed):**
```bash
# Add to ~/.bashrc or ~/.zshrc:
export PATH="$HOME/.local/bin:$PATH"
```

### GUI doesn't start (Linux)

**Check display:**
```bash
echo $DISPLAY
```

**Try:**
```bash
export DISPLAY=:0
lala
```

### Missing library errors (Linux)

**Install dependencies:**
```bash
# Debian/Ubuntu
sudo apt install libxcb1 libxcb-render0 libxcb-shape0 libxcb-xfixes0

# Arch Linux
sudo pacman -S libxcb
```

### Cargo build fails

**Update Rust:**
```bash
rustup update stable
```

**Clear cache:**
```bash
cargo clean
cargo build --release
```

---

## Getting Help

- **Documentation**: See [README.md](README.md)
- **CLI Usage**: See [CLI_USAGE.md](CLI_USAGE.md)
- **Format Support**: See [FORMAT_SUPPORT.md](FORMAT_SUPPORT.md)
- **Issues**: [GitHub Issues](https://github.com/yourusername/lala/issues)

---

## Next Steps

After installation, check out:
- [CLI Usage Guide](CLI_USAGE.md) - Learn CLI commands
- [Format Support](FORMAT_SUPPORT.md) - Understand supported formats
- [README](README.md) - Full documentation

**Quick Start:**
```bash
# Launch GUI
lala

# Preview Markdown
lala markdown README.md

# Preview HTML
lala html page.html

# Preview LaTeX
lala latex document.tex

# Preview Mermaid diagrams
lala mermaid diagram.mmd
```

Enjoy using Lala! 🎉
