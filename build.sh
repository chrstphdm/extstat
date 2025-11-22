#!/bin/bash
# build.sh - Build and install extstat

set -e

echo "🔨 Building extstat..."
cargo build --release

echo ""
echo "✅ Build successful!"
echo ""
echo "Binary location: target/release/extstat"
echo ""
echo "To install system-wide:"
echo "  sudo cp target/release/extstat /usr/local/bin/"
echo ""
echo "Or run directly:"
echo "  ./target/release/extstat"
echo ""
echo "Test it now:"
echo "  ./target/release/extstat . -c -n 10"
