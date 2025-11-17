# IME (日本語入力) トラブルシューティング

## 症状: 日本語入力が表示されない

**最も簡単な解決方法:**
```bash
./lala-ime-debug.sh
```

このスクリプトは以下を自動的に実行します：
- ibus-daemonの再起動
- すべての環境変数の設定（GLFW_IM_MODULEを含む）
- 診断情報の表示
- 正しい設定でlalaを起動

---

## 手動トラブルシューティング

### 問題の確認

1. **ibus-daemonが起動しているか確認**
```bash
pgrep -a ibus-daemon
# 出力例: 2362 /usr/bin/ibus-daemon --daemonize --xim
```

2. **環境変数が設定されているか確認**
```bash
env | grep -E "GTK_IM_MODULE|XMODIFIERS|QT_IM_MODULE|GLFW_IM_MODULE"
```

必要な環境変数:
- GTK_IM_MODULE=ibus
- XMODIFIERS=@im=ibus
- QT_IM_MODULE=ibus
- GLFW_IM_MODULE=ibus ← **これが重要！**

### 解決方法 1: 環境変数を設定して起動

```bash
# すべての環境変数を明示的に設定
export GTK_IM_MODULE=ibus
export XMODIFIERS=@im=ibus
export QT_IM_MODULE=ibus
export GLFW_IM_MODULE=ibus

# lalaを起動
./target/release/lala
```

### 解決方法 2: 起動スクリプトを使用

```bash
# 自動的に環境変数を設定してくれる
./lala-ime.sh
```

### 解決方法 3: ibus-daemonを再起動

```bash
# 既存のibus-daemonを終了
killall ibus-daemon

# 再起動
ibus-daemon -drx

# 環境変数を設定
export GTK_IM_MODULE=ibus
export XMODIFIERS=@im=ibus
export QT_IM_MODULE=ibus
export GLFW_IM_MODULE=ibus

# lalaを起動
./target/release/lala
```

### 解決方法 4: XIMではなくGTKネイティブIMを試す

```bash
# GTK3アプリケーションの場合
export GTK_IM_MODULE=ibus

./target/release/lala
```

## よくある問題

### 問題: 「日本語入力」と表示されるが文字が入力できない

**原因**: GLFW_IM_MODULEが設定されていない、またはeguiのIME処理が完全でない

**解決策**:
```bash
# ターミナルで実行
export GLFW_IM_MODULE=ibus
./target/release/lala
```

### 問題: 文字が浮いてしまう (IME候補ウィンドウの位置がずれる)

**原因**: eguiのIMEカーソル位置の報告が正しくない

**解決策**: 現在egui 0.33で修正中。将来のバージョンで改善予定。

### 問題: バックスペースやカーソルキーが動かない

**原因**: egui 0.29.0-0.32.xのバグ

**解決策**: egui 0.33にアップグレード済み (現在のバージョン)

## デバッグ情報の収集

```bash
# 1. ディスプレイサーバーの確認
echo $XDG_SESSION_TYPE
# 出力: x11 または wayland

# 2. ibusの状態確認
ibus version
ibus list-engine

# 3. mozc エンジンが利用可能か確認
ibus list-engine | grep mozc

# 4. 現在の入力メソッド確認
gsettings get org.gnome.desktop.input-sources sources
```

## 確実に動作させる方法

プロジェクトに含まれている `lala-ime-debug.sh` スクリプトを使用してください：

```bash
./lala-ime-debug.sh
```

このスクリプトは以下を実行します：
1. ibus-daemonを完全に再起動
2. すべての環境変数を設定（GTK_IM_MODULE, XMODIFIERS, QT_IM_MODULE, GLFW_IM_MODULE）
3. 診断情報を表示（ディスプレイサーバー、環境変数、ibus状態、mozcエンジン）
4. lalaを正しい設定で起動

スクリプトの内容を確認したい場合は、ファイルを直接開いてください。

## それでも動作しない場合

egui 0.33でもLinux IMEサポートは完全ではありません。以下の代替案を検討してください：

1. **CLIモードを使用**: Markdown/HTML/LaTeXのプレビューはCLIモードでも可能
2. **別のエディタで編集してプレビューのみlalaを使用**: 編集は他のエディタで、プレビューのみlalaのCLIモードで表示
3. **Issue報告**: https://github.com/emilk/egui/issues にIME問題を報告

## 参考リンク

- egui IME Support Issue: https://github.com/emilk/egui/issues/248
- Linux IME Fix PR: https://github.com/emilk/egui/pull/5198
- ibus Documentation: https://github.com/ibus/ibus/wiki
