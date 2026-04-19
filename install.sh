#!/bin/bash

# 发生错误时退出
set -e

REPO="kibuniverse/ai-terminal-inspect"
APP_NAME="ai-terminal-inspect"
SHORT_NAME="ati"
INSTALL_DIR="$HOME/.local/bin"

echo "🚀 开始安装 $APP_NAME..."

# 1. 检查并创建本地安装目录
if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR"
    echo "📁 创建了安装目录: $INSTALL_DIR"
fi

# 2. 检测操作系统和架构
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

# 匹配常见的架构名称
if [ "$ARCH" = "x86_64" ]; then
    ARCH_NAME="x86_64"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    ARCH_NAME="aarch64" # 注意：如果你的 M1/M2 Mac Release 包名为 arm64，请将此处改为 arm64
else
    echo "❌ 不支持的架构: $ARCH"
    exit 1
fi

# 匹配常见的系统名称 (这里使用了常见的 target triple 命名习惯)
if [ "$OS" = "darwin" ]; then
    OS_NAME="apple-darwin"
elif [ "$OS" = "linux" ]; then
    OS_NAME="unknown-linux-gnu"
else
    echo "❌ 不支持的操作系统: $OS"
    exit 1
fi

# ⚠️ 注意：这里假设了你的 GitHub Release 压缩包命名规范。
# 例如：ai-terminal-inspect-x86_64-apple-darwin.tar.gz
# 请确保此处与你实际打包（如 GitHub Actions 或 cargo-dist）的产物名称一致！
ASSET_NAME="${APP_NAME}-${ARCH_NAME}-${OS_NAME}.tar.xz"

# 3. 通过 GitHub API 获取最新版本的下载链接
echo "🔍 正在获取 $REPO 的最新版本信息..."
LATEST_RELEASE_URL=$(curl -s https://api.github.com/repos/${REPO}/releases/latest | grep "browser_download_url" | grep "$ASSET_NAME" | cut -d '"' -f 4)

if [ -z "$LATEST_RELEASE_URL" ]; then
    echo "❌ 错误: 未在最新 Release 中找到匹配 $ASSET_NAME 的下载链接。"
    echo "👉 请检查 GitHub Releases 中资产的命名格式是否与脚本中的 ASSET_NAME 匹配。"
    exit 1
fi

# 4. 下载并解压
TMP_DIR=$(mktemp -d)
echo "⬇️  正在下载: $LATEST_RELEASE_URL"
curl -L --progress-bar "$LATEST_RELEASE_URL" -o "$TMP_DIR/$ASSET_NAME"

echo "📦 正在解压..."
# 如果你的 Release 是直接的二进制文件而不是压缩包，请删除 tar 解压步骤，并直接移动文件
tar -xzf "$TMP_DIR/$ASSET_NAME" -C "$TMP_DIR"

# 5. 安装与配置软链接
# 假设解压出来的可执行文件名为 ai-terminal-inspect
mv "$TMP_DIR/$APP_NAME" "$INSTALL_DIR/$APP_NAME"
chmod +x "$INSTALL_DIR/$APP_NAME"

# 创建简写命令 'ati' 的软链接
ln -sf "$INSTALL_DIR/$APP_NAME" "$INSTALL_DIR/$SHORT_NAME"

# 清理临时文件
rm -rf "$TMP_DIR"

echo "================================================="
echo "✅ $APP_NAME 安装成功！"
echo "📍 可执行文件路径: $INSTALL_DIR/$APP_NAME"
echo "⚡ 快捷命令已就绪:  $SHORT_NAME"
echo "================================================="
echo ""
echo "💡 提示: 如果你在终端输入 '$SHORT_NAME' 提示 command not found，"
echo "请确保 $INSTALL_DIR 已加入你的 PATH 环境变量中。"
echo "你可以通过执行以下命令将其添加到你的配置文件中 (如 ~/.zshrc 或 ~/.bashrc):"
echo ""
echo '    echo '\''export PATH="$HOME/.local/bin:$PATH"'\'' >> ~/.zshrc'
echo '    source ~/.zshrc'
echo "================================================="