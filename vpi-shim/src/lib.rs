//! Dynamic VPI symbol shim for plugin builds on Windows and macOS.
//!
//! On these platforms, some linkers require all external symbols to be resolved
//! when building the plugin binary. This crate exports a subset of common VPI
//! symbols and resolves the real implementations at runtime.

#![allow(non_snake_case)]

#[cfg(target_os = "macos")]
use std::ffi::c_int;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::ffi::{c_char, c_void};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::sync::OnceLock;

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn missing_symbol(name: &str) -> ! {
    eprintln!("vpi-shim: failed to resolve symbol '{name}'.");
    std::process::abort();
}

#[cfg(target_os = "macos")]
unsafe fn resolve_symbol(name: &[u8]) -> *mut c_void {
    unsafe extern "C" {
        fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }

    const RTLD_NOW: c_int = 0x2;
    const RTLD_DEFAULT: *mut c_void = (-2isize) as *mut c_void;

    static HOST_MODULE: OnceLock<usize> = OnceLock::new();

    fn host_module() -> *mut c_void {
        let module = *HOST_MODULE.get_or_init(|| unsafe {
            let module = dlopen(std::ptr::null(), RTLD_NOW);
            if module.is_null() {
                eprintln!("vpi-shim: dlopen(NULL, RTLD_NOW) failed");
                std::process::abort();
            }
            module as usize
        });
        module as *mut c_void
    }

    let mut ptr = unsafe { dlsym(RTLD_DEFAULT, name.as_ptr().cast()) };
    if ptr.is_null() {
        ptr = unsafe { dlsym(host_module(), name.as_ptr().cast()) };
    }
    ptr
}

#[cfg(target_os = "windows")]
unsafe fn resolve_symbol(name: &[u8]) -> *mut c_void {
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetModuleHandleA(module_name: *const c_char) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, proc_name: *const c_char) -> *mut c_void;
    }

    static HOST_MODULE: OnceLock<usize> = OnceLock::new();

    fn host_module() -> *mut c_void {
        let module = *HOST_MODULE.get_or_init(|| unsafe {
            let module = GetModuleHandleA(std::ptr::null());
            if module.is_null() {
                eprintln!("vpi-shim: GetModuleHandleA(NULL) failed");
                std::process::abort();
            }
            module as usize
        });
        module as *mut c_void
    }

    unsafe { GetProcAddress(host_module(), name.as_ptr().cast()) }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
macro_rules! forward_fn {
    ($(fn $name:ident($($arg:ident : $arg_ty:ty),* $(,)?) -> $ret:ty;)+) => {
        $(
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn $name($($arg: $arg_ty),*) -> $ret {
                type FnTy = unsafe extern "C" fn($($arg_ty),*) -> $ret;
                static FN: OnceLock<FnTy> = OnceLock::new();
                let f = *FN.get_or_init(|| {
                    let symbol = concat!(stringify!($name), "\0").as_bytes();
                    let ptr = unsafe { resolve_symbol(symbol) };
                    if ptr.is_null() {
                        missing_symbol(stringify!($name));
                    }
                    unsafe { std::mem::transmute::<*mut c_void, FnTy>(ptr) }
                });
                unsafe { f($($arg),*) }
            }
        )+
    };
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
macro_rules! forward_fn_void {
    ($(fn $name:ident($($arg:ident : $arg_ty:ty),* $(,)?);)+) => {
        $(
            #[unsafe(no_mangle)]
            pub unsafe extern "C" fn $name($($arg: $arg_ty),*) {
                type FnTy = unsafe extern "C" fn($($arg_ty),*);
                static FN: OnceLock<FnTy> = OnceLock::new();
                let f = *FN.get_or_init(|| {
                    let symbol = concat!(stringify!($name), "\0").as_bytes();
                    let ptr = unsafe { resolve_symbol(symbol) };
                    if ptr.is_null() {
                        missing_symbol(stringify!($name));
                    }
                    unsafe { std::mem::transmute::<*mut c_void, FnTy>(ptr) }
                });
                unsafe { f($($arg),*) }
            }
        )+
    };
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
forward_fn! {
    fn vpi_register_cb(cb_data_p: vpi_sys::p_cb_data) -> vpi_sys::vpiHandle;
    fn vpi_register_systf(systf_data_p: vpi_sys::p_vpi_systf_data) -> vpi_sys::vpiHandle;
    fn vpi_remove_cb(cb_obj: vpi_sys::vpiHandle) -> vpi_sys::PLI_INT32;
    fn vpi_handle(type_: vpi_sys::PLI_INT32, refHandle: vpi_sys::vpiHandle) -> vpi_sys::vpiHandle;
    fn vpi_handle_multi(type_: vpi_sys::PLI_INT32, refHandle1: vpi_sys::vpiHandle, refHandle2: vpi_sys::vpiHandle) -> vpi_sys::vpiHandle;
    fn vpi_handle_by_name(name: *mut vpi_sys::PLI_BYTE8, scope: vpi_sys::vpiHandle) -> vpi_sys::vpiHandle;
    fn vpi_handle_by_index(object: vpi_sys::vpiHandle, indx: vpi_sys::PLI_INT32) -> vpi_sys::vpiHandle;
    fn vpi_handle_by_multi_index(obj: vpi_sys::vpiHandle, num_index: vpi_sys::PLI_INT32, index_array: *mut vpi_sys::PLI_INT32) -> vpi_sys::vpiHandle;
    fn vpi_iterate(type_: vpi_sys::PLI_INT32, refHandle: vpi_sys::vpiHandle) -> vpi_sys::vpiHandle;
    fn vpi_scan(iterator: vpi_sys::vpiHandle) -> vpi_sys::vpiHandle;
    fn vpi_get(property: vpi_sys::PLI_INT32, object: vpi_sys::vpiHandle) -> vpi_sys::PLI_INT32;
    fn vpi_get64(property: vpi_sys::PLI_INT32, object: vpi_sys::vpiHandle) -> vpi_sys::PLI_INT64;
    fn vpi_get_str(property: vpi_sys::PLI_INT32, object: vpi_sys::vpiHandle) -> *mut vpi_sys::PLI_BYTE8;
    fn vpi_get_vlog_info(vlog_info_p: vpi_sys::p_vpi_vlog_info) -> vpi_sys::PLI_INT32;
    fn vpi_control(operation: vpi_sys::PLI_INT32) -> vpi_sys::PLI_INT32;
    fn vpi_compare_objects(object1: vpi_sys::vpiHandle, object2: vpi_sys::vpiHandle) -> vpi_sys::PLI_INT32;
    fn vpi_chk_error(error_info_p: vpi_sys::p_vpi_error_info) -> vpi_sys::PLI_INT32;
    fn vpi_release_handle(object: vpi_sys::vpiHandle) -> vpi_sys::PLI_INT32;
    fn vpi_flush() -> vpi_sys::PLI_INT32;
    fn vpi_put_value(
        object: vpi_sys::vpiHandle,
        value_p: vpi_sys::p_vpi_value,
        time_p: vpi_sys::p_vpi_time,
        flags: vpi_sys::PLI_INT32,
    ) -> vpi_sys::vpiHandle;
    fn vpi_mcd_open(fileName: *mut vpi_sys::PLI_BYTE8) -> vpi_sys::PLI_UINT32;
    fn vpi_mcd_close(mcd: vpi_sys::PLI_UINT32) -> vpi_sys::PLI_UINT32;
    fn vpi_mcd_name(cd: vpi_sys::PLI_UINT32) -> *mut vpi_sys::PLI_BYTE8;
    fn vpi_mcd_flush(mcd: vpi_sys::PLI_UINT32) -> vpi_sys::PLI_INT32;

    // Forward common fixed-arity call patterns used by this workspace.
    fn vpi_printf(format: *mut vpi_sys::PLI_BYTE8, arg: *mut vpi_sys::PLI_BYTE8) -> vpi_sys::PLI_INT32;
    fn vpi_mcd_printf(mcd: vpi_sys::PLI_UINT32, format: *mut vpi_sys::PLI_BYTE8, arg: *mut vpi_sys::PLI_BYTE8) -> vpi_sys::PLI_INT32;
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
forward_fn_void! {
    fn vpi_get_cb_info(object: vpi_sys::vpiHandle, cb_data_p: vpi_sys::p_cb_data);
    fn vpi_get_systf_info(object: vpi_sys::vpiHandle, systf_data_p: vpi_sys::p_vpi_systf_data);
    fn vpi_get_delays(object: vpi_sys::vpiHandle, delay_p: vpi_sys::p_vpi_delay);
    fn vpi_put_delays(object: vpi_sys::vpiHandle, delay_p: vpi_sys::p_vpi_delay);
    fn vpi_get_time(object: vpi_sys::vpiHandle, time_p: vpi_sys::p_vpi_time);
    fn vpi_get_value(expr: vpi_sys::vpiHandle, value_p: vpi_sys::p_vpi_value);
    fn vpi_get_value_array(
        expr: vpi_sys::vpiHandle,
        arrayvalue_p: vpi_sys::p_vpi_arrayvalue,
        index_p: *mut vpi_sys::PLI_INT32,
        num: vpi_sys::PLI_UINT32,
    );
    fn vpi_put_value_array(
        object: vpi_sys::vpiHandle,
        arrayvalue_p: vpi_sys::p_vpi_arrayvalue,
        index_p: *mut vpi_sys::PLI_INT32,
        num: vpi_sys::PLI_UINT32,
    );
}

#[cfg(all(feature = "sv", any(target_os = "windows", target_os = "macos")))]
forward_fn! {
    fn vpi_register_assertion_cb(
        assertion: vpi_sys::vpiHandle,
        reason: vpi_sys::PLI_INT32,
        cb_rtn: vpi_sys::vpi_assertion_callback_func,
        user_data: *mut vpi_sys::PLI_BYTE8,
    ) -> vpi_sys::vpiHandle;
}
