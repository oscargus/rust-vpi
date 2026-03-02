use std::ffi::CString;

pub struct MCD {
    mask: u32,
}

pub static MCD_STDOUT: MCD = MCD { mask: 0x1 };

impl MCD {
    pub fn write(&self, msg: impl AsRef<str>) {
        let cstr = CString::new(msg.as_ref()).unwrap();
        unsafe {
            vpi_sys::vpi_mcd_printf(self.mask, cstr.as_ptr().cast_mut());
        }
    }

    pub fn open(filename: impl AsRef<str>) -> Self {
        let c_filename = crate::string_to_ascii_cstring(filename);
        let mask = unsafe { vpi_sys::vpi_mcd_open(c_filename.as_ptr().cast_mut()) };
        Self { mask }
    }

    pub fn close(&self) {
        unsafe {
            vpi_sys::vpi_mcd_close(self.mask);
        }
    }

    pub fn flush(&self) {
        unsafe {
            vpi_sys::vpi_mcd_flush(self.mask);
        }
    }

    #[must_use]
    pub fn file_name(&self) -> Option<String> {
        let ptr = unsafe { vpi_sys::vpi_mcd_name(self.mask) };
        if ptr.is_null() {
            None
        } else {
            let cstr = unsafe { CString::from_raw(ptr) };
            cstr.into_string().ok()
        }
    }
}

impl std::ops::BitOr for MCD {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            mask: self.mask | rhs.mask,
        }
    }
}

#[macro_export]
macro_rules! mcd_println {
    ($mcd:expr, $($arg:tt)*) => {{
        $mcd.write(&format!("{}\n", format!($($arg)*)));
    }}
}
