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
vpi = "0.4.0"
```

See top-level project examples for complete plugin implementations.

## Coverage and Current Gaps

The safe `vpi` crate covers the common plugin workflows in `vpi_user.h`:

- handle lookup and traversal
- property queries
- value get/put
- delay access
- callback registration/removal
- systf registration and argument access
- simulator control/time helpers
- basic simulator and MCD output helpers

Some lower-level `vpi_user.h` entry points are still only available through
`vpi-sys`, notably:

- `vpi_get_systf_info`
- `vpi_put_value_array`
- save/restart state APIs: `vpi_get_data`, `vpi_put_data`
- per-handle user data APIs: `vpi_get_userdata`, `vpi_put_userdata`
- varargs print APIs: `vpi_vprintf`, `vpi_mcd_vprintf`
- explicit object lifetime API: `vpi_free_object`

SystemVerilog support from `sv_vpi_user.h` is also partial at the high-level
API layer. The crate exposes many SV object and property constants through the
generic `ObjectType` and `Property` enums, and it supports assertion
callbacks, but it does not yet provide dedicated high-level helpers for several
SV-specific modeling areas such as:

- packages, interfaces, programs, and virtual interfaces
- class/type-spec trees and type parameters
- clocking blocks and clocking I/O
- constraint/randomization structures
- property/sequence AST-style traversal helpers

If you need one of these areas today, use `vpi-sys` directly for that portion
and keep the rest of the plugin on the safe `vpi` API.
