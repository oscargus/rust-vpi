#!/bin/bash
# Script to build and run the put_value VPI example

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

echo "=== Building put_value VPI Plugin ==="
if [ "$(uname -s)" = "Darwin" ]; then
    RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=-Wl,-undefined,dynamic_lookup" cargo build "${CARGO_PROFILE_ARGS[@]}" -p put_value
else
    cargo build "${CARGO_PROFILE_ARGS[@]}" -p put_value
fi

echo ""
echo "=== Running put_value Verilog Testbench ==="
echo ""

iverilog -g2012 -o put_value_tb.vvp test_examples/put_value_dut.v

VPI_LIB=$(find "$TARGET_DIR" \( -name "libput_value.so" -o -name "libput_value.dylib" -o -name "put_value.dll" -o -name "libput_value.dll" -o -name "put_value.vpi" \) 2>/dev/null | head -1)

if [ -z "$VPI_LIB" ]; then
    echo "Error: Could not find VPI shared library"
    echo "Expected under $TARGET_DIR: libput_value.so, libput_value.dylib, put_value.dll, or put_value.vpi"
    exit 1
fi

echo "Found VPI library: $VPI_LIB"
echo ""

if [[ "$VPI_LIB" == *.dll ]]; then
    module_dir=$(dirname "$VPI_LIB")
    module_file=$(basename "$VPI_LIB")
    module_name="${module_file%.dll}"
    cp -f "$VPI_LIB" "$module_dir/$module_name.vpi"
    vvp -M"$module_dir" -m"$module_name" put_value_tb.vvp
elif [[ "$VPI_LIB" == *.vpi ]]; then
    module_dir=$(dirname "$VPI_LIB")
    module_file=$(basename "$VPI_LIB")
    module_name="${module_file%.vpi}"
    vvp -M"$module_dir" -m"$module_name" put_value_tb.vvp
else
    vvp -M. -m"${VPI_LIB}" put_value_tb.vvp
fi

echo ""
echo "=== put_value test complete ==="
