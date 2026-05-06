//! Safe and ergonomic wrappers for selected Verilog VPI APIs.

#[macro_use]
mod macros;

mod callback;
mod control;
mod error;
mod handle;
mod mcd;
mod object;
mod property;
mod simulator;
mod time;
mod value;

use std::ffi::CString;

pub use callback::*;
pub use control::*;
pub use error::*;
pub use handle::*;
pub use mcd::*;
pub use object::*;
pub use property::*;
pub use simulator::*;
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
            FMT.as_ptr().cast::<i8>().cast_mut(),
            cstr.as_ptr().cast_mut(),
        )
    };
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
