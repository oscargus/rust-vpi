#!/bin/bash
# Script to build and run the hierarchical Verilog testbench with dumper VPI plugin

set -e

echo "=== Building Dumper VPI Plugin ==="
cargo build --release -p dumper

echo ""
echo "=== Compiling Hierarchical Verilog Testbench ==="
echo "Using Icarus Verilog (iverilog/vvp)"
echo ""

# Compile hierarchical Verilog design and testbench
iverilog -g2012 -o test_examples/hier_tb.vvp test_examples/hier_design.v test_examples/hier_tb.v

echo "Compilation successful"
echo ""

# Find the shared library
VPI_LIB=$(find target/release -name "libdumper.so" -o -name "libdumper.dylib" 2>/dev/null | head -1)

if [ -z "$VPI_LIB" ]; then
    echo "Error: Could not find VPI shared library"
    echo "Expected: target/release/libdumper.so or libdumper.dylib"
    exit 1
fi

echo "Found VPI library: $VPI_LIB"
echo ""
echo "=== Running Simulation with Dumper VPI Plugin ==="
echo ""

# Run simulation with VPI plugin loaded
vvp -M. -m"${VPI_LIB}" test_examples/hier_tb.vvp

echo ""
echo "=== Simulation Complete ==="
