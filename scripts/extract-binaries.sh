#!/bin/bash
# Extract the compiled Linux binary from system
echo "📦 Extracting QCLang Linux binary..."

# Create dist directory structure
mkdir -p dist/linux
mkdir -p dist/windows
mkdir -p dist/common/examples

# Copy the binary from /usr/local/bin
if [ -f "/usr/local/bin/qclang" ]; then
    echo "✅ Found binary at /usr/local/bin/qclang"
    cp /usr/local/bin/qclang dist/linux/qclang-linux
    chmod +x dist/linux/qclang-linux
    echo "✅ Binary copied to dist/linux/qclang-linux"
else
    echo "❌ Binary not found at /usr/local/bin/qclang"
    echo "Building from source..."
    cd compiler
    cargo build --release
    cp target/release/qclang ../dist/linux/qclang-linux
    chmod +x ../dist/linux/qclang-linux
    cd ..
fi

# Copy example files
cp libs/examples/*.qc dist/common/examples/

echo "📊 Binary info:"
file dist/linux/qclang-linux
ls -lh dist/linux/qclang-linux

echo "✅ Extraction complete!"