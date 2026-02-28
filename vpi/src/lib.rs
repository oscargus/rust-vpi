#[macro_use]
mod macros;

mod callback;
mod control;
mod error;
mod handle;
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
pub use object::*;
pub use property::*;
pub use simulator::*;
pub use time::*;
pub use value::*;

pub fn printf(msg: impl AsRef<str>) {
    static FMT: &[u8] = b"%s\n\0";
    let cstr = string_to_iso8859_1_cstring(msg);
    unsafe {
        vpi_sys::vpi_printf(
            FMT.as_ptr().cast::<i8>() as *mut i8,
            cstr.as_ptr() as *mut i8,
        )
    };
}

#[macro_export]
macro_rules! printf {
    ($($arg:tt)*) => {{
        $crate::printf(&format!($($arg)*));
    }}
}

/// Convert Rust string to ASCII < 128 encoded C string
/// Characters outside of ASCII range are replaced with ?
pub fn string_to_iso8859_1_cstring(msg: impl AsRef<str>) -> CString {
    // Convert UTF-8 string to ISO-8859-1 bytes
    let iso8859_1_bytes: Vec<u8> = msg
        .as_ref()
        .chars()
        .map(|c| {
            let code = c as u32;
            if code <= 0x7F {
                code as u8
            } else {
                b'?' // Replace characters outside ASCII range
            }
        })
        .collect();
    CString::new(iso8859_1_bytes).unwrap()
}
