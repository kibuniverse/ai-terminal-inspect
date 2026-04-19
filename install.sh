#!/bin/bash

# 发生错误时退出
set -e

REPO="kibuniverse/ai-terminal-inspect"
APP_NAME="ai-terminal-inspect"
SHORT_NAME="ati"
INSTALL_DIR="$HOME/.local/bin"

echo "🚀 开始安装 $APP_NAME..."

if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR"
    echo "📁 创建了安装目录: $INSTALL_DIR"
fi

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

if [ "$ARCH" = "x86_64" ]; then
    ARCH_NAME="x86_64"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    ARCH_NAME="aarch64" 
else
    echo "❌ 不支持的架构: $ARCH"
    exit 1
fi

if [ "$OS" = "darwin" ]; then
    OS_NAME="apple-darwin"
elif [ "$OS" = "linux" ]; then
    OS_NAME="unknown-linux-gnu"
else
    echo "❌ 不支持的操作系统: $OS"
    exit 1
fi

ASSET_NAME="${APP_NAME}-${ARCH_NAME}-${OS_NAME}.tar.xz"

echo "🔍 正在获取 $REPO 的最新版本信息..."
LATEST_RELEASE_URL=$(curl -s https://api.github.com/repos/${REPO}/releases/latest | grep "browser_download_url" | grep "$ASSET_NAME" | grep -v "\.sha256" | cut -d '"' -f 4)

if [ -z "$LATEST_RELEASE_URL" ]; then
    echo "❌ 错误: 未找到下载链接。"
    exit 1
fi

TMP_DIR=$(mktemp -d)
echo "⬇️  正在下载: $LATEST_RELEASE_URL"
curl -L --progress-bar "$LATEST_RELEASE_URL" -o "$TMP_DIR/$ASSET_NAME"

echo "📦 正在解压..."
tar -xf "$TMP_DIR/$ASSET_NAME" -C "$TMP_DIR"

# ================= 修复核心逻辑 =================
# 在解压后的临时目录中，智能查找名为 ai-terminal-inspect 的文件
EXTRACTED_BIN=$(find "$TMP_DIR" -type f -name "$APP_NAME" | head -n 1)

if [ -z "$EXTRACTED_BIN" ]; then
    echo "❌ 错误: 解压成功，但在压缩包内未找到名为 $APP_NAME 的可执行文件。"
    echo "🔍 请检查 GitHub Release 压缩包内的文件结构。"
    rm -rf "$TMP_DIR"
    exit 1
fi
# ================================================

# 安装与配置软链接
mv "$EXTRACTED_BIN" "$INSTALL_DIR/$APP_NAME"
chmod +x "$INSTALL_DIR/$APP_NAME"
ln -sf "$INSTALL_DIR/$APP_NAME" "$INSTALL_DIR/$SHORT_NAME"

# 清理临时文件
rm -rf "$TMP_DIR"

echo "================================================="
echo "✅ $APP_NAME 安装成功！"
echo "📍 可执行文件路径: $INSTALL_DIR/$APP_NAME"
echo "⚡ 快捷命令已就绪:  $SHORT_NAME"
echo "================================================="