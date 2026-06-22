#!/usr/bin/env bash

set -e

# 获取当前脚本所在目录（等价于 pwd）
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

echo "📂 script dir: $SCRIPT_DIR"

# 项目根目录（假设脚本就在项目根）
PROJECT_DIR="$SCRIPT_DIR"

# 构建
cargo build --release

# 目标二进制
BIN="$PROJECT_DIR/target/release/mkmake"

if [ ! -f "$BIN" ]; then
    echo "❌ binary not found: $BIN"
    exit 1
fi

# 安装目录
INSTALL_DIR="$HOME/Library/MyOwn/bin"
mkdir -p "$INSTALL_DIR"

# 安装
cp "$BIN" "$INSTALL_DIR/mkmake"

echo "✅ installed mkmake to $INSTALL_DIR"
echo "⚠️  make sure $INSTALL_DIR is in your PATH"
