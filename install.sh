#!/bin/bash

# LeetCode CLI Installation Script

set -e

echo "🚀 LeetCode CLI Installer"
echo ""

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not installed"
    echo ""
    echo "Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    exit 1
fi

echo "✓ Rust installed: $(rustc --version)"
echo ""

# Check Cargo
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not installed"
    exit 1
fi

echo "✓ Cargo installed: $(cargo --version)"
echo ""

# Build project
echo "🔨 Building project..."
cargo build --release

# Installation path
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Create installation directory
mkdir -p "$INSTALL_DIR"

# Copy binary
echo "📦 Installing to $INSTALL_DIR..."
cp target/release/leetcode-cli "$INSTALL_DIR/"

# Check PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "⚠️  Warning: $INSTALL_DIR is not in PATH"
    echo ""
    echo "Please add the following line to your shell config (~/.bashrc, ~/.zshrc, etc.):"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
    echo ""
fi

echo ""
echo "✅ Installation complete!"
echo ""
echo "Usage:"
echo "  leetcode-cli --help          Show help information"
echo "  leetcode-cli login           Login to LeetCode"
echo "  leetcode-cli pick            Random problem selection"
echo "  leetcode-cli download -i 1   Download problem"
echo ""
echo "See README.md for detailed documentation"
echo ""
