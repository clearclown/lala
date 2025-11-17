# 🪟 Windows (.exe) リリースガイド

**難易度**: ★★★☆☆ (中級)
**推定時間**: 2-4時間
**優先度**: Phase 3

---

## 🎯 概要

Windows向けにインストーラー(.exe)を作成すると、Windowsユーザーが簡単にインストールできるようになります。
MSIインストーラーまたはNSISインストーラーを作成できます。

### メリット
- Windowsユーザーに馴染みがある
- スタートメニュー統合
- アンインストーラーの自動生成
- コード署名でセキュリティ向上（オプション）

---

## 📋 前提条件

### 1. Windows向けクロスコンパイル環境

#### Linux上でクロスコンパイル

```bash
# crossツールを使用
cargo install cross

# または直接Rustターゲットを追加
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-pc-windows-msvc  # MSVCツールチェーン必要

# MinGWをインストール
sudo apt install mingw-w64
```

#### Windows上で直接ビルド

```powershell
# Rustをインストール（rustup-init.exe）
# https://rustup.rs/

# ビルドツールをインストール
# Visual Studio Build Tools または Visual Studio Community
```

### 2. インストーラー作成ツール

以下のいずれかを選択：

**Option A: WiX Toolset (MSI)**
```powershell
# Windows上で
winget install wixtoolset.wix
# または https://wixtoolset.org/

# cargo-wixをインストール
cargo install cargo-wix
```

**Option B: NSIS (EXE)**
```bash
# Linux
sudo apt install nsis

# Windows
# https://nsis.sourceforge.io/Download
```

---

## 🏗️ Option A: WiX Toolset でMSIインストーラーを作成

### Step 1: WiX設定の初期化

```bash
# プロジェクトルートで
cargo wix init
```

これにより`wix/main.wxs`が作成されます。

### Step 2: main.wxsの編集

```xml
<?xml version='1.0' encoding='windows-1252'?>
<Wix xmlns='http://schemas.microsoft.com/wix/2006/wi'>
  <Product
    Id='*'
    Name='Lala'
    UpgradeCode='YOUR-GUID-HERE'
    Manufacturer='Your Name'
    Language='1033'
    Codepage='1252'
    Version='0.1.0'>

    <Package
      Id='*'
      Keywords='Installer'
      Description='Lala Text Editor Installer'
      Manufacturer='Your Name'
      InstallerVersion='450'
      Languages='1033'
      Compressed='yes'
      SummaryCodepage='1252' />

    <Media Id='1' Cabinet='media1.cab' EmbedCab='yes' />

    <Directory Id='TARGETDIR' Name='SourceDir'>
      <Directory Id='ProgramFilesFolder'>
        <Directory Id='APPLICATIONFOLDER' Name='Lala'>

          <!-- バイナリ -->
          <Component Id='binary' Guid='YOUR-GUID-HERE'>
            <File
              Id='LalaEXE'
              Name='lala.exe'
              DiskId='1'
              Source='target\release\lala.exe'
              KeyPath='yes' />
          </Component>

          <!-- スタートメニューショートカット -->
          <Component Id='ApplicationShortcut' Guid='YOUR-GUID-HERE'>
            <Shortcut
              Id='ApplicationStartMenuShortcut'
              Name='Lala'
              Description='Modern Text Editor'
              Target='[APPLICATIONFOLDER]lala.exe'
              WorkingDirectory='APPLICATIONFOLDER' />
            <RemoveFolder Id='APPLICATIONFOLDER' On='uninstall' />
            <RegistryValue
              Root='HKCU'
              Key='Software\Lala'
              Name='installed'
              Type='integer'
              Value='1'
              KeyPath='yes' />
          </Component>

        </Directory>
      </Directory>

      <!-- スタートメニュー -->
      <Directory Id='ProgramMenuFolder'>
        <Directory Id='ApplicationProgramsFolder' Name='Lala' />
      </Directory>
    </Directory>

    <!-- 機能 -->
    <Feature Id='Complete' Level='1'>
      <ComponentRef Id='binary' />
      <ComponentRef Id='ApplicationShortcut' />
    </Feature>

    <!-- アイコン -->
    <Icon Id='ProductIcon' SourceFile='assets\icon.ico' />
    <Property Id='ARPPRODUCTICON' Value='ProductIcon' />

    <!-- Add/Removeプログラム -->
    <Property Id='ARPHELPLINK' Value='https://github.com/clearclown/lala' />
  </Product>
</Wix>
```

**GUIDの生成**:
```powershell
# PowerShellで
[guid]::NewGuid()

# またはオンラインツール
# https://www.guidgenerator.com/
```

### Step 3: アイコンの準備

```bash
# PNGからICOに変換
# ImageMagickを使用
convert assets/icon.png -define icon:auto-resize=256,128,64,48,32,16 assets/icon.ico

# またはオンラインツール
# https://convertio.co/png-ico/
```

### Step 4: ビルド

```bash
# Windows向けにビルド（Linux上で）
cross build --release --target x86_64-pc-windows-gnu

# MSIインストーラーを生成（Windows上で）
cargo wix

# 生成されたファイル: target/wix/lala-0.1.0-x86_64.msi
```

---

## 🏗️ Option B: NSIS でEXEインストーラーを作成

### Step 1: NSISスクリプトの作成

```bash
cat > installer.nsi << 'EOF'
; Lala Installer Script

!define PRODUCT_NAME "Lala"
!define PRODUCT_VERSION "0.1.0"
!define PRODUCT_PUBLISHER "Your Name"
!define PRODUCT_WEB_SITE "https://github.com/clearclown/lala"

; Modern UI
!include "MUI2.nsh"

; General
Name "${PRODUCT_NAME} ${PRODUCT_VERSION}"
OutFile "lala-setup-${PRODUCT_VERSION}.exe"
InstallDir "$PROGRAMFILES\Lala"
InstallDirRegKey HKLM "Software\Lala" "Install_Dir"
RequestExecutionLevel admin

; Interface Settings
!define MUI_ABORTWARNING
!define MUI_ICON "assets\icon.ico"
!define MUI_UNICON "assets\icon.ico"

; Pages
!insertmacro MUI_PAGE_LICENSE "LICENSE-MIT"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

; Languages
!insertmacro MUI_LANGUAGE "English"

; Installer Section
Section "Install"
  SetOutPath "$INSTDIR"

  ; ファイルのコピー
  File "target\x86_64-pc-windows-gnu\release\lala.exe"
  File "README.md"
  File "LICENSE-MIT"
  File "LICENSE-APACHE"

  ; レジストリキー
  WriteRegStr HKLM "Software\Lala" "Install_Dir" "$INSTDIR"

  ; アンインストーラー
  WriteUninstaller "$INSTDIR\Uninstall.exe"

  ; スタートメニューショートカット
  CreateDirectory "$SMPROGRAMS\Lala"
  CreateShortcut "$SMPROGRAMS\Lala\Lala.lnk" "$INSTDIR\lala.exe"
  CreateShortcut "$SMPROGRAMS\Lala\Uninstall.lnk" "$INSTDIR\Uninstall.exe"

  ; Add/Remove Programs
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Lala" \
    "DisplayName" "${PRODUCT_NAME}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Lala" \
    "UninstallString" "$INSTDIR\Uninstall.exe"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Lala" \
    "DisplayIcon" "$INSTDIR\lala.exe"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Lala" \
    "Publisher" "${PRODUCT_PUBLISHER}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Lala" \
    "URLInfoAbout" "${PRODUCT_WEB_SITE}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Lala" \
    "DisplayVersion" "${PRODUCT_VERSION}"
SectionEnd

; Uninstaller Section
Section "Uninstall"
  ; ファイルの削除
  Delete "$INSTDIR\lala.exe"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\LICENSE-MIT"
  Delete "$INSTDIR\LICENSE-APACHE"
  Delete "$INSTDIR\Uninstall.exe"

  ; ディレクトリの削除
  RMDir "$INSTDIR"

  ; スタートメニューの削除
  Delete "$SMPROGRAMS\Lala\*.*"
  RMDir "$SMPROGRAMS\Lala"

  ; レジストリの削除
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\Lala"
  DeleteRegKey HKLM "Software\Lala"
SectionEnd
EOF
```

### Step 2: ビルド

```bash
# Windows向けにビルド
cross build --release --target x86_64-pc-windows-gnu

# NSISでインストーラーを生成
makensis installer.nsi

# 生成されたファイル: lala-setup-0.1.0.exe
```

---

## 🧪 テスト

### Windows上でテスト

```powershell
# インストーラーを実行
.\lala-setup-0.1.0.exe

# または MSI
msiexec /i lala-0.1.0-x86_64.msi

# インストール確認
lala --version

# アンインストール
# コントロールパネル → プログラムと機能 → Lala → アンインストール
```

---

## 🔐 コード署名（オプション）

コード署名により、Windowsセキュリティ警告を回避できます。

### Step 1: コード署名証明書の取得

証明書プロバイダー:
- DigiCert
- Sectigo
- GlobalSign

価格: 年間 $100-400

### Step 2: 署名

```powershell
# signtoolを使用（Windows SDK に含まれる）
signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com lala.exe

# または
signtool sign /sha1 THUMBPRINT /tr http://timestamp.digicert.com /td sha256 lala.exe
```

---

## 📤 配布

### GitHub Releasesで配布

```bash
# GitHub Releasesにアップロード
gh release upload v0.1.0 lala-setup-0.1.0.exe
# または
gh release upload v0.1.0 target/wix/lala-0.1.0-x86_64.msi
```

**ユーザーへの案内**:
```markdown
## Installation on Windows

### Using Installer (Recommended)

1. Download [lala-setup-0.1.0.exe](https://github.com/clearclown/lala/releases/download/v0.1.0/lala-setup-0.1.0.exe)
2. Run the installer
3. Follow the installation wizard
4. Launch from Start Menu

### Using MSI

1. Download [lala-0.1.0-x86_64.msi](https://github.com/clearclown/lala/releases/download/v0.1.0/lala-0.1.0-x86_64.msi)
2. Double-click to install
3. Launch from Start Menu

### Portable Version

1. Download [lala-windows-x86_64.exe](https://github.com/clearclown/lala/releases/download/v0.1.0/lala-windows-x86_64.exe)
2. Rename to `lala.exe`
3. Run directly (no installation needed)
```

---

## 🔄 バージョンアップデート

### WiX の場合

```xml
<!-- main.wxs のバージョンを更新 -->
<Product
  Version='0.1.1'>  <!-- 更新 -->
```

### NSIS の場合

```nsi
; installer.nsi のバージョンを更新
!define PRODUCT_VERSION "0.1.1"
```

### 再ビルド

```bash
# ビルド
cross build --release --target x86_64-pc-windows-gnu

# インストーラー生成
cargo wix  # または makensis installer.nsi

# アップロード
gh release upload v0.1.1 lala-setup-0.1.1.exe
```

---

## 📊 高度な設定

### 自動更新機能

自動更新を実装する場合：

```rust
// Cargo.toml
[dependencies]
self_update = "0.39"

// src/main.rs
#[cfg(target_os = "windows")]
fn check_for_updates() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("clearclown")
        .repo_name("lala")
        .bin_name("lala")
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    println!("Update status: `{}`!", status.version());
    Ok(())
}
```

### Windows Defenderの例外

SmartScreenフィルターを避けるには：
1. コード署名証明書を使用
2. 十分なダウンロード数を獲得（Microsoftが自動的に信頼）

---

## 🚨 トラブルシューティング

### エラー: "LINK : fatal error LNK1181: cannot open input file 'windows.lib'"

**原因**: MSVCツールチェーンが見つからない

**解決策**:
```bash
# MinGWターゲットを使用
cross build --release --target x86_64-pc-windows-gnu
```

### エラー: "The code execution cannot proceed because VCRUNTIME140.dll was not found"

**原因**: Visual C++ Redistributableが不足

**解決策**:
```nsi
; installer.nsi に追加
Section "VC++ Redistributable"
  File "vcredist_x64.exe"
  ExecWait "$INSTDIR\vcredist_x64.exe /quiet /norestart"
  Delete "$INSTDIR\vcredist_x64.exe"
SectionEnd
```

### 警告: "Windows protected your PC"

**原因**: 署名されていない実行ファイル

**解決策**:
1. コード署名証明書で署名
2. ユーザーに「詳細情報」→「実行」をクリックするよう案内

### ビルドサイズが大きい

**解決策**:
```toml
# Cargo.toml
[profile.release]
strip = true
lto = true
codegen-units = 1
opt-level = "z"

# UPXで圧縮（オプション）
# upx --best --lzma lala.exe
```

---

## 📚 参考リンク

- [WiX Toolset Documentation](https://wixtoolset.org/documentation/)
- [cargo-wix](https://github.com/volks73/cargo-wix)
- [NSIS Documentation](https://nsis.sourceforge.io/Docs/)
- [Cross-compilation](https://rust-lang.github.io/rustup/cross-compilation.html)

---

## ✅ 完了チェック

Windowsリリースが完了したら：

- [ ] Windows向けにビルドできる
- [ ] インストーラー（MSIまたはNSIS）が作成されている
- [ ] アイコンが設定されている
- [ ] Windows上でインストールテストができる
- [ ] スタートメニューにショートカットが作成される
- [ ] アンインストールが正常に動作する
- [ ] GitHub Releasesにインストーラーがアップロードされている
- [ ] READMEにインストール手順が記載されている
- [ ] （オプション）コード署名されている
- [ ] todo.md の「Windows (.exe)」にチェックを入れる

---

**完了**: すべてのプラットフォームのリリースガイドが完成しました！🎉
Phase 1から順番に進めてください。
