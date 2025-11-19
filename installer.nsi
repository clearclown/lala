; Lala Installer Script

!define PRODUCT_NAME "Lala"
!define PRODUCT_VERSION "0.2.0"
!define PRODUCT_PUBLISHER "clearclown"
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
