#!/bin/bash
# Lala IME Debug Script
# このスクリプトは日本語入力の問題を診断し、正しい環境でlalaを起動します

echo "============================================"
echo "Lala IME デバッグスクリプト"
echo "============================================"
echo ""

# ibus-daemonを完全に再起動
echo "【1】ibus-daemon を再起動しています..."
killall ibus-daemon 2>/dev/null
sleep 1
ibus-daemon -drx
sleep 2

# すべての環境変数を設定
echo "【2】環境変数を設定しています..."
export GTK_IM_MODULE=ibus
export XMODIFIERS=@im=ibus
export QT_IM_MODULE=ibus
export GLFW_IM_MODULE=ibus

# デバッグ情報を表示
echo ""
echo "============================================"
echo "IME環境確認"
echo "============================================"
echo "ディスプレイサーバー: $XDG_SESSION_TYPE"
echo ""
echo "【環境変数】"
echo "  GTK_IM_MODULE:  $GTK_IM_MODULE"
echo "  XMODIFIERS:     $XMODIFIERS"
echo "  QT_IM_MODULE:   $QT_IM_MODULE"
echo "  GLFW_IM_MODULE: $GLFW_IM_MODULE"
echo ""
echo "【ibus-daemon】"
pgrep -a ibus-daemon
echo ""
echo "【ibus バージョン】"
ibus version
echo ""
echo "【mozc エンジン】"
ibus list-engine | grep mozc
echo ""
echo "============================================"
echo ""

# 実行ファイルの存在確認
if [ ! -f "./target/release/lala" ]; then
    echo "エラー: ./target/release/lala が見つかりません"
    echo "まず 'cargo build --release' を実行してください"
    exit 1
fi

echo "【3】lalaを起動します..."
echo "注意: Ctrl+Space で日本語入力に切り替えてください"
echo ""

# lalaを起動
exec ./target/release/lala
