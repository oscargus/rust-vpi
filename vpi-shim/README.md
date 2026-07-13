# vpi-shim

Runtime symbol shim for VPI plugins on platforms that require explicit symbol resolution.

This crate exports VPI entry points and resolves simulator symbols dynamically at runtime.

## Platform behavior

- Windows: Uses dynamic loading (`libloading`) to resolve symbols from simulator libraries.
- macOS: Uses dynamic linker APIs to resolve symbols at runtime.
- Linux: Typically not required because plugin symbol resolution is usually handled by the simulator loader.

## How it is used

`vpi-shim` is intended to be pulled in through the `vpi` crate's `dynamic` feature.
