use vpi_sys::PLI_INT32;

/// Simulator control operations for `vpi_control`.
#[repr(u32)]
pub enum Control {
    /// Pause simulation execution.
    Stop = vpi_sys::vpiStop,
    /// Terminate simulation.
    Finish = vpi_sys::vpiFinish,
    /// Reset simulator state.
    Reset = vpi_sys::vpiReset,
    /// Set the interactive scope used by the simulator.
    SetInteractiveScope = vpi_sys::vpiSetInteractiveScope,
}

/// Invokes `vpi_control` with the selected operation.
pub fn control(control: Control) {
    unsafe {
        vpi_sys::vpi_control(control as PLI_INT32);
    }
}
