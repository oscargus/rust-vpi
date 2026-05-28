use crate::{Handle, Time};

/// VPI callback reasons used when registering simulator callbacks.
///
/// These values map directly to `vpi_sys::cb*` constants.
#[repr(u32)]
pub enum CbReason {
    /// Callback on value change.
    ValueChange = vpi_sys::cbValueChange,
    /// Callback on statement execution.
    Stmt = vpi_sys::cbStmt,
    /// Callback on force.
    Force = vpi_sys::cbForce,
    /// Callback on release.
    Release = vpi_sys::cbRelease,
    /// Callback at the start of the current simulation time.
    AtStartOfSimTime = vpi_sys::cbAtStartOfSimTime,
    /// Callback during the read-write synchronization phase.
    ReadWriteSynch = vpi_sys::cbReadWriteSynch,
    /// Callback during the read-only synchronization phase.
    ReadOnlySynch = vpi_sys::cbReadOnlySynch,
    /// Callback at the next simulation time.
    NextSimTime = vpi_sys::cbNextSimTime,
    /// Callback after a delay.
    AfterDelay = vpi_sys::cbAfterDelay,
    /// Callback at the end of compilation.
    EndOfCompile = vpi_sys::cbEndOfCompile,
    /// Callback at the start of simulation.
    StartOfSimulation = vpi_sys::cbStartOfSimulation,
    /// Callback at the end of simulation.
    EndOfSimulation = vpi_sys::cbEndOfSimulation,
    /// Callback on error.
    Error = vpi_sys::cbError,
    /// Callback on timing-check violation.
    TchkViolation = vpi_sys::cbTchkViolation,
    /// Callback at the start of save.
    StartOfSave = vpi_sys::cbStartOfSave,
    /// Callback at the end of save.
    EndOfSave = vpi_sys::cbEndOfSave,
    /// Callback at the start of restart.
    StartOfRestart = vpi_sys::cbStartOfRestart,
    /// Callback at the end of restart.
    EndOfRestart = vpi_sys::cbEndOfRestart,
    /// Callback at the start of reset.
    StartOfReset = vpi_sys::cbStartOfReset,
    /// Callback at the end of reset.
    EndOfReset = vpi_sys::cbEndOfReset,
    /// Callback on entry to interactive mode.
    EnterInteractive = vpi_sys::cbEnterInteractive,
    /// Callback on exit from interactive mode.
    ExitInteractive = vpi_sys::cbExitInteractive,
    /// Callback when the interactive scope changes.
    InteractiveScopeChange = vpi_sys::cbInteractiveScopeChange,
    /// Callback on unresolved system task or function lookup.
    UnresolvedSystf = vpi_sys::cbUnresolvedSystf,
    /// Callback on PLI error.
    PLIError = vpi_sys::cbPLIError,
    /// Callback on assignment.
    Assign = vpi_sys::cbAssign,
    /// Callback on deassignment.
    Deassign = vpi_sys::cbDeassign,
    /// Callback on disable.
    Disable = vpi_sys::cbDisable,
    /// Callback on signal delivery.
    Signal = vpi_sys::cbSignal,
    /// Callback during the NBA synchronization phase.
    NBASynch = vpi_sys::cbNBASynch,
    /// Callback at the end of the current simulation time.
    AtEndOfSimTime = vpi_sys::cbAtEndOfSimTime,

    #[cfg(feature = "sv")]
    // SystemVerilog thread callbacks (600-605)
    /// Callback on thread creation.
    StartOfThread = vpi_sys::cbStartOfThread,
    #[cfg(feature = "sv")]
    /// Callback on thread termination.
    EndOfThread = vpi_sys::cbEndOfThread,
    #[cfg(feature = "sv")]
    /// Callback on thread reentry.
    EnterThread = vpi_sys::cbEnterThread,
    #[cfg(feature = "sv")]
    /// Callback on frame creation.
    StartOfFrame = vpi_sys::cbStartOfFrame,
    #[cfg(feature = "sv")]
    /// Callback on frame exit.
    EndOfFrame = vpi_sys::cbEndOfFrame,
    #[cfg(feature = "sv")]
    /// Callback on array variable size change.
    SizeChange = vpi_sys::cbSizeChange,

    #[cfg(feature = "sv")]
    // SystemVerilog assertion callbacks (606-662)
    /// Assertion start.
    AssertionStart = vpi_sys::cbAssertionStart,
    #[cfg(feature = "sv")]
    /// Assertion success.
    AssertionSuccess = vpi_sys::cbAssertionSuccess,
    #[cfg(feature = "sv")]
    /// Assertion failure.
    AssertionFailure = vpi_sys::cbAssertionFailure,
    #[cfg(feature = "sv")]
    /// Assertion step success.
    AssertionStepSuccess = vpi_sys::cbAssertionStepSuccess,
    #[cfg(feature = "sv")]
    /// Assertion step failure.
    AssertionStepFailure = vpi_sys::cbAssertionStepFailure,
    #[cfg(feature = "sv")]
    /// Assertion disable.
    AssertionDisable = vpi_sys::cbAssertionDisable,
    #[cfg(feature = "sv")]
    /// Assertion enable.
    AssertionEnable = vpi_sys::cbAssertionEnable,
    #[cfg(feature = "sv")]
    /// Assertion reset.
    AssertionReset = vpi_sys::cbAssertionReset,
    #[cfg(feature = "sv")]
    /// Assertion kill.
    AssertionKill = vpi_sys::cbAssertionKill,
    #[cfg(feature = "sv")]
    /// Assertion system initialization.
    AssertionSysInitialized = vpi_sys::cbAssertionSysInitialized,
    #[cfg(feature = "sv")]
    /// Assertion system on.
    AssertionSysOn = vpi_sys::cbAssertionSysOn,
    #[cfg(feature = "sv")]
    /// Assertion system off.
    AssertionSysOff = vpi_sys::cbAssertionSysOff,
    #[cfg(feature = "sv")]
    /// Assertion system end.
    AssertionSysEnd = vpi_sys::cbAssertionSysEnd,
    #[cfg(feature = "sv")]
    /// Assertion system reset.
    AssertionSysReset = vpi_sys::cbAssertionSysReset,
    #[cfg(feature = "sv")]
    /// Assertion vacuous success.
    AssertionVacuousSuccess = vpi_sys::cbAssertionVacuousSuccess,
    #[cfg(feature = "sv")]
    /// Assertion disabled evaluation.
    AssertionDisabledEvaluation = vpi_sys::cbAssertionDisabledEvaluation,
    #[cfg(feature = "sv")]
    /// Assertion system lock.
    AssertionSysLock = vpi_sys::cbAssertionSysLock,
    #[cfg(feature = "sv")]
    /// Assertion system unlock.
    AssertionSysUnlock = vpi_sys::cbAssertionSysUnlock,
    #[cfg(feature = "sv")]
    /// Assertion lock.
    AssertionLock = vpi_sys::cbAssertionLock,
    #[cfg(feature = "sv")]
    /// Assertion unlock.
    AssertionUnlock = vpi_sys::cbAssertionUnlock,
    #[cfg(feature = "sv")]
    /// Assertion enable pass action.
    AssertionEnablePassAction = vpi_sys::cbAssertionEnablePassAction,
    #[cfg(feature = "sv")]
    /// Assertion enable fail action.
    AssertionEnableFailAction = vpi_sys::cbAssertionEnableFailAction,
    #[cfg(feature = "sv")]
    /// Assertion disable pass action.
    AssertionDisablePassAction = vpi_sys::cbAssertionDisablePassAction,
    #[cfg(feature = "sv")]
    /// Assertion disable fail action.
    AssertionDisableFailAction = vpi_sys::cbAssertionDisableFailAction,
    #[cfg(feature = "sv")]
    /// Assertion enable non-vacuous action.
    AssertionEnableNonvacuousAction = vpi_sys::cbAssertionEnableNonvacuousAction,
    #[cfg(feature = "sv")]
    /// Assertion disable vacuous action.
    AssertionDisableVacuousAction = vpi_sys::cbAssertionDisableVacuousAction,
    #[cfg(feature = "sv")]
    /// Assertion system enable pass action.
    AssertionSysEnablePassAction = vpi_sys::cbAssertionSysEnablePassAction,
    #[cfg(feature = "sv")]
    /// Assertion system enable fail action.
    AssertionSysEnableFailAction = vpi_sys::cbAssertionSysEnableFailAction,
    #[cfg(feature = "sv")]
    /// Assertion system disable pass action.
    AssertionSysDisablePassAction = vpi_sys::cbAssertionSysDisablePassAction,
    #[cfg(feature = "sv")]
    /// Assertion system disable fail action.
    AssertionSysDisableFailAction = vpi_sys::cbAssertionSysDisableFailAction,
    #[cfg(feature = "sv")]
    /// Assertion system enable non-vacuous action.
    AssertionSysEnableNonvacuousAction = vpi_sys::cbAssertionSysEnableNonvacuousAction,
    #[cfg(feature = "sv")]
    /// Assertion system disable vacuous action.
    AssertionSysDisableVacuousAction = vpi_sys::cbAssertionSysDisableVacuousAction,

    #[cfg(feature = "sv")]
    // SystemVerilog object callbacks (700-702)
    /// Callback on class object creation.
    CreateObj = vpi_sys::cbCreateObj,
    #[cfg(feature = "sv")]
    /// Callback on class object reclamation.
    ReclaimObj = vpi_sys::cbReclaimObj,
    #[cfg(feature = "sv")]
    /// Callback on transient object deletion.
    EndOfObject = vpi_sys::cbEndOfObject,
}

/// Raw callback registration descriptor.
///
/// This mirrors the fields used by `vpi_register_cb` and is useful when
/// constructing callback registrations manually.
pub struct Callback {
    /// Event reason that triggers the callback.
    pub reason: CbReason,
    /// Native callback function pointer.
    pub cb_rtn: Option<unsafe extern "C" fn(*mut vpi_sys::t_cb_data) -> i32>,
    /// Optional object associated with the callback.
    pub obj: Option<crate::Handle>,
    /// Optional simulation time for time-based callbacks.
    pub time: Option<crate::Time>,
    /// Optional value payload used by value-based callbacks.
    pub value: Option<crate::Value>,
    /// Optional user data pointer passed back by the simulator.
    pub user_data: Option<*mut i8>,
}

/// Safe callback data passed to Rust closures.
pub struct CbData {
    /// Object handle associated with the callback invocation.
    pub obj: Handle,
}

impl Handle {
    /// Registers a callback associated with this handle.
    ///
    /// Returns a callback handle that can be removed with [`remove_cb`].
    pub fn register_cb<F>(&self, reason: CbReason, callback: F) -> Handle
    where
        F: Fn(&CbData) + 'static,
    {
        let boxed: Box<F> = Box::new(callback);
        let user_data = Box::into_raw(boxed).cast::<std::os::raw::c_void>();

        let handle = unsafe {
            let mut cb_data = vpi_sys::s_cb_data {
                reason: reason as i32,
                cb_rtn: Some(trampoline::<F>),
                obj: self.as_raw(),
                time: std::ptr::null_mut(),
                value: std::ptr::null_mut(),
                index: 0,
                user_data: user_data.cast::<i8>(),
            };
            vpi_sys::vpi_register_cb(&raw mut cb_data)
        };
        Handle::from_raw(handle)
    }
}

unsafe extern "C" fn trampoline<F>(cb_data: *mut vpi_sys::t_cb_data) -> i32
where
    F: Fn(&CbData),
{
    if cb_data.is_null() {
        return 0; // No callback data, just return
    }

    let user_data = unsafe { (*cb_data).user_data.cast::<F>() };
    if user_data.is_null() {
        return 0; // No user data, just return
    }

    let mut data = CbData {
        obj: Handle::from_raw(unsafe { (*cb_data).obj }),
    };

    let callback = unsafe { &*user_data };
    callback(&data);

    data.obj.clear(); // We do not own this handle
    0 // Return 0 to indicate success
}

/// Registers a global callback not tied to a specific object handle.
///
/// Returns a callback handle that can be removed with [`remove_cb`].
pub fn register_cb<F>(reason: CbReason, callback: F) -> Handle
where
    F: Fn(&CbData) + 'static,
{
    let boxed: Box<F> = Box::new(callback);
    let user_data = Box::into_raw(boxed).cast::<std::os::raw::c_void>();

    let handle = unsafe {
        let mut cb_data = vpi_sys::s_cb_data {
            reason: reason as i32,
            cb_rtn: Some(trampoline::<F>),
            obj: std::ptr::null_mut(),
            time: std::ptr::null_mut(),
            value: std::ptr::null_mut(),
            index: 0,
            user_data: user_data.cast::<i8>(),
        };
        vpi_sys::vpi_register_cb(&raw mut cb_data)
    };
    Handle::from_raw(handle)
}

/// Registers a time-based callback.
///
/// The callback is scheduled according to `reason` and `time` as interpreted
/// by the simulator.
pub fn register_cb_with_time<F>(reason: CbReason, time: Time, callback: F) -> Handle
where
    F: Fn(&CbData) + 'static,
{
    let boxed: Box<F> = Box::new(callback);
    let user_data = Box::into_raw(boxed).cast::<i8>();

    let handle = unsafe {
        let mut cb_data = vpi_sys::s_cb_data {
            reason: reason as i32,
            cb_rtn: Some(trampoline::<F>),
            obj: std::ptr::null_mut(),
            time: std::ptr::from_mut(&mut time.into()),
            value: std::ptr::null_mut(),
            index: 0,
            user_data,
        };
        vpi_sys::vpi_register_cb(&raw mut cb_data)
    };
    Handle::from_raw(handle)
}

/// Removes a previously registered callback.
///
/// If `handle` is null, this is a no-op.
pub fn remove_cb(handle: &Handle) {
    if !handle.is_null() {
        unsafe {
            vpi_sys::vpi_remove_cb(handle.as_raw());
            // SAFETY: We assume the callback was registered with a Box, so we can safely reconstruct it and drop it
            let _ = Box::from_raw(handle.as_raw().cast::<i8>());
        }
    }
}
