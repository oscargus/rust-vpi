#!/bin/bash
# Script to test timescale VPI functionality

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

echo "=== Building Timescale VPI Plugin ==="
if [ "$(uname -s)" = "Darwin" ]; then
    RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=-Wl,-undefined,dynamic_lookup" cargo build "${CARGO_PROFILE_ARGS[@]}" -p timescale
else
    cargo build "${CARGO_PROFILE_ARGS[@]}" -p timescale
fi

echo ""
echo "=== Running Verilog Testbench with Timescale VPI ==="
echo ""

# Compile Verilog testbench if not already compiled
if [ ! -f testbench.vvp ]; then
    iverilog -o testbench.vvp test_examples/testbench.v
fi

# Find the shared library
VPI_LIB=$(find "$TARGET_DIR" \( -name "libtimescale.so" -o -name "libtimescale.dylib" -o -name "timescale.dll" -o -name "libtimescale.dll" -o -name "timescale.vpi" \) 2>/dev/null | head -1)

if [ -z "$VPI_LIB" ]; then
    echo "Error: Could not find VPI shared library"
    echo "Expected under $TARGET_DIR: libtimescale.so, libtimescale.dylib, timescale.dll, or timescale.vpi"
    exit 1
fi

echo "Found VPI library: $VPI_LIB"
echo ""

# Run simulation with VPI plugin loaded
if [[ "$VPI_LIB" == *.dll ]]; then
    module_dir=$(dirname "$VPI_LIB")
    module_file=$(basename "$VPI_LIB")
    module_name="${module_file%.dll}"
    cp -f "$VPI_LIB" "$module_dir/$module_name.vpi"
    vvp -M"$module_dir" -m"$module_name" testbench.vvp
elif [[ "$VPI_LIB" == *.vpi ]]; then
    module_dir=$(dirname "$VPI_LIB")
    module_file=$(basename "$VPI_LIB")
    module_name="${module_file%.vpi}"
    vvp -M"$module_dir" -m"$module_name" testbench.vvp
else
    vvp -M. -m"${VPI_LIB}" testbench.vvp
fi

echo ""
echo "=== Test Complete ==="
