#!/usr/bin/env bash
#
# ModelSim/Questa SDF test script. However, it is not supported, but included for completeness.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXAMPLE_DIR="$ROOT_DIR/examples/sdf"
cd "$ROOT_DIR"

VLIB="${VLIB:-vlib}"
VLOG="${VLOG:-vlog}"
VSIM="${VSIM:-vsim}"

for tool in "$VLIB" "$VLOG" "$VSIM"; do
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "error: required ModelSim/Questa command not found: $tool" >&2
        exit 1
    fi
done

BUILD_PROFILE="${CARGO_BUILD_PROFILE:-release}"
case "$BUILD_PROFILE" in
    release)
        CARGO_PROFILE_ARGS=(--release)
        TARGET_DIR="$ROOT_DIR/target/release"
        ;;
    debug|dev)
        CARGO_PROFILE_ARGS=()
        TARGET_DIR="$ROOT_DIR/target/debug"
        ;;
    *)
        CARGO_PROFILE_ARGS=(--profile "$BUILD_PROFILE")
        TARGET_DIR="$ROOT_DIR/target/$BUILD_PROFILE"
        ;;
esac

echo "=== Building SDF VPI Plugin ==="
if [[ "$(uname -s)" == "Darwin" ]]; then
    RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }-C link-arg=-Wl,-undefined,dynamic_lookup" \
        cargo build "${CARGO_PROFILE_ARGS[@]}" -p intermod_test
else
    cargo build "${CARGO_PROFILE_ARGS[@]}" -p intermod_test
fi

VPI_LIB=""
for candidate in \
    "$TARGET_DIR/libintermod_test.so" \
    "$TARGET_DIR/libintermod_test.dylib" \
    "$TARGET_DIR/intermod_test.dll" \
    "$TARGET_DIR/libintermod_test.dll"
do
    if [[ -f "$candidate" ]]; then
        VPI_LIB="$candidate"
        break
    fi
done

if [[ -z "$VPI_LIB" ]]; then
    echo "error: could not find the intermod_test VPI library under $TARGET_DIR" >&2
    exit 1
fi

WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/rust-vpi-sdf.XXXXXX")"
trap 'rm -rf -- "$WORK_DIR"' EXIT
cp "$EXAMPLE_DIR/verilog/test.sdf" "$WORK_DIR/test.sdf"
cd "$WORK_DIR"

echo "=== Compiling SDF testbench ==="
"$VLIB" work
"$VLOG" -work work "$EXAMPLE_DIR/verilog/top.v"

echo "=== Running SDF testbench with ModelSim/Questa ==="
LOG_FILE="$WORK_DIR/vsim.log"
"$VSIM" -voptargs=+acc -c -pli "$VPI_LIB" work.top \
    -do 'onerror {quit -code 1}; run -all; quit -f' 2>&1 | tee "$LOG_FILE"

if ! grep -Fq '== intermod_test: PASSED (0 errors) ==' "$LOG_FILE"; then
    echo "error: SDF VPI test did not report success" >&2
    exit 1
fi

echo "=== SDF VPI test passed ==="
