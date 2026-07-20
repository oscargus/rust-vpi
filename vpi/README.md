# vpi

[![crates.io](https://img.shields.io/crates/v/vpi.svg)](https://crates.io/crates/vpi)
[![docs.rs](https://docs.rs/vpi/badge.svg)](https://docs.rs/vpi)

Safe and ergonomic Rust bindings for writing Verilog/SystemVerilog VPI plugins.

This crate wraps `vpi-sys` with a safer API surface.

## Features

- `bigint`: Enable conversions with `num-bigint`.
- `cb_info`: Enabled by default. Uses `vpi_get_cb_info` when removing callbacks.
- `dynamic`: On Windows/macOS, use runtime symbol lookup via `vpi-shim` so plugins can be built without directly linking simulator libraries.
- `release_handle`: Call `vpi_release_handle` when dropping a `Handle`.
- `sv`: Enable SystemVerilog VPI extensions.
- `value_array`: Use array-based functions in VPI. If not, they are implemented using scalar access.
- `verilator`: Add some Verilator-specific extensions.

## Usage

Add this crate as a dependency in your plugin crate and build as `cdylib`.

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
vpi = "0.5.0"
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

- save/restart state APIs: `vpi_get_data`, `vpi_put_data`
- per-handle user data APIs: `vpi_get_userdata`, `vpi_put_userdata`
- explicit object lifetime API: `vpi_free_object`

The varargs print functions `vpi_vprintf` and `vpi_mcd_vprintf`
will not be supported in `vpi` since it is preferred that the
formatting is done in Rust. If you have a reason to use these from
Rust, let me know!

SystemVerilog support from `sv_vpi_user.h` is also partial at the high-level
API layer. The crate exposes many SV object and property constants through the
generic `ObjectType` and `Property` enums, and it supports assertion
callbacks, as well as helpers for constraint/randomization structures and
packages/interfaces/programs/virtual interfaces, but it does not yet provide
dedicated high-level helpers for
SV-specific modeling areas such as:

- class/type-spec trees and type parameters
- clocking blocks and clocking I/O
- property/sequence AST-style traversal helpers

If you need one of these areas today, use `vpi-sys` directly for that portion
and keep the rest of the plugin on the safe `vpi` API.
