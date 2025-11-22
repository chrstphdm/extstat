#!/bin/bash
# test.sh - Create test files and run extstat

echo "📁 Creating test directory structure..."

# Create test directory
TEST_DIR="test_data"
rm -rf $TEST_DIR
mkdir -p $TEST_DIR

# Create various file types with different sizes
dd if=/dev/zero of=$TEST_DIR/large1.fastq bs=1M count=10 2>/dev/null
dd if=/dev/zero of=$TEST_DIR/large2.fastq bs=1M count=8 2>/dev/null
dd if=/dev/zero of=$TEST_DIR/large3.fastq bs=1M count=5 2>/dev/null

dd if=/dev/zero of=$TEST_DIR/genome1.fasta bs=1M count=3 2>/dev/null
dd if=/dev/zero of=$TEST_DIR/genome2.fasta bs=1M count=2 2>/dev/null

dd if=/dev/zero of=$TEST_DIR/data.bam bs=1M count=15 2>/dev/null

dd if=/dev/zero of=$TEST_DIR/script.py bs=1K count=50 2>/dev/null
dd if=/dev/zero of=$TEST_DIR/script.sh bs=1K count=20 2>/dev/null

dd if=/dev/zero of=$TEST_DIR/readme.txt bs=1K count=10 2>/dev/null
dd if=/dev/zero of=$TEST_DIR/notes.md bs=1K count=5 2>/dev/null

# Files without extension
dd if=/dev/zero of=$TEST_DIR/Makefile bs=1K count=2 2>/dev/null
dd if=/dev/zero of=$TEST_DIR/LICENSE bs=1K count=1 2>/dev/null

echo ""
echo "✅ Test files created in $TEST_DIR/"
echo ""
echo "📊 Running extstat..."
echo ""

# Build if not already built
if [ ! -f target/release/extstat ]; then
    echo "🔨 Building extstat first..."
    cargo build --release
    echo ""
fi

# Run extstat on test data
./target/release/extstat $TEST_DIR -c

echo ""
echo "🧹 Cleanup: rm -rf $TEST_DIR"
