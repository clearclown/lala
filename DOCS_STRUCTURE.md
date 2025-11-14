# Documentation Structure

## Directory Layout

```
lala/
├── README.md                    # Main README (English)
├── docs/
│   ├── README_ja.md            # 日本語README
│   ├── README_fa.md            # README فارسی (Persian)
│   ├── README_ar.md            # README العربية (Arabic)
│   ├── README_zh-CN.md         # 简体中文README
│   ├── README_zh-TW.md         # 繁體中文README
│   ├── README_ru.md            # README на русском (Russian)
│   │
│   ├── en/                     # English documentation
│   │   ├── install.md
│   │   ├── cli-usage.md
│   │   ├── format-support.md
│   │   ├── packaging.md
│   │   └── contributing.md
│   │
│   ├── ja/                     # 日本語ドキュメント
│   │   ├── 初心者ガイド.md
│   │   ├── インストール.md
│   │   ├── CLI使い方.md
│   │   ├── フォーマット対応.md
│   │   ├── パッケージング.md
│   │   └── トラブルシューティング.md
│   │
│   ├── fa/                     # مستندات فارسی
│   ├── ar/                     # الوثائق العربية
│   ├── zh-CN/                  # 简体中文文档
│   ├── zh-TW/                  # 繁體中文文檔
│   ├── ru/                     # Русская документация
│   │
│   └── images/                 # Screenshots and diagrams
│       ├── screenshots/
│       ├── diagrams/
│       └── logos/
│
├── packaging/
│   ├── arch/
│   │   └── PKGBUILD
│   ├── debian/
│   │   ├── control
│   │   ├── changelog
│   │   ├── rules
│   │   └── compat
│   ├── rpm/
│   │   └── lala.spec
│   ├── homebrew/
│   │   └── lala.rb
│   ├── windows/
│   │   ├── installer.nsi        # NSIS installer script
│   │   └── chocolatey/
│   │       └── lala.nuspec
│   └── scripts/
│       ├── install.sh           # Universal installer
│       ├── install.ps1          # Windows installer
│       └── uninstall.sh
│
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── release.yml
│       └── package.yml          # Package building workflow
│
└── [existing files...]
```

## Language Support

### Available Languages

1. **English (en)** - Primary language
2. **日本語 (ja)** - Japanese
3. **فارسی (fa)** - Persian/Farsi
4. **العربية (ar)** - Arabic
5. **简体中文 (zh-CN)** - Simplified Chinese
6. **繁體中文 (zh-TW)** - Traditional Chinese
7. **Русский (ru)** - Russian

## Installation Methods

### Package Managers

| Method | Command | Platform | Status |
|--------|---------|----------|--------|
| Cargo | `cargo install lala` | All | ✅ Ready |
| APT | `apt install lala` | Debian/Ubuntu | ✅ Package ready |
| Pacman | `pacman -S lala` | Arch Linux | ✅ PKGBUILD ready |
| YUM/DNF | `dnf install lala` | Fedora/RHEL | 🔄 RPM in progress |
| Homebrew | `brew install lala` | macOS/Linux | ✅ Formula ready |
| Chocolatey | `choco install lala` | Windows | 🔄 In progress |
| Scoop | `scoop install lala` | Windows | 🔄 Planned |

### Binary Downloads

- Linux: .tar.gz, .deb, .rpm, .AppImage
- macOS: .tar.gz, .dmg
- Windows: .zip, .msi, .exe installer

### Installation Scripts

- `install.sh` - Universal Linux/macOS installer
- `install.ps1` - Windows PowerShell installer
- `uninstall.sh` - Uninstaller script

## Documentation Organization

### Beginner Documentation (Japanese)

特に日本語の初心者向けドキュメントには以下を含む：

1. **初心者ガイド.md**
   - Lalaエディタとは
   - なぜLalaを使うのか
   - 基本的な概念
   - 最初の一歩

2. **インストール.md**
   - システム要件
   - インストール方法（詳細）
   - 初期設定
   - 動作確認

3. **CLI使い方.md**
   - コマンドライン基礎
   - 各コマンドの使い方
   - 実用例
   - Tips & Tricks

4. **フォーマット対応.md**
   - Markdown
   - HTML
   - Mermaid
   - LaTeX

5. **トラブルシューティング.md**
   - よくある問題と解決方法
   - エラーメッセージの意味
   - FAQ

## Maintenance

- Keep all language versions synchronized
- Update screenshots regularly
- Maintain version-specific documentation
- Regular review of beginner docs for clarity
