# put_value example

Example VPI plugin crate that demonstrates setting and getting DUT values from Rust only.

The top-level DUT contains three input kinds:

- single-bit input
- bit-vector input
- integer input

Rust accesses the DUT directly (no wrapper testbench), performs all writes with
`Handle::put_value`, reads back values with `Handle::get_value`, validates DUT
outputs, and ends simulation.

The example is organized as a modular test suite with multiple test cases.
Each case provides input vectors and timing configuration, then the runner:

1. applies values from Rust,
2. waits a configurable delay,
3. verifies DUT inputs and outputs from Rust,
4. waits an inter-test delay before the next case.

## Build

From the workspace root:

```bash
cargo build -p put_value
```

## Run with Icarus Verilog

From the workspace root:

- Unix: `./test_scripts/run_put_value_test.sh`
- Windows: `./test_scripts/run_put_value_test.ps1`
