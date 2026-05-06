use std::ffi::CString;

/// Multi-channel descriptor used by VPI for output streams.
pub struct MCD {
    mask: u32,
}

/// Standard output MCD descriptor.
pub static MCD_STDOUT: MCD = MCD { mask: 0x1 };

impl MCD {
    /// Creates a new MCD for the given output file name.
    pub fn new(filename: impl AsRef<str>) -> Self {
        let c_filename = CString::new(filename.as_ref()).unwrap();
        let mask = unsafe { vpi_sys::vpi_mcd_open(c_filename.as_ptr().cast_mut()) };
        Self { mask }
    }

    /// Write a message to the MCD.
    pub fn write(&self, msg: impl AsRef<str>) {
        let cstr = CString::new(msg.as_ref()).unwrap();
        unsafe {
            vpi_sys::vpi_mcd_printf(self.mask, cstr.as_ptr().cast_mut());
        }
    }

    /// Write a message with a newline to the MCD.
    pub fn writeln(&self, msg: impl AsRef<str>) {
        self.write(format!("{}\n", msg.as_ref()));
    }

    /// Closes this MCD stream in the simulator.
    pub fn close(&self) {
        unsafe {
            vpi_sys::vpi_mcd_close(self.mask);
        }
    }

    /// Flushes any buffered MCD output.
    pub fn flush(&self) {
        unsafe {
            vpi_sys::vpi_mcd_flush(self.mask);
        }
    }

    #[must_use]
    /// Get the filename associated with this MCD, if any.
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

/// Combines two MCD descriptors into one destination mask.
impl std::ops::BitOr for MCD {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            mask: self.mask | rhs.mask,
        }
    }
}

/// Formats and writes a line to an [`MCD`].
#[macro_export]
macro_rules! mcd_println {
    ($mcd:expr, $($arg:tt)*) => {{
        $mcd.writeln(&format!($($arg)*));
    }}
}
