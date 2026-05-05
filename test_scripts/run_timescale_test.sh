#!/bin/bash
# Script to test timescale VPI functionality

set -e

echo "=== Building Timescale VPI Plugin ==="
if [ "$(uname -s)" = "Darwin" ]; then
    RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=-Wl,-undefined,dynamic_lookup" cargo build --release -p timescale
else
    cargo build --release -p timescale
fi

echo ""
echo "=== Running Verilog Testbench with Timescale VPI ==="
echo ""

# Compile Verilog testbench if not already compiled
if [ ! -f testbench.vvp ]; then
    iverilog -o testbench.vvp test_examples/testbench.v
fi

# Find the shared library
VPI_LIB=$(find target/release -name "libtimescale.so" -o -name "libtimescale.dylib" 2>/dev/null | head -1)

if [ -z "$VPI_LIB" ]; then
    echo "Error: Could not find VPI shared library"
    echo "Expected: target/release/libtimescale.so or libtimescale.dylib"
    exit 1
fi

echo "Found VPI library: $VPI_LIB"
echo ""

# Run simulation with VPI plugin loaded
vvp -M. -m"${VPI_LIB}" testbench.vvp

echo ""
echo "=== Test Complete ==="
