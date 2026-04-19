#!/bin/bash

set -e

# 配置
REPO="kibuniverse/ai-terminal-inspect"
TOOL_NAME="ati"

# 检测平台
detect_platform() {
    local _ostype _cputype
    _ostype="$(uname -s)"
    _cputype="$(uname -m)"

    case "$_ostype" in
        Linux) _ostype="unknown-linux-gnu" ;;
        Darwin) _ostype="apple-darwin" ;;
        *) echo "错误: 不支持的操作系统: $_ostype"; exit 1 ;;
    esac

    case "$_cputype" in
        x86_64) ;;
        aarch64|arm64) _cputype="aarch64" ;;
        *) echo "错误: 不支持的架构: $_cputype"; exit 1 ;;
    esac

    echo "${_cputype}-${_ostype}"
}

# 获取最新版本
get_latest_version() {
    curl -s "https://api.github.com/repos/$REPO/releases/latest" | 
    grep '"tag_name":' | 
    sed -E 's/.*"([^"]+)".*/\1/'
}

# 安装
main() {
    echo "🔧 正在安装 $TOOL_NAME..."
    
    PLATFORM=$(detect_platform)
    VERSION=$(get_latest_version)
    
    echo "检测到平台: $PLATFORM"
    echo "最新版本: $VERSION"
    
    # 下载地址
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/${TOOL_NAME}-${PLATFORM}.tar.gz"
    
    # 临时目录
    TMP_DIR=$(mktemp -d)
    cd "$TMP_DIR"
    
    echo "📥 正在下载..."
    curl -L -o "${TOOL_NAME}.tar.gz" "$DOWNLOAD_URL"
    
    echo "📦 正在解压..."
    tar xzf "${TOOL_NAME}.tar.gz"
    
    # 安装路径
    INSTALL_DIR="$HOME/.local/bin"
    if [ ! -d "$INSTALL_DIR" ]; then
        mkdir -p "$INSTALL_DIR"
    fi
    
    echo "⚙️  正在安装到 $INSTALL_DIR..."
    mv "$TOOL_NAME" "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/$TOOL_NAME"
    
    # 清理
    cd ..
    rm -rf "$TMP_DIR"
    
    # 检查 PATH
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo ""
        echo "⚠️  警告: $INSTALL_DIR 不在 PATH 中"
        echo "请添加以下行到你的 ~/.bashrc 或 ~/.zshrc："
        echo "    export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi
    
    echo ""
    echo "✅ 安装成功！"
    echo "运行 '$TOOL_NAME --help' 开始使用"
}

main