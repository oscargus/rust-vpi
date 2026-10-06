use std::ffi::CString;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Failure to open an MCD output stream.
#[derive(Debug)]
pub enum MCDOpenError {
    /// The filename contains an interior NUL byte.
    InvalidFilename(std::ffi::NulError),
    /// The simulator failed to open the requested output stream.
    OpenFailed,
}

impl std::fmt::Display for MCDOpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFilename(error) => write!(f, "invalid MCD filename: {error}"),
            Self::OpenFailed => write!(f, "simulator failed to open MCD output stream"),
        }
    }
}

impl std::error::Error for MCDOpenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidFilename(error) => Some(error),
            Self::OpenFailed => None,
        }
    }
}

/// An operation was attempted after an MCD descriptor was closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MCDClosedError;

impl std::fmt::Display for MCDClosedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MCD output stream is closed")
    }
}

impl std::error::Error for MCDClosedError {}

/// Failure to write a message to an MCD output stream.
#[derive(Debug)]
pub enum MCDWriteError {
    /// The descriptor has already been closed.
    Closed,
    /// The message contains an interior NUL byte.
    InvalidMessage(std::ffi::NulError),
}

impl std::fmt::Display for MCDWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed => write!(f, "MCD output stream is closed"),
            Self::InvalidMessage(error) => write!(f, "invalid MCD message: {error}"),
        }
    }
}

impl std::error::Error for MCDWriteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Closed => None,
            Self::InvalidMessage(error) => Some(error),
        }
    }
}

/// Multi-channel descriptor used by VPI for output streams.
///
/// Descriptors returned by [`MCD::try_new`] and [`MCD::new`] close their
/// opened streams when dropped. Built-in descriptors such as [`MCD_STDOUT`]
/// are not owned and are not closed on drop.
pub struct MCD {
    /// Bitmask identifying one or more simulator output channels.
    mask: u32,
    owned_mask: AtomicU32,
    closed: AtomicBool,
}

/// Standard output MCD descriptor.
pub static MCD_STDOUT: MCD = MCD {
    mask: 0x1,
    owned_mask: AtomicU32::new(0),
    closed: AtomicBool::new(false),
};

impl MCD {
    /// Opens an MCD output stream for the given file name.
    ///
    /// The stream is closed automatically when the returned descriptor is
    /// dropped. Returns an error if the filename contains an interior NUL or
    /// if the simulator cannot open the stream.
    pub fn try_new(filename: impl AsRef<str>) -> Result<Self, MCDOpenError> {
        let c_filename = CString::new(filename.as_ref()).map_err(MCDOpenError::InvalidFilename)?;
        let mask = unsafe { vpi_sys::vpi_mcd_open(c_filename.as_ptr().cast_mut()) };
        if mask == 0 {
            return Err(MCDOpenError::OpenFailed);
        }
        Ok(Self {
            mask,
            owned_mask: AtomicU32::new(mask),
            closed: AtomicBool::new(false),
        })
    }

    /// Opens an MCD output stream for the given file name.
    ///
    /// Panics if the filename is invalid or the simulator cannot open the
    /// stream. Use [`MCD::try_new`] to handle those failures.
    pub fn new(filename: impl AsRef<str>) -> Self {
        Self::try_new(filename).expect("failed to open MCD output stream")
    }

    /// Write a message to the MCD.
    ///
    /// Panics if the stream is closed or the message contains an interior NUL
    /// byte. Use [`MCD::try_write`] to handle these cases.
    pub fn write(&self, msg: impl AsRef<str>) {
        let _ = self
            .try_write(msg)
            .expect("cannot write to a closed MCD or a message containing NUL");
    }

    /// Writes a message and returns the raw `vpi_mcd_printf` result.
    ///
    /// Returns an error if the descriptor is closed or the message contains
    /// an interior NUL byte.
    pub fn try_write(&self, msg: impl AsRef<str>) -> Result<vpi_sys::PLI_INT32, MCDWriteError> {
        if self.closed.load(Ordering::Acquire) {
            return Err(MCDWriteError::Closed);
        }
        static FORMAT: &[u8] = b"%s\0";
        let cstr = CString::new(msg.as_ref()).map_err(MCDWriteError::InvalidMessage)?;
        Ok(unsafe {
            vpi_sys::vpi_mcd_printf(
                self.mask,
                FORMAT.as_ptr().cast_mut().cast(),
                cstr.as_ptr().cast_mut(),
            )
        })
    }

    /// Write a message with a newline to the MCD.
    ///
    /// Panics if the stream is closed or the message contains an interior NUL
    /// byte. Use [`MCD::try_writeln`] to handle these cases.
    pub fn writeln(&self, msg: impl AsRef<str>) {
        let _ = self
            .try_writeln(msg)
            .expect("cannot write to a closed MCD or a message containing NUL");
    }

    /// Writes a message with a newline and returns the raw VPI result.
    pub fn try_writeln(&self, msg: impl AsRef<str>) -> Result<vpi_sys::PLI_INT32, MCDWriteError> {
        self.try_write(format!("{}\n", msg.as_ref()))
    }

    /// Closes this MCD stream in the simulator.
    pub fn close(&self) {
        let owned_mask = self.owned_mask.swap(0, Ordering::AcqRel);
        if self.closed.swap(true, Ordering::AcqRel) {
            if owned_mask != 0 {
                unsafe {
                    vpi_sys::vpi_mcd_close(owned_mask);
                }
            }
        } else {
            unsafe {
                vpi_sys::vpi_mcd_close(self.mask);
            }
        }
    }

    /// Flushes any buffered MCD output.
    ///
    /// Panics if the stream has already been closed. Use [`MCD::try_flush`]
    /// to handle that case.
    pub fn flush(&self) {
        let _ = self.try_flush().expect("cannot flush a closed MCD");
    }

    /// Flushes the stream and returns the raw `vpi_mcd_flush` result.
    ///
    /// Returns an error if the descriptor has already been closed.
    pub fn try_flush(&self) -> Result<vpi_sys::PLI_INT32, MCDClosedError> {
        if self.closed.load(Ordering::Acquire) {
            return Err(MCDClosedError);
        }
        Ok(unsafe { vpi_sys::vpi_mcd_flush(self.mask) })
    }

    #[must_use]
    /// Get the filename associated with this MCD, if any.
    pub fn file_name(&self) -> Option<String> {
        if self.closed.load(Ordering::Acquire) {
            return None;
        }
        let ptr = unsafe { vpi_sys::vpi_mcd_name(self.mask) };
        if ptr.is_null() {
            None
        } else {
            Some(
                unsafe { std::ffi::CStr::from_ptr(ptr) }
                    .to_string_lossy()
                    .into_owned(),
            )
        }
    }
}

impl Drop for MCD {
    fn drop(&mut self) {
        let owned_mask = self.owned_mask.swap(0, Ordering::AcqRel);
        if owned_mask != 0 {
            self.closed.store(true, Ordering::Release);
            unsafe {
                vpi_sys::vpi_mcd_close(owned_mask);
            }
        }
    }
}

/// Combines two MCD descriptors into one destination mask.
impl std::ops::BitOr for MCD {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mask = self.mask | rhs.mask;
        let owned_mask =
            self.owned_mask.swap(0, Ordering::AcqRel) | rhs.owned_mask.swap(0, Ordering::AcqRel);
        let closed = self.closed.load(Ordering::Acquire) || rhs.closed.load(Ordering::Acquire);
        Self {
            mask,
            owned_mask: AtomicU32::new(owned_mask),
            closed: AtomicBool::new(closed),
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

#[cfg(test)]
mod tests {
    use super::{MCDClosedError, MCDOpenError, MCDWriteError, MCD};
    use std::sync::atomic::{AtomicBool, AtomicU32};

    #[test]
    fn try_new_reports_invalid_filename() {
        assert!(matches!(
            MCD::try_new("bad\0filename"),
            Err(MCDOpenError::InvalidFilename(_))
        ));
    }

    #[cfg(not(all(feature = "dynamic", any(target_os = "windows", target_os = "macos"))))]
    #[test]
    fn try_new_reports_simulator_open_failure() {
        assert!(matches!(
            MCD::try_new("output.log"),
            Err(MCDOpenError::OpenFailed)
        ));
    }

    #[test]
    fn closed_mcd_rejects_io() {
        let mcd = MCD {
            mask: 1,
            owned_mask: AtomicU32::new(0),
            closed: AtomicBool::new(true),
        };

        assert!(matches!(
            mcd.try_write("message"),
            Err(MCDWriteError::Closed)
        ));
        assert_eq!(mcd.try_flush(), Err(MCDClosedError));
    }

    #[test]
    fn try_write_reports_interior_nul() {
        let mcd = MCD {
            mask: 1,
            owned_mask: AtomicU32::new(0),
            closed: AtomicBool::new(false),
        };

        assert!(matches!(
            mcd.try_write("bad\0message"),
            Err(MCDWriteError::InvalidMessage(_))
        ));
    }
}
