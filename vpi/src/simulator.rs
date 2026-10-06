use crate::Time;

/// Failure reported while retrieving simulator invocation metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulatorInfoError {
    /// `vpi_get_vlog_info` reported that the information is unavailable.
    Unavailable,
    /// The simulator did not provide a product name.
    MissingProduct,
    /// The simulator did not provide a version string.
    MissingVersion,
    /// The simulator returned a negative argument count.
    InvalidArgumentCount,
    /// The simulator reported arguments but returned a null argument array.
    MissingArguments,
    /// An argument entry in the simulator-provided array was null.
    MissingArgument(usize),
}

impl std::fmt::Display for SimulatorInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable => write!(f, "simulator invocation information is unavailable"),
            Self::MissingProduct => write!(f, "simulator product name is missing"),
            Self::MissingVersion => write!(f, "simulator version is missing"),
            Self::InvalidArgumentCount => write!(f, "simulator returned a negative argument count"),
            Self::MissingArguments => write!(f, "simulator argument array is missing"),
            Self::MissingArgument(index) => {
                write!(f, "simulator argument {index} is missing")
            }
        }
    }
}

impl std::error::Error for SimulatorInfoError {}

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

/// Returns simulator invocation metadata from `vpi_get_vlog_info`.
///
/// This includes the simulator product string, version string, and command-line
/// arguments as reported by the active VPI implementation.
#[must_use]
pub fn simulator_info() -> SimulatorInfo {
    try_simulator_info().expect("could not retrieve simulator invocation information")
}

/// Fallible version of [`simulator_info`].
///
/// Checks the return status and required pointers supplied by
/// `vpi_get_vlog_info`.
pub fn try_simulator_info() -> Result<SimulatorInfo, SimulatorInfoError> {
    let mut vlog_info = vpi_sys::t_vpi_vlog_info {
        argc: 0,
        argv: std::ptr::null_mut(),
        version: std::ptr::null_mut(),
        product: std::ptr::null_mut(),
    };
    if unsafe { vpi_sys::vpi_get_vlog_info(&raw mut vlog_info) } == 0 {
        return Err(SimulatorInfoError::Unavailable);
    }
    let version = copy_c_string(vlog_info.version).ok_or(SimulatorInfoError::MissingVersion)?;
    let product = copy_c_string(vlog_info.product).ok_or(SimulatorInfoError::MissingProduct)?;
    if vlog_info.argc < 0 {
        return Err(SimulatorInfoError::InvalidArgumentCount);
    }
    if vlog_info.argc > 0 && vlog_info.argv.is_null() {
        return Err(SimulatorInfoError::MissingArguments);
    }
    let mut arguments = Vec::new();
    for i in 0..vlog_info.argc as usize {
        let arg_ptr = unsafe { *vlog_info.argv.add(i) };
        arguments.push(copy_c_string(arg_ptr).ok_or(SimulatorInfoError::MissingArgument(i))?);
    }
    Ok(SimulatorInfo {
        arguments,
        version,
        product,
    })
}

/// Simulator metadata reported by `vpi_get_vlog_info`.
#[derive(Debug)]
pub struct SimulatorInfo {
    /// Command-line arguments used to start the simulator.
    pub arguments: Vec<String>,
    /// Simulator version string.
    pub version: String,
    /// Simulator product name.
    pub product: String,
}

/// Returns the simulator product name.
#[must_use]
pub fn simulator_name() -> String {
    try_simulator_name().expect("could not retrieve simulator invocation information")
}

/// Fallible version of [`simulator_name`].
pub fn try_simulator_name() -> Result<String, SimulatorInfoError> {
    Ok(try_simulator_info()?.product)
}

/// Returns the simulator version string.
#[must_use]
pub fn simulator_version() -> String {
    try_simulator_version().expect("could not retrieve simulator invocation information")
}

/// Fallible version of [`simulator_version`].
pub fn try_simulator_version() -> Result<String, SimulatorInfoError> {
    Ok(try_simulator_info()?.version)
}

/// Returns the current simulation time.
///
/// This uses `vpi_get_time` with a null handle, which requests the simulator's
/// current global simulation time.
#[must_use]
pub fn current_simulation_time() -> Time {
    let mut vpi_time = vpi_sys::s_vpi_time {
        type_: vpi_sys::vpiSimTime as i32,
        high: 0,
        low: 0,
        real: 0.0,
    };
    unsafe { vpi_sys::vpi_get_time(std::ptr::null_mut(), &raw mut vpi_time) };
    Time::from(vpi_time)
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
    unsafe fn from_module(module_handle: vpi_sys::vpiHandle) -> Self {
        // SAFETY: Caller guarantees module_handle is valid
        let unit = unsafe {
            vpi_sys::vpi_get(
                crate::Property::TimeUnit as vpi_sys::PLI_INT32,
                module_handle,
            )
        };
        let precision = unsafe {
            vpi_sys::vpi_get(
                crate::Property::TimePrecision as vpi_sys::PLI_INT32,
                module_handle,
            )
        };
        Timescale { unit, precision }
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
        -4 => "100us".to_string(),
        -5 => "10us".to_string(),
        -6 => "1us".to_string(),
        -7 => "100ns".to_string(),
        -8 => "10ns".to_string(),
        -9 => "1ns".to_string(),
        -10 => "100ps".to_string(),
        -11 => "10ps".to_string(),
        -12 => "1ps".to_string(),
        -13 => "100fs".to_string(),
        -14 => "10fs".to_string(),
        -15 => "1fs".to_string(),
        _ => format!("10^{power}s"),
    }
}

/// Returns timescale information for top-level modules.
///
/// Each entry contains a module name and its effective time unit/precision.
#[must_use]
pub fn get_top_module_timescales() -> Vec<(String, Timescale)> {
    let mut results = Vec::new();

    let iter = crate::Handle::null().iterator(crate::ObjectType::Module);
    for module in iter {
        let name_ptr =
            unsafe { vpi_sys::vpi_get_str(crate::Property::Name as i32, module.as_raw()) };
        let name = if name_ptr.is_null() {
            "Unknown".to_string()
        } else {
            unsafe { std::ffi::CStr::from_ptr(name_ptr) }
                .to_str()
                .unwrap_or("Unknown")
                .to_string()
        };

        // SAFETY: The iterator yields a valid module handle for this iteration.
        let timescale = unsafe { Timescale::from_module(module.as_raw()) };
        results.push((name, timescale));
    }

    results
}

/// Returns the simulator's time precision as an integer power of 10.
pub fn get_simulator_precision() -> i32 {
    unsafe {
        vpi_sys::vpi_get(
            crate::Property::TimePrecision as vpi_sys::PLI_INT32,
            crate::Handle::null().as_raw(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        power_of_10_to_time_str, try_simulator_info, try_simulator_name, try_simulator_version,
        SimulatorInfoError, Timescale,
    };

    #[cfg(not(all(feature = "dynamic", any(target_os = "windows", target_os = "macos"))))]
    #[test]
    fn try_simulator_info_reports_unavailable_vpi_info() {
        assert!(matches!(
            try_simulator_info(),
            Err(SimulatorInfoError::Unavailable)
        ));
        assert!(matches!(
            try_simulator_name(),
            Err(SimulatorInfoError::Unavailable)
        ));
        assert!(matches!(
            try_simulator_version(),
            Err(SimulatorInfoError::Unavailable)
        ));
    }

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
