# systf example

Example VPI plugin crate that registers and handles custom system task/function callbacks.

Registered symbols:

- `$rust_log_plus_one(arg)` as a system task
- `$rust_add_one(arg)` as a system function
- `$rust_reverse_bits(arg)` as a system function

Both callbacks read one integer argument through the safe `vpi` crate API.
The function returns `arg + 1` by writing an integer value to the current
system function call handle.

`$rust_reverse_bits(arg)` accepts a scalar or vector argument and returns a
same-width bit-vector with the bit order reversed. Four-state values are
preserved (`0`, `1`, `x`, `z`) so unknown/high-impedance bits survive the
reversal.

Example:

- input: `13'b1011001001110`
- output: `13'b0111001001101`

## What was missing to avoid low-level bindings

To avoid calling raw bindings directly in example code, the high-level `vpi`
API needed two helpers:

- `current_systf_call()` to obtain the active `vpiSysTfCall` handle
- `Handle::put_value(...)` (and `put_int_value`) to return function values from systf callbacks

These helpers are now available, so this example no longer needs direct
`vpi-sys` usage.

## Testbench coverage

The testbench at `test_examples/systf_tb.v` checks:

- `$rust_add_one` integer behavior
- `$rust_reverse_bits` on a 13-bit vector
- `$rust_reverse_bits` on a 1-bit scalar
- `$rust_reverse_bits` with mixed four-state bits (`x`/`z`)

## Build

From the workspace root:

```bash
cargo build -p systf
```

## Run with Icarus Verilog

From the workspace root:

- Unix: `./test_scripts/run_systf_test.sh`
- Windows: `./test_scripts/run_systf_test.ps1`
