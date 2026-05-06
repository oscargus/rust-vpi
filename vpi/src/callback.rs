use crate::{Handle, Time};

/// VPI callback reasons used when registering simulator callbacks.
///
/// These values map directly to `vpi_sys::cb*` constants.
#[repr(u32)]
pub enum CbReason {
    ValueChange = vpi_sys::cbValueChange,
    Stmt = vpi_sys::cbStmt,
    Force = vpi_sys::cbForce,
    Release = vpi_sys::cbRelease,
    AtStartOfSimTime = vpi_sys::cbAtStartOfSimTime,
    ReadWriteSynch = vpi_sys::cbReadWriteSynch,
    ReadOnlySynch = vpi_sys::cbReadOnlySynch,
    NextSimTime = vpi_sys::cbNextSimTime,
    AfterDelay = vpi_sys::cbAfterDelay,
    EndOfCompile = vpi_sys::cbEndOfCompile,
    StartOfSimulation = vpi_sys::cbStartOfSimulation,
    EndOfSimulation = vpi_sys::cbEndOfSimulation,
    Error = vpi_sys::cbError,
    TchkViolation = vpi_sys::cbTchkViolation,
    StartOfSave = vpi_sys::cbStartOfSave,
    EndOfSave = vpi_sys::cbEndOfSave,
    StartOfRestart = vpi_sys::cbStartOfRestart,
    EndOfRestart = vpi_sys::cbEndOfRestart,
    StartOfReset = vpi_sys::cbStartOfReset,
    EndOfReset = vpi_sys::cbEndOfReset,
    EnterInteractive = vpi_sys::cbEnterInteractive,
    ExitInteractive = vpi_sys::cbExitInteractive,
    InteractiveScopeChange = vpi_sys::cbInteractiveScopeChange,
    UnresolvedSystf = vpi_sys::cbUnresolvedSystf,
    PLIError = vpi_sys::cbPLIError,
    Assign = vpi_sys::cbAssign,
    Deassign = vpi_sys::cbDeassign,
    Disable = vpi_sys::cbDisable,
    Signal = vpi_sys::cbSignal,
    NBASynch = vpi_sys::cbNBASynch,
    AtEndOfSimTime = vpi_sys::cbAtEndOfSimTime,

    #[cfg(feature = "sv")]
    // SystemVerilog thread callbacks (600-605)
    /// callback on thread creation
    StartOfThread = vpi_sys::cbStartOfThread,
    #[cfg(feature = "sv")]
    /// callback on thread termination
    EndOfThread = vpi_sys::cbEndOfThread,
    #[cfg(feature = "sv")]
    /// callback on reentering thread
    EnterThread = vpi_sys::cbEnterThread,
    #[cfg(feature = "sv")]
    /// callback on frame creation
    StartOfFrame = vpi_sys::cbStartOfFrame,
    #[cfg(feature = "sv")]
    /// callback on frame exit
    EndOfFrame = vpi_sys::cbEndOfFrame,
    #[cfg(feature = "sv")]
    /// callback on array variable size change
    SizeChange = vpi_sys::cbSizeChange,

    #[cfg(feature = "sv")]
    // SystemVerilog assertion callbacks (606-662)
    /// assertion starts
    AssertionStart = vpi_sys::cbAssertionStart,
    #[cfg(feature = "sv")]
    /// assertion succeeds
    AssertionSuccess = vpi_sys::cbAssertionSuccess,
    #[cfg(feature = "sv")]
    /// assertion fails
    AssertionFailure = vpi_sys::cbAssertionFailure,
    #[cfg(feature = "sv")]
    /// assertion step succeeds
    AssertionStepSuccess = vpi_sys::cbAssertionStepSuccess,
    #[cfg(feature = "sv")]
    /// assertion step fails
    AssertionStepFailure = vpi_sys::cbAssertionStepFailure,
    #[cfg(feature = "sv")]
    /// assertion disabled
    AssertionDisable = vpi_sys::cbAssertionDisable,
    #[cfg(feature = "sv")]
    /// assertion enabled
    AssertionEnable = vpi_sys::cbAssertionEnable,
    #[cfg(feature = "sv")]
    /// assertion reset
    AssertionReset = vpi_sys::cbAssertionReset,
    #[cfg(feature = "sv")]
    /// assertion killed
    AssertionKill = vpi_sys::cbAssertionKill,
    #[cfg(feature = "sv")]
    /// assertion system initialized
    AssertionSysInitialized = vpi_sys::cbAssertionSysInitialized,
    #[cfg(feature = "sv")]
    /// assertion system on
    AssertionSysOn = vpi_sys::cbAssertionSysOn,
    #[cfg(feature = "sv")]
    /// assertion system off
    AssertionSysOff = vpi_sys::cbAssertionSysOff,
    #[cfg(feature = "sv")]
    /// assertion system end
    AssertionSysEnd = vpi_sys::cbAssertionSysEnd,
    #[cfg(feature = "sv")]
    /// assertion system reset
    AssertionSysReset = vpi_sys::cbAssertionSysReset,
    #[cfg(feature = "sv")]
    /// assertion vacuous success
    AssertionVacuousSuccess = vpi_sys::cbAssertionVacuousSuccess,
    #[cfg(feature = "sv")]
    /// assertion disabled evaluation
    AssertionDisabledEvaluation = vpi_sys::cbAssertionDisabledEvaluation,
    #[cfg(feature = "sv")]
    /// assertion system lock
    AssertionSysLock = vpi_sys::cbAssertionSysLock,
    #[cfg(feature = "sv")]
    /// assertion system unlock
    AssertionSysUnlock = vpi_sys::cbAssertionSysUnlock,
    #[cfg(feature = "sv")]
    /// assertion lock
    AssertionLock = vpi_sys::cbAssertionLock,
    #[cfg(feature = "sv")]
    /// assertion unlock
    AssertionUnlock = vpi_sys::cbAssertionUnlock,
    #[cfg(feature = "sv")]
    /// assertion enable pass action
    AssertionEnablePassAction = vpi_sys::cbAssertionEnablePassAction,
    #[cfg(feature = "sv")]
    /// assertion enable fail action
    AssertionEnableFailAction = vpi_sys::cbAssertionEnableFailAction,
    #[cfg(feature = "sv")]
    /// assertion disable pass action
    AssertionDisablePassAction = vpi_sys::cbAssertionDisablePassAction,
    #[cfg(feature = "sv")]
    /// assertion disable fail action
    AssertionDisableFailAction = vpi_sys::cbAssertionDisableFailAction,
    #[cfg(feature = "sv")]
    /// assertion enable non-vacuous action
    AssertionEnableNonvacuousAction = vpi_sys::cbAssertionEnableNonvacuousAction,
    #[cfg(feature = "sv")]
    /// assertion disable vacuous action
    AssertionDisableVacuousAction = vpi_sys::cbAssertionDisableVacuousAction,
    #[cfg(feature = "sv")]
    /// assertion system enable pass action
    AssertionSysEnablePassAction = vpi_sys::cbAssertionSysEnablePassAction,
    #[cfg(feature = "sv")]
    /// assertion system enable fail action
    AssertionSysEnableFailAction = vpi_sys::cbAssertionSysEnableFailAction,
    #[cfg(feature = "sv")]
    /// assertion system disable pass action
    AssertionSysDisablePassAction = vpi_sys::cbAssertionSysDisablePassAction,
    #[cfg(feature = "sv")]
    /// assertion system disable fail action
    AssertionSysDisableFailAction = vpi_sys::cbAssertionSysDisableFailAction,
    #[cfg(feature = "sv")]
    /// assertion system enable non-vacuous action
    AssertionSysEnableNonvacuousAction = vpi_sys::cbAssertionSysEnableNonvacuousAction,
    #[cfg(feature = "sv")]
    /// assertion system disable vacuous action
    AssertionSysDisableVacuousAction = vpi_sys::cbAssertionSysDisableVacuousAction,

    #[cfg(feature = "sv")]
    // SystemVerilog object callbacks (700-702)
    /// callback on class object creation
    CreateObj = vpi_sys::cbCreateObj,
    #[cfg(feature = "sv")]
    /// callback on class object reclaimed
    ReclaimObj = vpi_sys::cbReclaimObj,
    #[cfg(feature = "sv")]
    /// callback on transient object deletion
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
