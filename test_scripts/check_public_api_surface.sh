#!/usr/bin/env bash
set -euo pipefail

# Public API surface guard:
# fail if any public function argument list uses a vpi_sys type directly.
# Usage: bash test_scripts/check_public_api_surface.sh [root]

ROOT_DIR="${1:-vpi/src}"

if [[ ! -d "$ROOT_DIR" ]]; then
  echo "error: directory not found: $ROOT_DIR" >&2
  exit 2
fi

status=0

while IFS= read -r -d '' file; do
  awk -v file="$file" '
    function trim(s) {
      gsub(/^[ \t\r\n]+|[ \t\r\n]+$/, "", s)
      return s
    }

    function extract_args(sig,     i,ch,depth,started,args) {
      started = 0
      depth = 0
      args = ""
      for (i = 1; i <= length(sig); i++) {
        ch = substr(sig, i, 1)
        if (!started) {
          if (ch == "(") {
            started = 1
            depth = 1
          }
          continue
        }

        if (ch == "(") {
          depth++
          args = args ch
          continue
        }

        if (ch == ")") {
          depth--
          if (depth == 0) {
            return args
          }
          args = args ch
          continue
        }

        args = args ch
      }
      return ""
    }

    BEGIN {
      in_sig = 0
      sig = ""
      fn_name = ""
      start_line = 0
    }

    {
      line = $0

      if (!in_sig) {
        # Start capturing at pub fn / pub unsafe fn.
        if (line ~ /^[ \t]*pub[ \t]+(unsafe[ \t]+)?fn[ \t]+[A-Za-z_][A-Za-z0-9_]*/) {
          in_sig = 1
          sig = line
          start_line = NR

          if (match(line, /fn[ \t]+([A-Za-z_][A-Za-z0-9_]*)/, m)) {
            fn_name = m[1]
          } else {
            fn_name = "<unknown>"
          }

          # Fall through to completion check below.
        }
      } else {
        sig = sig " " line
      }

      if (in_sig) {
        # End capture at opening body or declaration terminator.
        if (line ~ /\{/ || line ~ /;[ \t]*$/) {
          args = extract_args(sig)
          if (args ~ /vpi_sys::/) {
            print file ":" start_line ": public function " fn_name " has vpi_sys type in arguments"
            status = 1
          }

          in_sig = 0
          sig = ""
          fn_name = ""
          start_line = 0
        }
      }
    }

    END {
      if (status != 0) {
        exit 1
      }
    }
  ' "$file" || status=1
done < <(find "$ROOT_DIR" -type f -name '*.rs' -print0)

if [[ $status -ne 0 ]]; then
  echo "Found public functions with direct vpi_sys argument types." >&2
  exit 1
fi

echo "OK: no public function arguments use direct vpi_sys types."
