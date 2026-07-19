#!/bin/bash
# Script to build and run the systf VPI example

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

echo "=== Building systf VPI Plugin ==="
if [ "$(uname -s)" = "Darwin" ]; then
    RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=-Wl,-undefined,dynamic_lookup" cargo build "${CARGO_PROFILE_ARGS[@]}" -p systf
else
    cargo build "${CARGO_PROFILE_ARGS[@]}" -p systf
fi

echo ""
echo "=== Running systf Verilog Testbench ==="
echo ""

iverilog -o systf_tb.vvp test_examples/systf_tb.v

VPI_LIB=$(find "$TARGET_DIR" \( -name "libsystf.so" -o -name "libsystf.dylib" -o -name "systf.dll" -o -name "libsystf.dll" -o -name "systf.vpi" \) 2>/dev/null | head -1)

if [ -z "$VPI_LIB" ]; then
    echo "Error: Could not find VPI shared library"
    echo "Expected under $TARGET_DIR: libsystf.so, libsystf.dylib, systf.dll, or systf.vpi"
    exit 1
fi

echo "Found VPI library: $VPI_LIB"
echo ""

if [[ "$VPI_LIB" == *.dll ]]; then
    module_dir=$(dirname "$VPI_LIB")
    module_file=$(basename "$VPI_LIB")
    module_name="${module_file%.dll}"
    cp -f "$VPI_LIB" "$module_dir/$module_name.vpi"
    vvp -M"$module_dir" -m"$module_name" systf_tb.vvp
elif [[ "$VPI_LIB" == *.vpi ]]; then
    module_dir=$(dirname "$VPI_LIB")
    module_file=$(basename "$VPI_LIB")
    module_name="${module_file%.vpi}"
    vvp -M"$module_dir" -m"$module_name" systf_tb.vvp
else
    vvp -M. -m"${VPI_LIB}" systf_tb.vvp
fi

echo ""
echo "=== systf test complete ==="
