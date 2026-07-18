//! Helpers for registering and implementing VPI system tasks/functions.
//!
//! This module provides typed wrappers around `vpi_register_systf` and
//! convenience helpers for reading system task/function arguments inside
//! `calltf` callbacks.
//!
//! # Example
//!
//! ```no_run
//! use std::ffi::CStr;
//! use std::os::raw::c_char;
//! use vpi::{
//!     current_systf_call, get_systf_arg, register_systf, SysFuncType, SystfKind, Value,
//!     ValueType,
//! };
//!
//! static FUNC_NAME: &CStr = c"$add_one";
//!
//! unsafe extern "C" fn compiletf(_user_data: *mut c_char) -> i32 {
//!     0
//! }
//!
//! unsafe extern "C" fn calltf_add_one(_user_data: *mut c_char) -> i32 {
//!     let arg = match get_systf_arg(0, ValueType::Int) {
//!         Some(Value::Int(v)) => v,
//!         _ => 0,
//!     };
//!     let result = arg + 1;
//!
//!     let call = current_systf_call();
//!     let _ = call.put_value(&Value::Int(result));
//!     0
//! }
//!
//! let _func_handle = register_systf(
//!     SystfKind::Func,
//!     FUNC_NAME,
//!     Some(calltf_add_one),
//!     Some(compiletf),
//!     None,
//!     std::ptr::null_mut(),
//!     Some(SysFuncType::Sized),
//! );
//! ```
//!
//! In Verilog/SystemVerilog, the registered function is called using its `$`
//! name from expressions like a built-in system function:
//!
//! ```verilog
//! module tb;
//!   integer x;
//!   initial begin
//!     x = $add_one(41);
//!     $display("x=%0d", x); // expected: x=42
//!   end
//! endmodule
//! ```

use std::ffi::CStr;
use std::os::raw::c_char;

use crate::{Handle, ObjectType, SysFuncType, Value, ValueType};

/// Raw VPI registration record type.
pub type RawSystfData = vpi_sys::s_vpi_systf_data;

/// Function pointer type for VPI system task/function callbacks.
pub type SystfCallback = unsafe extern "C" fn(*mut c_char) -> i32;

/// System task/function registration kind.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SystfKind {
    /// Register a system task.
    Task = vpi_sys::vpiSysTask,
    /// Register a system function.
    Func = vpi_sys::vpiSysFunc,
}

/// Registers a raw `s_vpi_systf_data` record with the simulator.
///
/// This is a thin wrapper around `vpi_register_systf`.
#[must_use]
pub fn register_systf_raw(data: &mut RawSystfData) -> Handle {
    let handle = unsafe { vpi_sys::vpi_register_systf(data) };
    Handle::from_raw(handle)
}

/// Registers a system task or function with typed inputs.
///
/// `name` must be static and begin with `$` to satisfy VPI requirements.
///
/// For [`SystfKind::Task`], `sys_func_type` is ignored and set to `vpiSysTask`.
/// For [`SystfKind::Func`], `sys_func_type` defaults to `vpiIntFunc` when omitted.
#[must_use]
pub fn register_systf(
    kind: SystfKind,
    name: &'static CStr,
    calltf: Option<SystfCallback>,
    compiletf: Option<SystfCallback>,
    sizetf: Option<SystfCallback>,
    user_data: *mut c_char,
    sys_func_type: Option<SysFuncType>,
) -> Handle {
    let sysfunctype = match kind {
        SystfKind::Task => vpi_sys::vpiSysTask as i32,
        SystfKind::Func => sys_func_type.map_or(vpi_sys::vpiIntFunc as i32, |t| t as i32),
    };

    let mut data = RawSystfData {
        type_: kind as i32,
        sysfunctype,
        tfname: name.as_ptr().cast_mut(),
        calltf,
        compiletf,
        sizetf,
        user_data,
    };

    register_systf_raw(&mut data)
}

/// Returns the current system task/function call handle.
///
/// This is typically used inside `calltf` callbacks to access arguments or
/// assign a return value for system functions.
#[must_use]
pub fn current_systf_call() -> Handle {
    Handle::null().get(ObjectType::SysTfCall)
}

/// Returns one system task/function argument by index in a requested format.
///
/// The index is zero-based (`0` is the first argument).
/// Returns `None` when called outside a `calltf` context, when the index is
/// out of range, or when the simulator cannot provide the requested value
/// format.
#[must_use]
pub fn get_systf_arg(index: u32, format: ValueType) -> Option<Value> {
    let call = current_systf_call();
    if call.is_null() {
        return None;
    }

    call.iterator(ObjectType::Argument)
        .nth(index as usize)
        .and_then(|arg| arg.get_value(format))
}

/// Returns system task/function arguments using per-argument value formats.
///
/// Each entry in `formats` corresponds to one argument position and controls
/// the `vpi_get_value` format requested for that argument.
///
/// The returned vector has the same length as `formats`:
///
/// - `Some(Value)` when the argument exists and can be decoded in the
///   requested format.
/// - `None` when called outside a `calltf` context, when the argument is
///   missing, or when value retrieval fails.
#[must_use]
pub fn get_systf_args(formats: &[ValueType]) -> Vec<Option<Value>> {
    let call = current_systf_call();
    if call.is_null() {
        return std::iter::repeat_with(|| None)
            .take(formats.len())
            .collect();
    }

    let mut args = call.iterator(ObjectType::Argument);
    formats
        .iter()
        .map(|format| args.next().and_then(|arg| arg.get_value(*format)))
        .collect()
}
