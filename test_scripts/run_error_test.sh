#!/bin/bash
# Script to build and run the error_test VPI example

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

echo "=== Building error_test VPI Plugin ==="
if [ "$(uname -s)" = "Darwin" ]; then
    RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=-Wl,-undefined,dynamic_lookup" cargo build "${CARGO_PROFILE_ARGS[@]}" -p error_test
else
    cargo build "${CARGO_PROFILE_ARGS[@]}" -p error_test
fi

echo ""
echo "=== Compiling Verilog Testbench ==="
iverilog -g2012 -o test_examples/error_test_tb.vvp test_examples/error_test_tb.v test_examples/error_test_dut.v

echo ""
echo "=== Finding VPI Library ==="
VPI_LIB=$(find "$TARGET_DIR" \( -name "liberror_test.so" -o -name "liberror_test.dylib" -o -name "error_test.dll" -o -name "liberror_test.dll" -o -name "error_test.vpi" \) 2>/dev/null | head -1)

if [ -z "$VPI_LIB" ]; then
    echo "Error: Could not find VPI shared library"
    echo "Expected under $TARGET_DIR: liberror_test.so, liberror_test.dylib, error_test.dll, or error_test.vpi"
    exit 1
fi

echo "Found VPI library: $VPI_LIB"
echo ""

echo "=== Running error_test Testbench ==="
if [[ "$VPI_LIB" == *.dll ]]; then
    module_dir=$(dirname "$VPI_LIB")
    module_file=$(basename "$VPI_LIB")
    module_name="${module_file%.dll}"
    cp -f "$VPI_LIB" "$module_dir/$module_name.vpi"
    vvp -M"$module_dir" -m"$module_name" test_examples/error_test_tb.vvp
elif [[ "$VPI_LIB" == *.vpi ]]; then
    module_dir=$(dirname "$VPI_LIB")
    module_file=$(basename "$VPI_LIB")
    module_name="${module_file%.vpi}"
    vvp -M"$module_dir" -m"$module_name" test_examples/error_test_tb.vvp
else
    if [ "$(uname -s)" = "Darwin" ]; then
        DYLD_LIBRARY_PATH="$(dirname "$VPI_LIB"):$(dirname "$VPI_LIB")/deps:${DYLD_LIBRARY_PATH}" \
        vvp -M. -m"${VPI_LIB}" test_examples/error_test_tb.vvp
    else
        LD_LIBRARY_PATH="$(dirname "$VPI_LIB"):$(dirname "$VPI_LIB")/deps:${LD_LIBRARY_PATH}" \
        vvp -M. -m"${VPI_LIB}" test_examples/error_test_tb.vvp
    fi
fi

echo ""
echo "=== error_test Test Complete ==="
