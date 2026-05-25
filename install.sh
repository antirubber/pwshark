#!/usr/bin/env bash
set -euo pipefail

REPO="https://github.com/antirubber/pwshark.git"
INSTALL_DIR="${HOME}/.local/bin"
TMPDIR=$(mktemp -d)

cleanup() { rm -rf "$TMPDIR"; }
trap cleanup EXIT

echo "pwshark installer"

# Check/install Rust
if ! command -v cargo &>/dev/null; then
    echo "Rust not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "${HOME}/.cargo/env"
fi

# Install clipboard deps on Debian/Ubuntu
if command -v apt &>/dev/null; then
    echo "Installing clipboard dependencies..."
    sudo apt install -y libxcb1-dev libx11-dev libxkbcommon-dev 2>/dev/null || \
        echo "Could not install deps automatically. Build may fail — install libxcb1-dev libx11-dev libxkbcommon-dev manually."
fi

# Clone and build
echo "Cloning pwshark..."
git clone "$REPO" "$TMPDIR/pwshark"
cd "$TMPDIR/pwshark"

echo "Building release binary..."
cargo build --release

# Install
mkdir -p "$INSTALL_DIR"
cp target/release/pwshark "$INSTALL_DIR/pwshark"
chmod +x "$INSTALL_DIR/pwshark"

echo ""
echo "Installed pwshark to $INSTALL_DIR/pwshark"
echo "Make sure $INSTALL_DIR is in your PATH."
echo ""
echo "Run: pwshark"
