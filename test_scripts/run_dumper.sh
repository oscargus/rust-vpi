#!/bin/bash
# Script to build and run the hierarchical Verilog testbench with dumper VPI plugin

set -e

BUILD_PROFILE="${CARGO_BUILD_PROFILE:-release}"

case "$BUILD_PROFILE" in
    release)
        CARGO_PROFILE_ARGS=(--release)
        TARGET_DIR="target/release"
        ;;
    debug|dev)
        CARGO_PROFILE_ARGS=()
        TARGET_DIR="target/debug"
        ;;
    *)
        CARGO_PROFILE_ARGS=(--profile "$BUILD_PROFILE")
        TARGET_DIR="target/$BUILD_PROFILE"
        ;;
esac

echo "=== Building Dumper VPI Plugin ==="
if [ "$(uname -s)" = "Darwin" ]; then
    RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=-Wl,-undefined,dynamic_lookup" cargo build "${CARGO_PROFILE_ARGS[@]}" -p dumper
else
    cargo build "${CARGO_PROFILE_ARGS[@]}" -p dumper
fi

echo ""
echo "=== Compiling Hierarchical Verilog Testbench ==="
echo "Using Icarus Verilog (iverilog/vvp)"
echo ""

# Compile hierarchical Verilog design and testbench
iverilog -g2012 -o test_examples/hier_tb.vvp test_examples/hier_design.v test_examples/hier_tb.v

echo "Compilation successful"
echo ""

# Find the shared library
VPI_LIB=$(find "$TARGET_DIR" \( -name "libdumper.so" -o -name "libdumper.dylib" -o -name "dumper.dll" -o -name "libdumper.dll" -o -name "dumper.vpi" \) 2>/dev/null | head -1)

if [ -z "$VPI_LIB" ]; then
    echo "Error: Could not find VPI shared library"
    echo "Expected under $TARGET_DIR: libdumper.so, libdumper.dylib, dumper.dll, or dumper.vpi"
    exit 1
fi

echo "Found VPI library: $VPI_LIB"
echo ""
echo "=== Running Simulation with Dumper VPI Plugin ==="
echo ""

# Run simulation with VPI plugin loaded
if [[ "$VPI_LIB" == *.dll ]]; then
    module_dir=$(dirname "$VPI_LIB")
    module_file=$(basename "$VPI_LIB")
    module_name="${module_file%.dll}"
    cp -f "$VPI_LIB" "$module_dir/$module_name.vpi"
    vvp -M"$module_dir" -m"$module_name" test_examples/hier_tb.vvp
elif [[ "$VPI_LIB" == *.vpi ]]; then
    module_dir=$(dirname "$VPI_LIB")
    module_file=$(basename "$VPI_LIB")
    module_name="${module_file%.vpi}"
    vvp -M"$module_dir" -m"$module_name" test_examples/hier_tb.vvp
else
    vvp -M. -m"${VPI_LIB}" test_examples/hier_tb.vvp
fi

echo ""
echo "=== Simulation Complete ==="
