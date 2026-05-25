#!/usr/bin/env bash
set -euo pipefail

REPO="https://github.com/antirubber/pwshark.git"
INSTALL_DIR="${HOME}/.local/bin"
SRC_DIR="${HOME}/.local/share/pwshark"

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

# Clone or update
if [ -d "$SRC_DIR/.git" ]; then
    echo "Updating pwshark..."
    cd "$SRC_DIR"
    git pull --ff-only
else
    echo "Cloning pwshark..."
    rm -rf "$SRC_DIR"
    git clone "$REPO" "$SRC_DIR"
    cd "$SRC_DIR"
fi

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
