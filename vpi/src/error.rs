use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Error severity levels reported by VPI.
#[repr(u32)]
#[derive(FromPrimitive)]
pub enum Severity {
    /// Informational notice.
    Notice = vpi_sys::vpiNotice,
    /// Warning that does not necessarily stop simulation.
    Warning = vpi_sys::vpiWarning,
    /// Error condition.
    Error = vpi_sys::vpiError,
    /// Simulator/system-level error.
    System = vpi_sys::vpiSystem,
    /// Internal simulator error.
    Internal = vpi_sys::vpiInternal,
}

/// Simulation phase/state where an error occurred.
#[repr(u32)]
#[derive(FromPrimitive)]
pub enum ErrorState {
    /// Compile-time context.
    Compile = vpi_sys::vpiCompile,
    /// PLI callback or API context.
    PLI = vpi_sys::vpiPLI,
    /// Runtime simulation context.
    Run = vpi_sys::vpiRun,
}

/// Rich error information returned by `vpi_chk_error`.
pub struct VPIError {
    /// Simulator-defined error code.
    pub code: String,
    /// Human-readable error message.
    pub message: String,
    /// Source file path, when provided by the simulator.
    pub file: Option<String>,
    /// Source line number, or `0` when unavailable.
    pub line: i32,
    /// Optional mapped error severity.
    pub severity: Option<Severity>,
    /// Optional mapped error state.
    pub state: Option<ErrorState>,
    /// Simulator product name reporting the error.
    pub product: String,
}

/// Checks whether the simulator has a pending VPI error.
///
/// Returns `None` when no error is present, otherwise returns the translated
/// [`VPIError`] payload.
#[must_use]
pub fn chk_error() -> Option<VPIError> {
    let mut error_info = vpi_sys::t_vpi_error_info {
        code: std::ptr::null_mut(),
        message: std::ptr::null_mut(),
        file: std::ptr::null_mut(),
        line: 0,
        level: 0,
        state: 0,
        product: std::ptr::null_mut(),
    };
    let error_code = unsafe { vpi_sys::vpi_chk_error(&raw mut error_info) };
    if error_code == 0 {
        None
    } else {
        Some(VPIError {
            code: unsafe { std::ffi::CStr::from_ptr(error_info.code) }
                .to_str()
                .unwrap_or("Unknown")
                .to_string(),
            message: unsafe { std::ffi::CStr::from_ptr(error_info.message) }
                .to_str()
                .unwrap_or("Unknown")
                .to_string(),
            file: if error_info.file.is_null() {
                None
            } else {
                Some(
                    unsafe { std::ffi::CStr::from_ptr(error_info.file) }
                        .to_str()
                        .unwrap_or("Unknown")
                        .to_string(),
                )
            },
            line: error_info.line,
            severity: Severity::from_i32(error_info.level),
            state: ErrorState::from_i32(error_info.state),
            product: unsafe { std::ffi::CStr::from_ptr(error_info.product) }
                .to_str()
                .unwrap_or("Unknown")
                .to_string(),
        })
    }
}

/// Alias of `chk_error` for consistency with rust-vhpi
#[must_use]
pub fn check_error() -> Option<VPIError> {
    chk_error()
}
