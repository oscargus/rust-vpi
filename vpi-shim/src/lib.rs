//! Dynamic VPI symbol shim for plugin builds on Windows and macOS.
//!
//! On these platforms, some linkers require all external symbols to be resolved
//! when building the plugin binary. This crate exports a subset of common VPI
//! symbols and resolves the real implementations at runtime.

#![allow(non_snake_case)]

#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::ffi::c_void;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::sync::OnceLock;

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn missing_symbol(name: &str) -> ! {
    eprintln!(
        "vpi-shim: failed to resolve symbol '{name}'. On Windows, set VPI_SHIM_LIB to the simulator DLL path if auto-discovery fails."
    );
    std::process::abort();
}

#[cfg(target_os = "macos")]
unsafe fn resolve_symbol(name: &[u8]) -> *mut c_void {
    let ptr = unsafe { libc::dlsym(libc::RTLD_NEXT, name.as_ptr().cast()) };
    if !ptr.is_null() {
        return ptr;
    }
    unsafe { libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr().cast()) }
}

#[cfg(target_os = "windows")]
unsafe fn resolve_symbol(name: &[u8]) -> *mut c_void {
    use libloading::os::windows::Library;

    fn candidate_libraries() -> Vec<String> {
        let mut names = Vec::new();

        if let Ok(path) = std::env::var("VPI_SHIM_LIB") {
            for p in path.split(';').map(str::trim).filter(|s| !s.is_empty()) {
                names.push(p.to_string());
            }
        }

        // Common simulator/runtime library names. Users can override via VPI_SHIM_LIB.
        names.extend([
            "vpi.dll".to_string(),
            "iverilog-vpi.dll".to_string(),
            "modelsim.dll".to_string(),
            "questa.dll".to_string(),
        ]);

        names
    }

    static LIBRARIES: OnceLock<Vec<Library>> = OnceLock::new();

    let libraries = LIBRARIES.get_or_init(|| {
        let mut libs = Vec::new();
        for name in candidate_libraries() {
            if let Ok(lib) = unsafe { Library::new(&name) } {
                libs.push(lib);
            }
        }
        libs
    });

    for lib in libraries {
        if let Ok(sym) = unsafe { lib.get::<*mut c_void>(name) } {
            let ptr = *sym;
            if !ptr.is_null() {
                return ptr;
            }
        }
    }

    std::ptr::null_mut()
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
    fn vpi_remove_cb(cb_obj: vpi_sys::vpiHandle) -> vpi_sys::PLI_INT32;
    fn vpi_handle_by_name(name: *mut vpi_sys::PLI_BYTE8, scope: vpi_sys::vpiHandle) -> vpi_sys::vpiHandle;
    fn vpi_handle_by_index(object: vpi_sys::vpiHandle, indx: vpi_sys::PLI_INT32) -> vpi_sys::vpiHandle;
    fn vpi_iterate(type_: vpi_sys::PLI_INT32, refHandle: vpi_sys::vpiHandle) -> vpi_sys::vpiHandle;
    fn vpi_scan(iterator: vpi_sys::vpiHandle) -> vpi_sys::vpiHandle;
    fn vpi_get(property: vpi_sys::PLI_INT32, object: vpi_sys::vpiHandle) -> vpi_sys::PLI_INT32;
    fn vpi_get_str(property: vpi_sys::PLI_INT32, object: vpi_sys::vpiHandle) -> *mut vpi_sys::PLI_BYTE8;
    fn vpi_get_vlog_info(vlog_info_p: vpi_sys::p_vpi_vlog_info) -> vpi_sys::PLI_INT32;
    fn vpi_control(operation: vpi_sys::PLI_INT32) -> vpi_sys::PLI_INT32;
    fn vpi_compare_objects(object1: vpi_sys::vpiHandle, object2: vpi_sys::vpiHandle) -> vpi_sys::PLI_INT32;
    fn vpi_chk_error(error_info_p: vpi_sys::p_vpi_error_info) -> vpi_sys::PLI_INT32;
    fn vpi_release_handle(object: vpi_sys::vpiHandle) -> vpi_sys::PLI_INT32;
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
    fn vpi_get_time(object: vpi_sys::vpiHandle, time_p: vpi_sys::p_vpi_time);
    fn vpi_get_value(expr: vpi_sys::vpiHandle, value_p: vpi_sys::p_vpi_value);
    fn vpi_get_value_array(
        expr: vpi_sys::vpiHandle,
        arrayvalue_p: vpi_sys::p_vpi_arrayvalue,
        index_p: *mut vpi_sys::PLI_INT32,
        num: vpi_sys::PLI_UINT32,
    );
}
