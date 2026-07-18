# vpi

[![crates.io](https://img.shields.io/crates/v/vpi.svg)](https://crates.io/crates/vpi)
[![docs.rs](https://docs.rs/vpi/badge.svg)](https://docs.rs/vpi)

Safe and ergonomic Rust bindings for writing Verilog/SystemVerilog VPI plugins.

This crate wraps `vpi-sys` with a safer API surface.

## Features

- `cb_info`: Enabled by default. Uses `vpi_get_cb_info` when removing callbacks.
- `sv`: Enable SystemVerilog VPI extensions.
- `bigint`: Enable conversions with `num-bigint`.
- `dynamic`: On Windows/macOS, use runtime symbol lookup via `vpi-shim` so plugins can be built without directly linking simulator libraries.

## Usage

Add this crate as a dependency in your plugin crate and build as `cdylib`.

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
vpi = { path = "../vpi" }
```

See top-level project examples for complete plugin implementations.
