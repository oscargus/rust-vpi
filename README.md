# vpi and vpi-sys

[![CI](https://github.com/oscargus/rust-vpi/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/oscargus/rust-vpi/actions/workflows/ci.yml)
[![crates.io vpi](https://img.shields.io/crates/v/vpi.svg)](https://crates.io/crates/vpi)
[![docs.rs vpi](https://docs.rs/vpi/badge.svg)](https://docs.rs/vpi)
[![MSRV vpi](https://img.shields.io/badge/MSRV-1.88-blue)](https://github.com/oscargus/rust-vpi)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

vpi is a Rust crate for easily working with the Verilog Procedural Interface (VPI) through Rust abstractions. There are several similar libraries, but the unique features of vpi are:

- **Rust abstractions**: The user should only have to deal with Rust types and it should be easy to access things in a (relatively) safe way.
- **Features**: The inclusion of extensions are optional. Currently, there is the `sv` feature to include SystemVerilog extensions, and the `cb_info` feature is enabled by default for simulators that support `vpi_get_cb_info`, but one can also consider simulator-specific ones.

vpi-sys is a simple bindgen-wrapper that automatically generates Rust-bindings.

There are examples in `test_example` to see how the crate can be used.

Coverage reports are published from the dedicated coverage workflow:

- GitHub Pages: https://oscargus.github.io/rust-vpi/

For more details about what the safe `vpi` crate currently wraps from
`vpi_user.h` / `sv_vpi_user.h`, and which lower-level areas still require
`vpi-sys`, see [vpi/README.md](vpi/README.md).

Both crates are licensed under the MIT license. However, I do not claim any license for the .h-files in vpi-sys, which are based on the IEEE 1800 standard.
