#![allow(clippy::missing_const_for_fn)]
#![cfg(all(
    test,
    any(
        not(any(target_os = "windows", target_os = "macos")),
        not(feature = "dynamic")
    )
))]

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_register_cb(_cb_data_p: vpi_sys::p_cb_data) -> vpi_sys::vpiHandle {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_register_systf(
    _systf_data_p: vpi_sys::p_vpi_systf_data,
) -> vpi_sys::vpiHandle {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_remove_cb(_cb_obj: vpi_sys::vpiHandle) -> vpi_sys::PLI_INT32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_handle(
    _type: vpi_sys::PLI_INT32,
    _ref_handle: vpi_sys::vpiHandle,
) -> vpi_sys::vpiHandle {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_handle_multi(
    _type: vpi_sys::PLI_INT32,
    _ref_handle1: vpi_sys::vpiHandle,
    _ref_handle2: vpi_sys::vpiHandle,
) -> vpi_sys::vpiHandle {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_handle_by_name(
    _name: *mut vpi_sys::PLI_BYTE8,
    _scope: vpi_sys::vpiHandle,
) -> vpi_sys::vpiHandle {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_handle_by_index(
    _object: vpi_sys::vpiHandle,
    _indx: vpi_sys::PLI_INT32,
) -> vpi_sys::vpiHandle {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_handle_by_multi_index(
    _obj: vpi_sys::vpiHandle,
    _num_index: vpi_sys::PLI_INT32,
    _index_array: *mut vpi_sys::PLI_INT32,
) -> vpi_sys::vpiHandle {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_iterate(
    _type: vpi_sys::PLI_INT32,
    _ref_handle: vpi_sys::vpiHandle,
) -> vpi_sys::vpiHandle {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_scan(_iterator: vpi_sys::vpiHandle) -> vpi_sys::vpiHandle {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_get(
    _property: vpi_sys::PLI_INT32,
    _object: vpi_sys::vpiHandle,
) -> vpi_sys::PLI_INT32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_get64(
    _property: vpi_sys::PLI_INT32,
    _object: vpi_sys::vpiHandle,
) -> vpi_sys::PLI_INT64 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_get_str(
    _property: vpi_sys::PLI_INT32,
    _object: vpi_sys::vpiHandle,
) -> *mut vpi_sys::PLI_BYTE8 {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_get_vlog_info(
    _vlog_info_p: vpi_sys::p_vpi_vlog_info,
) -> vpi_sys::PLI_INT32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_control(_operation: vpi_sys::PLI_INT32) -> vpi_sys::PLI_INT32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_compare_objects(
    object1: vpi_sys::vpiHandle,
    object2: vpi_sys::vpiHandle,
) -> vpi_sys::PLI_INT32 {
    i32::from(object1 == object2)
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_chk_error(_error_info_p: vpi_sys::p_vpi_error_info) -> vpi_sys::PLI_INT32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_release_handle(_object: vpi_sys::vpiHandle) -> vpi_sys::PLI_INT32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_flush() -> vpi_sys::PLI_INT32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_put_value(
    _object: vpi_sys::vpiHandle,
    _value_p: vpi_sys::p_vpi_value,
    _time_p: vpi_sys::p_vpi_time,
    _flags: vpi_sys::PLI_INT32,
) -> vpi_sys::vpiHandle {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_get_cb_info(_object: vpi_sys::vpiHandle, _cb_data_p: vpi_sys::p_cb_data) {}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_get_systf_info(
    _object: vpi_sys::vpiHandle,
    _systf_data_p: vpi_sys::p_vpi_systf_data,
) {
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_get_delays(_object: vpi_sys::vpiHandle, _delay_p: vpi_sys::p_vpi_delay) {}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_put_delays(_object: vpi_sys::vpiHandle, _delay_p: vpi_sys::p_vpi_delay) {}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_get_time(_object: vpi_sys::vpiHandle, _time_p: vpi_sys::p_vpi_time) {}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_get_value(_expr: vpi_sys::vpiHandle, _value_p: vpi_sys::p_vpi_value) {}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_get_value_array(
    _expr: vpi_sys::vpiHandle,
    _arrayvalue_p: vpi_sys::p_vpi_arrayvalue,
    _index_p: *mut vpi_sys::PLI_INT32,
    _num: vpi_sys::PLI_UINT32,
) {
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_put_value_array(
    _object: vpi_sys::vpiHandle,
    _arrayvalue_p: vpi_sys::p_vpi_arrayvalue,
    _index_p: *mut vpi_sys::PLI_INT32,
    _num: vpi_sys::PLI_UINT32,
) {
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_mcd_open(_file_name: *mut vpi_sys::PLI_BYTE8) -> vpi_sys::PLI_UINT32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_mcd_close(_mcd: vpi_sys::PLI_UINT32) -> vpi_sys::PLI_UINT32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_mcd_name(_cd: vpi_sys::PLI_UINT32) -> *mut vpi_sys::PLI_BYTE8 {
    std::ptr::null_mut()
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_mcd_flush(_mcd: vpi_sys::PLI_UINT32) -> vpi_sys::PLI_INT32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_printf(
    _format: *mut vpi_sys::PLI_BYTE8,
    _arg: *mut vpi_sys::PLI_BYTE8,
) -> vpi_sys::PLI_INT32 {
    0
}

#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_mcd_printf(
    _mcd: vpi_sys::PLI_UINT32,
    _format: *mut vpi_sys::PLI_BYTE8,
    _arg: *mut vpi_sys::PLI_BYTE8,
) -> vpi_sys::PLI_INT32 {
    0
}

#[cfg(feature = "sv")]
#[unsafe(no_mangle)]
unsafe extern "C" fn vpi_register_assertion_cb(
    _assertion: vpi_sys::vpiHandle,
    _reason: vpi_sys::PLI_INT32,
    _cb_rtn: vpi_sys::vpi_assertion_callback_func,
    _user_data: *mut vpi_sys::PLI_BYTE8,
) -> vpi_sys::vpiHandle {
    std::ptr::null_mut()
}
