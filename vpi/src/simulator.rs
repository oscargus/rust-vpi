#[must_use]
pub fn simulator_info() -> SimulatorInfo {
    let mut vlog_info = vpi_sys::t_vpi_vlog_info {
        argc: 0,
        argv: std::ptr::null_mut(),
        version: std::ptr::null_mut(),
        product: std::ptr::null_mut(),
    };
    unsafe { vpi_sys::vpi_get_vlog_info(&raw mut vlog_info) };
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
    unsafe { vpi_sys::vpi_get_vlog_info(&raw mut vlog_info) };
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
    unsafe { vpi_sys::vpi_get_vlog_info(&raw mut vlog_info) };
    unsafe { std::ffi::CStr::from_ptr(vlog_info.version) }
        .to_str()
        .unwrap_or("Unknown")
        .to_string()
}

/// Represents a module's timescale information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timescale {
    /// Time unit as a power of 10 (e.g., -9 for 1ns, -12 for 1ps)
    pub unit: i32,
    /// Time precision as a power of 10 (e.g., -12 for 1ps)
    pub precision: i32,
}

impl Timescale {
    /// Get the timescale for a given module handle
    ///
    /// # Safety
    /// The handle must be a valid VPI module handle
    pub unsafe fn from_module(module_handle: vpi_sys::vpiHandle) -> Option<Self> {
        // SAFETY: Caller guarantees module_handle is valid
        let unit = unsafe { vpi_sys::vpi_get(crate::Property::TimeUnit as i32, module_handle) };
        let precision =
            unsafe { vpi_sys::vpi_get(crate::Property::TimePrecision as i32, module_handle) };
        Some(Timescale { unit, precision })
    }

    /// Convert time unit/precision to a human-readable string
    /// E.g., -9 => "1ns", -12 => "1ps"
    #[must_use]
    pub fn unit_str(&self) -> String {
        power_of_10_to_time_str(self.unit)
    }

    /// Convert time precision to a human-readable string
    #[must_use]
    pub fn precision_str(&self) -> String {
        power_of_10_to_time_str(self.precision)
    }
}

impl std::fmt::Display for Timescale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / {}", self.unit_str(), self.precision_str())
    }
}

/// Convert a power of 10 to a time unit string
fn power_of_10_to_time_str(power: i32) -> String {
    match power {
        2 => "100s".to_string(),
        1 => "10s".to_string(),
        0 => "1s".to_string(),
        -1 => "100ms".to_string(),
        -2 => "10ms".to_string(),
        -3 => "1ms".to_string(),
        -6 => "1us".to_string(),
        -9 => "1ns".to_string(),
        -12 => "1ps".to_string(),
        -15 => "1fs".to_string(),
        _ => format!("10^{power}s"),
    }
}

/// Get timescale for the top-level modules
///
/// Returns a vector of (`module_name`, timescale) tuples for all top-level modules
#[must_use]
pub fn get_top_module_timescales() -> Vec<(String, Option<Timescale>)> {
    let mut results = Vec::new();

    unsafe {
        // Iterate over all top-level modules
        let iter = vpi_sys::vpi_iterate(vpi_sys::vpiModule as i32, std::ptr::null_mut());
        if iter.is_null() {
            return results;
        }

        loop {
            let module = vpi_sys::vpi_scan(iter);
            if module.is_null() {
                break;
            }

            // Get module name
            let name_ptr = vpi_sys::vpi_get_str(crate::Property::Name as i32, module);
            let name = if name_ptr.is_null() {
                "Unknown".to_string()
            } else {
                std::ffi::CStr::from_ptr(name_ptr)
                    .to_str()
                    .unwrap_or("Unknown")
                    .to_string()
            };

            // Get timescale
            let timescale = Timescale::from_module(module);

            results.push((name, timescale));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::{power_of_10_to_time_str, Timescale};

    #[test]
    fn maps_known_power_values_to_expected_units() {
        assert_eq!(power_of_10_to_time_str(2), "100s");
        assert_eq!(power_of_10_to_time_str(1), "10s");
        assert_eq!(power_of_10_to_time_str(0), "1s");
        assert_eq!(power_of_10_to_time_str(-1), "100ms");
        assert_eq!(power_of_10_to_time_str(-2), "10ms");
        assert_eq!(power_of_10_to_time_str(-3), "1ms");
        assert_eq!(power_of_10_to_time_str(-6), "1us");
        assert_eq!(power_of_10_to_time_str(-9), "1ns");
        assert_eq!(power_of_10_to_time_str(-12), "1ps");
        assert_eq!(power_of_10_to_time_str(-15), "1fs");
    }

    #[test]
    fn maps_unknown_power_values_to_fallback_format() {
        assert_eq!(power_of_10_to_time_str(3), "10^3s");
        assert_eq!(power_of_10_to_time_str(-4), "10^-4s");
        assert_eq!(power_of_10_to_time_str(-10), "10^-10s");
    }

    #[test]
    fn timescale_unit_and_precision_helpers_use_power_mapping() {
        let timescale = Timescale {
            unit: -9,
            precision: -12,
        };

        assert_eq!(timescale.unit_str(), "1ns");
        assert_eq!(timescale.precision_str(), "1ps");
    }

    #[test]
    fn timescale_display_formats_as_unit_slash_precision() {
        let timescale = Timescale {
            unit: -6,
            precision: -15,
        };

        assert_eq!(timescale.to_string(), "1us / 1fs");
    }
}
