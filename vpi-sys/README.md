# vpi-sys

[![crates.io](https://img.shields.io/crates/v/vpi-sys.svg)](https://crates.io/crates/vpi-sys)
[![docs.rs](https://docs.rs/vpi-sys/badge.svg)](https://docs.rs/vpi-sys)

Low-level FFI bindings to the Verilog/SystemVerilog VPI C API.

Bindings are generated with `bindgen` and map the C API directly.

## Notes

- This crate is intended as an internal foundation for the safe `vpi` crate.
- Most users should depend on `vpi` instead of using `vpi-sys` directly.

## Feature

- `sv`: Include SystemVerilog VPI extensions.
