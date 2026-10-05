# vpi-shim

[![crates.io](https://img.shields.io/crates/v/vpi-shim.svg)](https://crates.io/crates/vpi-shim)
[![docs.rs](https://docs.rs/vpi-shim/badge.svg)](https://docs.rs/vpi-shim)

Runtime symbol shim for VPI plugins on platforms that require explicit symbol resolution.

This crate exports VPI entry points and resolves simulator symbols dynamically at runtime.

## Platform behavior

- Windows: Resolves symbols from the host process using `GetModuleHandleA` and `GetProcAddress`.
- macOS: Resolves symbols with `dlsym` (first `RTLD_DEFAULT`, then the host module from `dlopen(NULL, RTLD_NOW)`).
- Linux: Typically not required because plugin symbol resolution is usually handled by the simulator loader.

The variadic `vpi_printf` and `vpi_mcd_printf` entry points use a C bridge to
forward their arguments through the simulator's `vpi_vprintf` and
`vpi_mcd_vprintf` functions, preserving the platform's variadic calling
convention.

## How it is used

`vpi-shim` is intended to be pulled in through the `vpi` crate's `dynamic` feature.
