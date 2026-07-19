#!/bin/bash
# Script to build and run the Verilog testbench with VPI plugin

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

echo "=== Building VPI Plugin ==="
if [ "$(uname -s)" = "Darwin" ]; then
    RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=-Wl,-undefined,dynamic_lookup" cargo build "${CARGO_PROFILE_ARGS[@]}" -p siminfo
else
    cargo build "${CARGO_PROFILE_ARGS[@]}" -p siminfo
fi

echo ""
echo "=== Running Verilog Testbench with VPI ==="
echo "Using Icarus Verilog (iverilog/vvp)"
echo ""

# Compile Verilog testbench
iverilog -o testbench.vvp test_examples/testbench.v

# Find the shared library
VPI_LIB=$(find "$TARGET_DIR" \( -name "libsiminfo.so" -o -name "libsiminfo.dylib" -o -name "siminfo.dll" -o -name "libsiminfo.dll" -o -name "siminfo.vpi" \) 2>/dev/null | head -1)

if [ -z "$VPI_LIB" ]; then
    echo "Error: Could not find VPI shared library"
    echo "Expected under $TARGET_DIR: libsiminfo.so, libsiminfo.dylib, siminfo.dll, or siminfo.vpi"
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
echo "=== Simulation Complete ==="
