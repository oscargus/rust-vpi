use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[repr(u32)]
#[derive(FromPrimitive)]
pub enum Severity {
    Notice = vpi_sys::vpiNotice,
    Warning = vpi_sys::vpiWarning,
    Error = vpi_sys::vpiError,
    System = vpi_sys::vpiSystem,
    Internal = vpi_sys::vpiInternal,
}

#[repr(u32)]
#[derive(FromPrimitive)]
pub enum ErrorState {
    Compile = vpi_sys::vpiCompile,
    PLI = vpi_sys::vpiPLI,
    Run = vpi_sys::vpiRun,
}

pub struct VPIError {
    pub code: String,
    pub message: String,
    pub file: Option<String>,
    pub line: i32,
    pub severity: Option<Severity>,
    pub state: Option<ErrorState>,
    pub product: String,
}

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
