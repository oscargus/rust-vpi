//! Safe and ergonomic wrappers for selected Verilog VPI APIs.
//!
//! # Features
//!
//! | Feature | Description | Default |
//! |---------|-------------|---------|
//! | `bigint` | Enables conversion between [`LogicVec`] and arbitrary-precision integers using [`num_bigint::BigInt`] and [`num_bigint::BigUint`]. | No |
//! | `cb_info` | Uses `vpi_get_cb_info` when removing callbacks. | Yes |
//! | `dynamic` | Enables runtime VPI symbol lookup via `vpi-shim` on Windows and macOS, allowing plugins to build without directly linking to a simulator library. | No |
//! | `release_handle` | Calls `vpi_release_handle` when dropping a [`Handle`]. | No |
//! | `sv`     | Enables SystemVerilog VPI extensions (types, callbacks, and properties defined in IEEE 1800). | No |
//! | `value_array` | Enables support for VPI array values via `vpi_get_value_array` and `vpi_put_value_array`. Otherwise the related functions are still available, but use repeated calls to the scalar `vpi_get_value` and `vpi_put_value` functions. | No |
//! | `verilator` | Enables support for Verilator-specific VPI extensions, including two-state and four-state raw vector values. | No |
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(all(feature = "dynamic", any(target_os = "windows", target_os = "macos")))]
use vpi_shim as _;

#[macro_use]
mod macros;

mod callback;
mod control;
mod delays;
mod error;
mod handle;
mod logic;
mod mcd;
mod object;
mod property;
mod simulator;
mod systf;
mod test_vpi_stubs;
mod time;
mod value;

use std::ffi::CString;

pub use callback::*;
pub use control::*;
pub use delays::*;
pub use error::*;
pub use handle::*;
pub use logic::*;
pub use mcd::*;
pub use object::*;
pub use property::*;
pub use simulator::*;
pub use systf::*;
pub use time::*;
pub use value::*;

/// Prints a message through the simulator's `vpi_printf`.
///
/// A trailing newline is appended automatically.
pub fn printf(msg: impl AsRef<str>) {
    static FMT: &[u8] = b"%s\n\0";
    let cstr = string_to_ascii_cstring(msg);
    unsafe {
        vpi_sys::vpi_printf(
            FMT.as_ptr().cast::<vpi_sys::PLI_BYTE8>().cast_mut(),
            cstr.as_ptr().cast::<vpi_sys::PLI_BYTE8>().cast_mut(),
        )
    };
}

/// Flushes the simulator's default output streams via `vpi_flush`.
///
/// Returns `Ok(())` on success (`0`) and `Err(code)` otherwise.
pub fn flush() -> Result<(), i32> {
    let code = unsafe { vpi_sys::vpi_flush() };
    if code == 0 {
        Ok(())
    } else {
        Err(code)
    }
}

/// `format!`-style wrapper around [`printf`].
#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {{
        $crate::printf(&format!($($arg)*));
    }}
}

/// Converts a Rust string into a 7-bit ASCII `CString`.
///
/// Characters outside the ASCII range are replaced with `?`.
pub fn string_to_ascii_cstring(msg: impl AsRef<str>) -> CString {
    let sevenbit_ascii_bytes: Vec<u8> = msg
        .as_ref()
        .chars()
        .map(|c| {
            let code = c as u32;
            if let Ok(code) = u8::try_from(code) {
                if code < 128 {
                    code // 7-bit ASCII character
                } else {
                    b'?' // Replace characters outside 7-bit ASCII with '?'
                }
            } else {
                b'?' // Replace characters outside 7-bit ASCII with '?'
            }
        })
        .collect();
    CString::new(sevenbit_ascii_bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::string_to_ascii_cstring;

    #[test]
    fn ascii_is_preserved() {
        let cstr = string_to_ascii_cstring("Hello, VPI!");
        assert_eq!(cstr.to_bytes(), b"Hello, VPI!");
    }

    #[test]
    fn non_ascii_chars_are_replaced_with_question_mark() {
        let cstr = string_to_ascii_cstring("café Ω");
        assert_eq!(cstr.to_bytes(), b"caf? ?");
    }

    #[test]
    fn mixed_ascii_and_non_ascii_is_converted_correctly() {
        let cstr = string_to_ascii_cstring("AéB中C");
        assert_eq!(cstr.to_bytes(), b"A?B?C");
    }

    #[test]
    #[should_panic]
    fn interior_nul_panics() {
        let _ = string_to_ascii_cstring("abc\0def");
    }
}
