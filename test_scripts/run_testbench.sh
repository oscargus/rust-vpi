#!/bin/bash
# Script to build and run the Verilog testbench with VPI plugin

set -e

echo "=== Building VPI Plugin ==="
cargo build --release -p siminfo

echo ""
echo "=== Running Verilog Testbench with VPI ==="
echo "Using Icarus Verilog (iverilog/vvp)"
echo ""

# Compile Verilog testbench
iverilog -o testbench.vvp testbench.v

# Find the shared library
VPI_LIB=$(find target/release -name "libsiminfo.so" -o -name "libsiminfo.dylib" 2>/dev/null | head -1)

if [ -z "$VPI_LIB" ]; then
    echo "Error: Could not find VPI shared library"
    echo "Expected: target/release/libsiminfo.so or libsiminfo.dylib"
    exit 1
fi

echo "Found VPI library: $VPI_LIB"
echo ""

# Run simulation with VPI plugin loaded
vvp -M. -m"${VPI_LIB}" testbench.vvp

echo ""
echo "=== Simulation Complete ==="
