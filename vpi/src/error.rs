use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Error severity levels reported by VPI.
#[repr(u32)]
#[derive(FromPrimitive, Debug, Clone, PartialEq, Eq)]
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

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Notice => write!(f, "Notice"),
            Severity::Warning => write!(f, "Warning"),
            Severity::Error => write!(f, "Error"),
            Severity::System => write!(f, "System"),
            Severity::Internal => write!(f, "Internal"),
        }
    }
}

/// Simulation phase/state where an error occurred.
#[repr(u32)]
#[derive(FromPrimitive, Debug, Clone, PartialEq, Eq)]
pub enum ErrorState {
    /// Compile-time context.
    Compile = vpi_sys::vpiCompile,
    /// PLI callback or API context.
    PLI = vpi_sys::vpiPLI,
    /// Runtime simulation context.
    Run = vpi_sys::vpiRun,
}

impl std::fmt::Display for ErrorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorState::Compile => write!(f, "Compile"),
            ErrorState::PLI => write!(f, "PLI"),
            ErrorState::Run => write!(f, "Run"),
        }
    }
}
/// Rich error information returned by `vpi_chk_error`.
#[derive(Debug, Clone)]
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

impl std::fmt::Display for VPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let severity_str = self.severity.as_ref().map(std::string::ToString::to_string);
        write!(
            f,
            "[{}] {} ({}:{}) - {}",
            severity_str.as_deref().unwrap_or("Unknown"),
            self.message,
            self.file.as_deref().unwrap_or("Unknown"),
            self.line,
            self.product
        )
    }
}

/// Missing required data returned by `vpi_chk_error`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VPIErrorInfoError {
    /// The simulator reported an error without a code string.
    MissingCode,
    /// The simulator reported an error without a message string.
    MissingMessage,
    /// The simulator reported an error without a product string.
    MissingProduct,
}

impl std::fmt::Display for VPIErrorInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCode => write!(f, "VPI error information has no code"),
            Self::MissingMessage => write!(f, "VPI error information has no message"),
            Self::MissingProduct => write!(f, "VPI error information has no product"),
        }
    }
}

impl std::error::Error for VPIErrorInfoError {}

fn copy_c_string(ptr: *const std::ffi::c_char) -> Option<String> {
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

/// Checks whether the simulator has a pending VPI error.
///
/// Returns `None` when no error is present, otherwise returns the translated
/// [`VPIError`] payload.
#[must_use]
pub fn chk_error() -> Option<VPIError> {
    try_chk_error().expect("simulator returned incomplete VPI error information")
}

/// Fallible version of [`chk_error`].
///
/// Returns `Ok(None)` when no error is pending. Returns an error if the
/// simulator reports an error but omits a required string field.
pub fn try_chk_error() -> Result<Option<VPIError>, VPIErrorInfoError> {
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
        Ok(None)
    } else {
        let code = copy_c_string(error_info.code).ok_or(VPIErrorInfoError::MissingCode)?;
        let message = copy_c_string(error_info.message).ok_or(VPIErrorInfoError::MissingMessage)?;
        let product = copy_c_string(error_info.product).ok_or(VPIErrorInfoError::MissingProduct)?;
        Ok(Some(VPIError {
            code,
            message,
            file: copy_c_string(error_info.file),
            line: error_info.line,
            severity: Severity::from_i32(error_info.level),
            state: ErrorState::from_i32(error_info.state),
            product,
        }))
    }
}

/// Alias of `chk_error` for consistency with rust-vhpi
#[must_use]
pub fn check_error() -> Option<VPIError> {
    chk_error()
}

#[cfg(test)]
mod tests {
    use super::{try_chk_error, VPIErrorInfoError};

    #[cfg(not(all(feature = "dynamic", any(target_os = "windows", target_os = "macos"))))]
    #[test]
    fn try_chk_error_returns_none_when_no_error_is_pending() {
        assert!(matches!(try_chk_error(), Ok(None)));
    }

    #[test]
    fn missing_c_string_is_reported() {
        assert_eq!(super::copy_c_string(std::ptr::null()), None::<String>);

        let error = VPIErrorInfoError::MissingMessage;
        assert_eq!(error.to_string(), "VPI error information has no message");
    }
}
