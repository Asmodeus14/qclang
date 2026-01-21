#!/bin/bash
# QCLang Linux Installer
# One-line install: curl -fsSL https://raw.githubusercontent.com/Asmodeus14/qclang/master/dist/linux/install.sh | bash

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}"
echo "┌─────────────────────────────────────────────────────┐"
echo "│     QCLang Quantum Compiler Installer v0.2.0        │"
echo "│     Quantum systems programming language            │"
echo "└─────────────────────────────────────────────────────┘${NC}"
echo

# Default installation directory
INSTALL_DIR="/usr/local/bin"
BACKUP_DIR="$HOME/.qclang-backup"
VERSION="0.2.0"
REPO="Asmodeus14/qclang"

# Check for existing installation
if command -v qclang &> /dev/null; then
    echo -e "${YELLOW}⚠️  QCLang is already installed${NC}"
    CURRENT_VERSION=$(qclang --version 2>/dev/null | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+' || echo "unknown")
    echo -e "  Current version: ${CYAN}$CURRENT_VERSION${NC}"
    echo -e "  Installing version: ${CYAN}$VERSION${NC}"
    echo
    
    read -p "Do you want to continue? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Installation cancelled."
        exit 0
    fi
fi

# Detect architecture
ARCH=$(uname -m)
case $ARCH in
    x86_64)
        ARCH="x64"
        ;;
    aarch64|arm64)
        ARCH="arm64"
        ;;
    *)
        echo -e "${RED}❌ Unsupported architecture: $ARCH${NC}"
        echo "Please install from source: https://github.com/$REPO"
        exit 1
        ;;
esac

# Detect OS
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

echo -e "${CYAN}📦 System detected:${NC}"
echo -e "  OS: ${YELLOW}$OS${NC}"
echo -e "  Architecture: ${YELLOW}$ARCH${NC}"
echo -e "  Install directory: ${YELLOW}$INSTALL_DIR${NC}"
echo

# Backup existing installation
if [ -f "$INSTALL_DIR/qclang" ]; then
    echo -e "${YELLOW}📋 Backing up existing installation...${NC}"
    mkdir -p "$BACKUP_DIR"
    cp "$INSTALL_DIR/qclang" "$BACKUP_DIR/qclang-backup-$(date +%Y%m%d_%H%M%S)"
fi

# Download binary
echo -e "${CYAN}⬇️  Downloading QCLang binary...${NC}"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/v$VERSION/qclang-$OS-$ARCH"

# Try different download methods
if command -v wget &> /dev/null; then
    wget -q -O "/tmp/qclang-$OS-$ARCH" "$DOWNLOAD_URL"
elif command -v curl &> /dev/null; then
    curl -s -L -o "/tmp/qclang-$OS-$ARCH" "$DOWNLOAD_URL"
else
    echo -e "${RED}❌ Neither wget nor curl found. Please install one.${NC}"
    exit 1
fi

if [ ! -f "/tmp/qclang-$OS-$ARCH" ]; then
    echo -e "${YELLOW}⚠️  Binary not available for download, building from source...${NC}"
    
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Rust/cargo not found.${NC}"
        echo "Please install Rust from: https://rustup.rs/"
        exit 1
    fi
    
    # Clone and build
    echo "Building from source (this may take a few minutes)..."
    git clone https://github.com/$REPO.git /tmp/qclang-build
    cd /tmp/qclang-build/compiler
    cargo build --release
    cp target/release/qclang "/tmp/qclang-$OS-$ARCH"
    cd -
    rm -rf /tmp/qclang-build
fi

# Verify binary
if [ ! -f "/tmp/qclang-$OS-$ARCH" ]; then
    echo -e "${RED}❌ Failed to get QCLang binary${NC}"
    exit 1
fi

# Make executable
chmod +x "/tmp/qclang-$OS-$ARCH"

# Install
echo -e "${CYAN}🔧 Installing to $INSTALL_DIR...${NC}"
sudo cp "/tmp/qclang-$OS-$ARCH" "$INSTALL_DIR/qclang"
sudo chmod +x "$INSTALL_DIR/qclang"

# Cleanup
rm -f "/tmp/qclang-$OS-$ARCH"

# Verify installation
echo -e "${CYAN}✅ Verifying installation...${NC}"
if command -v qclang &> /dev/null; then
    INSTALLED_VERSION=$(qclang --version 2>/dev/null || echo "unknown")
    echo -e "${GREEN}🎉 QCLang installed successfully!${NC}"
    echo -e "  Version: ${CYAN}$INSTALLED_VERSION${NC}"
    echo -e "  Location: ${YELLOW}$(which qclang)${NC}"
else
    echo -e "${YELLOW}⚠️  Installation complete but qclang command not found in PATH${NC}"
    echo "You may need to add $INSTALL_DIR to your PATH or restart your terminal."
fi

# Create example
echo -e "${CYAN}📝 Creating example file...${NC}"
EXAMPLE_FILE="$HOME/qclang-example.qc"
cat > "$EXAMPLE_FILE" << 'EOF'
// hello_quantum.qc
fn main() -> int {
    qubit q = |0>;
    q = H(q);
    cbit result = measure(q);
    return 0;
}
EOF

echo -e "${GREEN}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "💎 QCLang Installation Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo
echo -e "${CYAN}🚀 Quick Start:${NC}"
echo -e "  1. Create a quantum circuit:"
echo -e "     ${YELLOW}nano circuit.qc${NC}"
echo -e "  2. Compile it:"
echo -e "     ${YELLOW}qclang circuit.qc${NC}"
echo -e "  3. Try the example:"
echo -e "     ${YELLOW}qclang $EXAMPLE_FILE${NC}"
echo
echo -e "${CYAN}📚 Documentation:${NC}"
echo -e "  GitHub: ${YELLOW}https://github.com/$REPO${NC}"
echo -e "  Examples: ${YELLOW}$HOME/qclang-example.qc${NC}"
echo
echo -e "${CYAN}🆘 Need help?${NC}"
echo -e "  Run: ${YELLOW}qclang --help${NC}"
echo -e "  Issues: ${YELLOW}https://github.com/$REPO/issues${NC}"
echo