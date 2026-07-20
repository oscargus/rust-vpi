# vpi, vpi-sys, and vpi-shim

[![CI](https://github.com/oscargus/rust-vpi/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/oscargus/rust-vpi/actions/workflows/ci.yml)
[![crates.io vpi](https://img.shields.io/crates/v/vpi.svg)](https://crates.io/crates/vpi)
[![docs.rs vpi](https://docs.rs/vpi/badge.svg)](https://docs.rs/vpi)
[![MSRV vpi](https://img.shields.io/badge/MSRV-1.88-blue)](https://github.com/oscargus/rust-vpi)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## vpi

vpi is a Rust crate for easily working with the Verilog Procedural Interface (VPI) through Rust abstractions. There are several similar libraries, but the unique features of vpi are:

- **Rust abstractions**: The user should only have to deal with Rust types and it should be easy to access things in a (relatively) safe way.
- **Features**: There are a number of features that can be enables, either for increased functionality or depending on simulator and simulator VPI support.

| Feature | Description | Default |
| ------- | ----------- | ------- |
| `bigint` | Enables conversion between `LogicVec` and arbitrary-precision integers using `num_bigint::BigInt` and `num_bigint::BigUint`. | No |
| `cb_info` | Uses `vpi_get_cb_info` when removing callbacks. | Yes |
| `dynamic` | Enables runtime VPI symbol lookup via `vpi-shim` on Windows and macOS, allowing plugins to build without directly linking to a simulator library. | No |
| `release_handle` | Calls `vpi_release_handle` when dropping a `Handle`. | No |
| `sv` | Enables SystemVerilog VPI extensions (types, callbacks, and properties defined in IEEE 1800). | No |
| `value_array` | Enables support for VPI array values via `vpi_get_value_array` and `vpi_put_value_array`. Otherwise the related functions are still available, but use repeated calls to the scalar `vpi_get_value` and `vpi_put_value` functions. | No |
| `verilator` | Enables support for Verilator-specific VPI extensions, including two-state and four-state raw vector values. | No |

## vpi-sys

vpi-sys is a simple bindgen-wrapper that automatically generates Rust-bindings.

## vpi-shim

vpi-shim is a small shim crate to enable dynamic lookup of VPI symbols at runtime. This is required on Windows and Mac unless you link with the simulator directly.

## Examples

There are examples in `test_example` to see how the crate can be used.

## Coverage

Coverage reports are [published](https://oscargus.github.io/rust-vpi/) based on the latest commit on main.

## Status

For more details about what the safe `vpi` crate currently wraps from
`vpi_user.h` / `sv_vpi_user.h`, and which lower-level areas still require
`vpi-sys`, see [vpi/README.md](vpi/README.md).

## License

All three crates are licensed under the MIT license. However, I do not claim any license for the .h-files in vpi-sys, which are based on the IEEE 1800 standard.
