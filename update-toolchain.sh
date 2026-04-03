#!/bin/bash
# Update all rust-toolchain files in examples and examples_negative directories

VERSION="${1:-.}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ "$VERSION" = "." ] || [ -z "$VERSION" ]; then
    echo "Usage: $0 <rust-version>"
    echo ""
    echo "Examples:"
    echo "  $0 stable           # Update to stable"
    echo "  $0 1.83.0           # Update to 1.83.0"
    echo "  $0 nightly          # Update to nightly"
    echo "  $0 nightly-2022-07-05"
    exit 1
fi

echo "Updating all rust-toolchain files to: $VERSION"
echo ""

# Count files
TOTAL=$(find "$SCRIPT_DIR/examples" "$SCRIPT_DIR/examples_negative" -name "rust-toolchain" -type f 2>/dev/null | wc -l)
if [ "$TOTAL" -eq 0 ]; then
    echo "❌ No rust-toolchain files found"
    exit 1
fi

echo "Found $TOTAL files to update"

# Update all files
find "$SCRIPT_DIR/examples" "$SCRIPT_DIR/examples_negative" -name "rust-toolchain" -type f -exec sh -c 'echo "'"$VERSION"'" > "$1"' _ {} \;

# Verify
VERIFIED=$(find "$SCRIPT_DIR/examples" "$SCRIPT_DIR/examples_negative" -name "rust-toolchain" -type f -exec cat {} \; | sort | uniq | wc -l)

if [ "$VERIFIED" -eq 1 ]; then
    echo "✅ Successfully updated all $TOTAL files to $VERSION"
    
    # Show sample files
    echo ""
    echo "Sample files:"
    find "$SCRIPT_DIR/examples" "$SCRIPT_DIR/examples_negative" -name "rust-toolchain" -type f | head -3 | while read f; do
        echo "  $f: $(cat "$f")"
    done
    
    # Download the specified version
    echo ""
    echo "📥 Downloading Rust $VERSION..."
    if rustup install "$VERSION"; then
        echo "✅ Rust $VERSION downloaded successfully"
    else
        echo "⚠️  Failed to download Rust $VERSION"
        echo "   Try running manually: rustup install $VERSION"
        exit 1
    fi
else
    echo "❌ Update failed - found multiple versions:"
    find "$SCRIPT_DIR/examples" "$SCRIPT_DIR/examples_negative" -name "rust-toolchain" -type f -exec cat {} \; | sort | uniq -c
    exit 1
fi
