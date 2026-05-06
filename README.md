# vpi and vpi-sys

vpi is a Rust crate for easily working with the Verilog Procedural Interface (VPI) through Rust abstractions. There are several similar libraries, but the unique features of vpi are:

- **Rust abstractions**: The user should only have to deal with Rust types and it should be easy to access things in a (relatively) safe way.
- **Features**: The inclusion of extensions are optional. Currently, there is the `sv` feature to include SystemVerilog extensions, but one can also consider simulator-specific ones.

vpi-sys is a simple bindgen-wrapper that automatically generates Rust-bindings.

There are examples in `test_example` to see how the crate can be used.

Both crates are licensed under the MIT license. However, I do not claim any license for the .h-file in vpi-sys, which are based on the IEEE 1800 standard.
