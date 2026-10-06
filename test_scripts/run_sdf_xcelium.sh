#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
EXAMPLE_DIR="$ROOT_DIR/examples/sdf"
cd "$ROOT_DIR"

XRUN="${XRUN:-xrun}"
if ! command -v "$XRUN" >/dev/null 2>&1; then
    echo "error: Xcelium xrun command not found: $XRUN" >&2
    exit 1
fi

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
cargo build "${CARGO_PROFILE_ARGS[@]}" -p intermod_test

VPI_LIB="$TARGET_DIR/libintermod_test.so"
if [[ ! -f "$VPI_LIB" ]]; then
    echo "error: VPI shared library not found: $VPI_LIB" >&2
    exit 1
fi

WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/rust-vpi-sdf-xcelium.XXXXXX")"
trap 'rm -rf -- "$WORK_DIR"' EXIT
cp "$EXAMPLE_DIR/verilog/test.sdf" "$WORK_DIR/test.sdf"
cd "$WORK_DIR"

echo "=== Running SDF testbench with Xcelium ==="
LOG_FILE="$WORK_DIR/xrun.log"
"$XRUN" -64bit -sv -access +rwc -ANNO_SIMTIME \
    -loadvpi "$VPI_LIB:intermod_test_register" \
    -loadvpisim "$VPI_LIB:intermod_test_register" \
    "$EXAMPLE_DIR/verilog/top.v" 2>&1 | tee "$LOG_FILE"

if ! grep -Fq '== intermod_test: PASSED (0 errors) ==' "$LOG_FILE"; then
    echo "error: SDF VPI test did not report success" >&2
    exit 1
fi

echo "=== SDF VPI test passed ==="
