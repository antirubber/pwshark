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

# Verify C toolchain — cargo's linker step needs a working cc/as/ld
echo "Verifying C toolchain..."
cc_src=$(mktemp --suffix=.c)
cc_out="${cc_src%.c}"
cc_log=$(mktemp)
echo 'int main(void){return 0;}' > "$cc_src"
if ! cc "$cc_src" -o "$cc_out" 2>"$cc_log"; then
    echo ""
    echo "ERROR: C toolchain is not usable — cargo build would fail at the linker step."
    echo ""
    sed 's/^/  /' "$cc_log"
    echo ""
    echo "Toolchain binaries on this system:"
    for tool in cc gcc as ld; do
        bin=$(command -v "$tool" 2>/dev/null) || { printf "  %-4s not found\n" "$tool"; continue; }
        real=$(readlink -f "$bin")
        perms=$(stat -c '%A %U' "$real" 2>/dev/null || echo "?")
        printf "  %-4s %s  [%s]\n" "$tool" "$real" "$perms"
    done
    echo ""
    echo "Likely fix:"
    echo "  sudo apt install --reinstall gcc g++ binutils      # Debian/Ubuntu"
    echo "  sudo dnf reinstall gcc gcc-c++ binutils            # Fedora/RHEL"
    echo "  # or chmod 755 any binary above that isn't world-executable"
    rm -f "$cc_src" "$cc_out" "$cc_log"
    exit 1
fi
rm -f "$cc_src" "$cc_out" "$cc_log"

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

# Install — atomic replace so a running pwshark can update itself (cp in place
# would hit ETXTBSY "Text file busy" on the live binary; rename sidesteps it)
mkdir -p "$INSTALL_DIR"
tmp="$INSTALL_DIR/.pwshark.new"
cp target/release/pwshark "$tmp"
chmod +x "$tmp"
mv -f "$tmp" "$INSTALL_DIR/pwshark"

echo ""
echo "Installed pwshark to $INSTALL_DIR/pwshark"
echo "Make sure $INSTALL_DIR is in your PATH."
echo ""
echo "Run: pwshark"
