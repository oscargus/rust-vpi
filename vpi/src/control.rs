use vpi_sys::PLI_INT32;

/// Simulator control operations for `vpi_control`.
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// SystemVerilog coverage control operations for `vpi_control`.
#[cfg(feature = "sv")]
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverageControl {
    /// Start coverage collection.
    Start = vpi_sys::vpiCoverageStart,
    /// Stop coverage collection.
    Stop = vpi_sys::vpiCoverageStOp,
    /// Reset collected coverage data.
    Reset = vpi_sys::vpiCoverageReset,
    /// Perform a coverage consistency/check operation.
    Check = vpi_sys::vpiCoverageCheck,
    /// Merge coverage data.
    Merge = vpi_sys::vpiCoverageMerge,
    /// Save coverage data.
    Save = vpi_sys::vpiCoverageSave,
}

/// SystemVerilog assertion control operations for `vpi_control`.
#[cfg(feature = "sv")]
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssertionControl {
    /// Lock assertion control state.
    Lock = vpi_sys::vpiAssertionLock,
    /// Unlock assertion control state.
    Unlock = vpi_sys::vpiAssertionUnlock,
    /// Disable assertion checking.
    Disable = vpi_sys::vpiAssertionDisable,
    /// Enable assertion checking.
    Enable = vpi_sys::vpiAssertionEnable,
    /// Reset assertion state.
    Reset = vpi_sys::vpiAssertionReset,
    /// Kill active assertion attempts.
    Kill = vpi_sys::vpiAssertionKill,
    /// Enable step mode for assertions.
    EnableStep = vpi_sys::vpiAssertionEnableStep,
    /// Disable step mode for assertions.
    DisableStep = vpi_sys::vpiAssertionDisableStep,
    /// Advance assertion clock steps.
    ClockSteps = vpi_sys::vpiAssertionClockSteps,
    /// Enable assertion system globally.
    SysOn = vpi_sys::vpiAssertionSysOn,
    /// Disable assertion system globally.
    SysOff = vpi_sys::vpiAssertionSysOff,
    /// Kill assertion system activity.
    SysKill = vpi_sys::vpiAssertionSysKill,
    /// End assertion system run.
    SysEnd = vpi_sys::vpiAssertionSysEnd,
    /// Reset assertion system.
    SysReset = vpi_sys::vpiAssertionSysReset,
    /// Disable pass actions.
    DisablePassAction = vpi_sys::vpiAssertionDisablePassAction,
    /// Enable pass actions.
    EnablePassAction = vpi_sys::vpiAssertionEnablePassAction,
    /// Disable fail actions.
    DisableFailAction = vpi_sys::vpiAssertionDisableFailAction,
    /// Enable fail actions.
    EnableFailAction = vpi_sys::vpiAssertionEnableFailAction,
    /// Disable vacuous actions.
    DisableVacuousAction = vpi_sys::vpiAssertionDisableVacuousAction,
    /// Enable non-vacuous actions.
    EnableNonvacuousAction = vpi_sys::vpiAssertionEnableNonvacuousAction,
    /// Enable system pass actions.
    SysEnablePassAction = vpi_sys::vpiAssertionSysEnablePassAction,
    /// Enable system fail actions.
    SysEnableFailAction = vpi_sys::vpiAssertionSysEnableFailAction,
    /// Disable system pass actions.
    SysDisablePassAction = vpi_sys::vpiAssertionSysDisablePassAction,
    /// Disable system fail actions.
    SysDisableFailAction = vpi_sys::vpiAssertionSysDisableFailAction,
    /// Enable system non-vacuous actions.
    SysEnableNonvacuousAction = vpi_sys::vpiAssertionSysEnableNonvacuousAction,
    /// Disable system vacuous actions.
    SysDisableVacuousAction = vpi_sys::vpiAssertionSysDisableVacuousAction,
}

/// Invokes `vpi_control` with the selected operation.
pub fn control(control: Control) {
    unsafe {
        vpi_sys::vpi_control(control as PLI_INT32);
    }
}

#[cfg(feature = "sv")]
fn control_sv(code: PLI_INT32) {
    unsafe {
        vpi_sys::vpi_control(code);
    }
}

/// Invokes `vpi_control` with a SystemVerilog coverage control operation.
#[cfg(feature = "sv")]
pub fn coverage_control(control: CoverageControl) {
    control_sv(control as PLI_INT32);
}

/// Invokes `vpi_control` with a SystemVerilog assertion control operation.
#[cfg(feature = "sv")]
pub fn assertion_control(control: AssertionControl) {
    control_sv(control as PLI_INT32);
}
