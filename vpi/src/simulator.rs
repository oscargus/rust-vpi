pub fn simulator_info() -> SimulatorInfo {
    let mut vlog_info = vpi_sys::t_vpi_vlog_info {
        argc: 0,
        argv: std::ptr::null_mut(),
        version: std::ptr::null_mut(),
        product: std::ptr::null_mut(),
    };
    unsafe { vpi_sys::vpi_get_vlog_info(&mut vlog_info) };
    let version = unsafe { std::ffi::CStr::from_ptr(vlog_info.version) }
        .to_str()
        .unwrap_or("Unknown")
        .to_string();
    let product = unsafe { std::ffi::CStr::from_ptr(vlog_info.product) }
        .to_str()
        .unwrap_or("Unknown")
        .to_string();
    let mut arguments = Vec::new();
    for i in 0..vlog_info.argc {
        let arg_ptr = unsafe { *vlog_info.argv.add(i as usize) };
        let arg = unsafe { std::ffi::CStr::from_ptr(arg_ptr) }
            .to_str()
            .unwrap_or("Unknown")
            .to_string();
        arguments.push(arg);
    }
    SimulatorInfo {
        arguments,
        version,
        product,
    }
}

#[derive(Debug)]
pub struct SimulatorInfo {
    pub arguments: Vec<String>,
    pub version: String,
    pub product: String,
}

#[must_use]
pub fn simulator_name() -> String {
    let mut vlog_info = vpi_sys::t_vpi_vlog_info {
        argc: 0,
        argv: std::ptr::null_mut(),
        version: std::ptr::null_mut(),
        product: std::ptr::null_mut(),
    };
    unsafe { vpi_sys::vpi_get_vlog_info(&mut vlog_info) };
    unsafe { std::ffi::CStr::from_ptr(vlog_info.product) }
        .to_str()
        .unwrap_or("Unknown")
        .to_string()
}

#[must_use]
pub fn simulator_version() -> String {
    let mut vlog_info = vpi_sys::t_vpi_vlog_info {
        argc: 0,
        argv: std::ptr::null_mut(),
        version: std::ptr::null_mut(),
        product: std::ptr::null_mut(),
    };
    unsafe { vpi_sys::vpi_get_vlog_info(&mut vlog_info) };
    unsafe { std::ffi::CStr::from_ptr(vlog_info.version) }
        .to_str()
        .unwrap_or("Unknown")
        .to_string()
}
