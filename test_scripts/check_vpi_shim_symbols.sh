#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

if ! command -v rg >/dev/null 2>&1; then
  echo "error: ripgrep (rg) is required for this check"
  exit 1
fi

used_symbols="$(rg -No --no-line-number --no-filename 'vpi_sys::(vpi_[A-Za-z0-9_]+)' vpi/src -r '$1' | sort -u)"
forwarded_symbols="$(rg -No --no-line-number --no-filename 'fn (vpi_[A-Za-z0-9_]+)\(' vpi-shim/src/lib.rs -r '$1' | sort -u)"

missing_symbols="$(comm -23 \
  <(printf '%s\n' "$used_symbols") \
  <(printf '%s\n' "$forwarded_symbols") \
  | sed '/^$/d')"

if [[ -n "$missing_symbols" ]]; then
  echo "error: vpi-shim is missing forwarded symbols used by vpi crate:"
  printf '  - %s\n' $missing_symbols
  exit 1
fi

echo "vpi-shim symbol coverage check passed."
