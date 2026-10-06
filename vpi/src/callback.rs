use crate::{value::decode_vpi_value, Handle, Time, Value, ValueType};
use num_traits::FromPrimitive;
#[cfg(any(not(feature = "cb_info"), feature = "sv"))]
use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

/// VPI callback reasons used when registering simulator callbacks.
///
/// These values map directly to `vpi_sys::cb*` constants.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, num_derive::FromPrimitive)]
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
    /// Assertion system kill.
    AssertionSysKill = vpi_sys::cbAssertionSysKill,
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

/// Safe callback data passed to Rust closures.
#[derive(Debug)]
pub struct CbData {
    /// Callback reason.
    pub reason: CbReason,
    /// Object handle associated with the callback invocation.
    pub obj: Handle,
    /// Optional callback time payload.
    pub time: Option<Time>,
    /// Optional callback value payload.
    pub value: Option<Value>,
    /// Optional value format.
    pub value_type: Option<ValueType>,
    /// Callback index payload. Meaning depends on callback reason; may be unused for some callbacks.
    pub index: i32,
}

fn time_from_cb_data(raw_time: vpi_sys::s_vpi_time) -> Option<Time> {
    match raw_time.type_ as u32 {
        vpi_sys::vpiSimTime => Some(Time::Sim(
            u64::from(raw_time.high) << 32 | u64::from(raw_time.low),
        )),
        vpi_sys::vpiScaledRealTime => Some(Time::ScaledReal(raw_time.real)),
        vpi_sys::vpiSuppressTime => Some(Time::Suppress),
        _ => None,
    }
}

struct CallbackState {
    callback: Box<dyn Fn(&CbData)>,
    time: Option<Box<vpi_sys::t_vpi_time>>,
    value: Option<Box<vpi_sys::t_vpi_value>>,
}

#[cfg(not(feature = "cb_info"))]
fn callback_state_registry() -> &'static Mutex<HashMap<usize, usize>> {
    static REGISTRY: OnceLock<Mutex<HashMap<usize, usize>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(not(feature = "cb_info"))]
fn register_callback_state(handle: vpi_sys::vpiHandle, state_ptr: *mut CallbackState) {
    if !handle.is_null() {
        callback_state_registry()
            .lock()
            .expect("callback state registry poisoned")
            .insert(handle as usize, state_ptr as usize);
    }
}

#[cfg(not(feature = "cb_info"))]
fn take_callback_state(handle: vpi_sys::vpiHandle) -> Option<*mut CallbackState> {
    callback_state_registry()
        .lock()
        .expect("callback state registry poisoned")
        .remove(&(handle as usize))
        .map(|state_ptr| state_ptr as *mut CallbackState)
}

fn default_cb_time() -> vpi_sys::t_vpi_time {
    vpi_sys::t_vpi_time {
        type_: vpi_sys::vpiSimTime as i32,
        high: 0,
        low: 0,
        real: 0.0,
    }
}

fn default_cb_value() -> vpi_sys::t_vpi_value {
    vpi_sys::t_vpi_value {
        format: vpi_sys::vpiObjTypeVal as i32,
        value: vpi_sys::t_vpi_value__bindgen_ty_1 { integer: 0 },
    }
}

fn cb_value_with_format(value_type: ValueType) -> vpi_sys::t_vpi_value {
    vpi_sys::t_vpi_value {
        format: value_type as i32,
        value: vpi_sys::t_vpi_value__bindgen_ty_1 { integer: 0 },
    }
}

#[cfg(feature = "sv")]
fn assertion_callback_state_registry() -> &'static Mutex<HashMap<usize, usize>> {
    static REGISTRY: OnceLock<Mutex<HashMap<usize, usize>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(feature = "sv")]
fn register_assertion_callback_state(
    handle: vpi_sys::vpiHandle,
    state_ptr: *mut AssertionCallbackState,
) {
    if !handle.is_null() {
        assertion_callback_state_registry()
            .lock()
            .expect("assertion callback state registry poisoned")
            .insert(handle as usize, state_ptr as usize);
    }
}

#[cfg(feature = "sv")]
fn take_assertion_callback_state(
    handle: vpi_sys::vpiHandle,
) -> Option<*mut AssertionCallbackState> {
    assertion_callback_state_registry()
        .lock()
        .expect("assertion callback state registry poisoned")
        .remove(&(handle as usize))
        .map(|state_ptr| state_ptr as *mut AssertionCallbackState)
}

#[cfg(feature = "sv")]
struct AssertionCallbackState {
    callback: Box<dyn Fn(&AssertionCbData)>,
}

/// Safe callback data passed to SystemVerilog assertion callbacks.
#[cfg(feature = "sv")]
#[derive(Debug)]
pub struct AssertionCbData {
    /// Callback reason.
    pub reason: CbReason,
    /// Assertion object associated with the callback.
    pub assertion: Handle,
    /// Optional callback time payload.
    pub time: Option<Time>,
    /// Optional assertion-attempt metadata from `p_vpi_attempt_info`.
    pub attempt_info: Option<AssertionAttemptInfo>,
}

/// Decoded step-transition details for assertion step callbacks.
#[cfg(feature = "sv")]
#[derive(Debug)]
pub struct AssertionStepInfo {
    /// Matched expression handles provided by the simulator.
    pub matched_exprs: Vec<Handle>,
    /// Source state ID for the transition.
    pub state_from: i32,
    /// Destination state ID for the transition.
    pub state_to: i32,
}

/// Decoded union payload from `t_vpi_attempt_info.detail`.
#[cfg(feature = "sv")]
#[derive(Debug)]
pub enum AssertionAttemptDetail {
    /// Failure expression handle.
    FailExpr(Handle),
    /// Step-transition payload.
    Step(AssertionStepInfo),
}

/// Safe wrapper for `t_vpi_attempt_info`.
#[cfg(feature = "sv")]
#[derive(Debug)]
pub struct AssertionAttemptInfo {
    /// Union payload interpreted according to callback reason.
    pub detail: AssertionAttemptDetail,
    /// Attempt start time, when decodable.
    pub attempt_start_time: Option<Time>,
}

#[cfg(feature = "sv")]
fn decode_assertion_attempt_info(
    reason: CbReason,
    info: vpi_sys::p_vpi_attempt_info,
) -> Option<AssertionAttemptInfo> {
    if info.is_null() {
        return None;
    }

    let info_ref = unsafe { &*info };
    let attempt_start_time = time_from_cb_data(info_ref.attemptStartTime);

    let detail = match reason {
        CbReason::AssertionStepSuccess | CbReason::AssertionStepFailure => {
            let step_ptr = unsafe { info_ref.detail.step };
            if step_ptr.is_null() {
                return None;
            }

            let step_ref = unsafe { &*step_ptr };
            let count = if step_ref.matched_expression_count <= 0 {
                0
            } else {
                usize::try_from(step_ref.matched_expression_count).ok()?
            };

            let matched_exprs = if count == 0 || step_ref.matched_exprs.is_null() {
                Vec::new()
            } else {
                let slice = unsafe { std::slice::from_raw_parts(step_ref.matched_exprs, count) };
                slice.iter().copied().map(Handle::from_raw).collect()
            };

            AssertionAttemptDetail::Step(AssertionStepInfo {
                matched_exprs,
                state_from: step_ref.stateFrom,
                state_to: step_ref.stateTo,
            })
        }
        _ => {
            let fail_expr = unsafe { info_ref.detail.failExpr };
            AssertionAttemptDetail::FailExpr(Handle::from_raw(fail_expr))
        }
    };

    Some(AssertionAttemptInfo {
        detail,
        attempt_start_time,
    })
}

#[cfg(feature = "sv")]
unsafe extern "C" fn assertion_trampoline(
    reason: vpi_sys::PLI_INT32,
    cb_time: vpi_sys::p_vpi_time,
    assertion: vpi_sys::vpiHandle,
    info: vpi_sys::p_vpi_attempt_info,
    user_data: *mut vpi_sys::PLI_BYTE8,
) -> vpi_sys::PLI_INT32 {
    if user_data.is_null() {
        return 0;
    }

    let state_ptr = user_data.cast::<AssertionCallbackState>();
    if state_ptr.is_null() {
        return 0;
    }

    let Some(reason) = CbReason::from_u32(reason as u32) else {
        return 0;
    };

    let mut data = AssertionCbData {
        reason,
        assertion: Handle::from_raw(assertion),
        time: if cb_time.is_null() {
            None
        } else {
            time_from_cb_data(unsafe { *cb_time })
        },
        attempt_info: decode_assertion_attempt_info(reason, info),
    };

    let state = unsafe { &*state_ptr };
    (state.callback)(&data);
    data.assertion.clear();
    0
}

fn register_with_state(
    reason: CbReason,
    obj: vpi_sys::vpiHandle,
    state: Box<CallbackState>,
) -> Handle {
    let state_ptr = Box::into_raw(state);
    let state_ref = unsafe { &mut *state_ptr };

    let handle = unsafe {
        let mut cb_data = vpi_sys::s_cb_data {
            reason: reason as i32,
            cb_rtn: Some(trampoline),
            obj,
            time: std::ptr::from_mut(
                state_ref
                    .time
                    .as_mut()
                    .expect("register_with_state requires time storage")
                    .as_mut(),
            ),
            value: std::ptr::from_mut(
                state_ref
                    .value
                    .as_mut()
                    .expect("register_with_state requires value storage")
                    .as_mut(),
            ),
            index: 0,
            user_data: state_ptr.cast::<vpi_sys::PLI_BYTE8>(),
        };
        vpi_sys::vpi_register_cb(&raw mut cb_data)
    };

    if handle.is_null() {
        unsafe {
            let _ = Box::from_raw(state_ptr);
        }
    } else {
        #[cfg(not(feature = "cb_info"))]
        register_callback_state(handle, state_ptr);
    }

    Handle::from_raw(handle)
}

impl Handle {
    /// Registers a callback associated with this handle.
    ///
    /// Returns a callback handle that can be removed with [`remove_cb`].
    pub fn register_cb<F>(&self, reason: CbReason, callback: F) -> Handle
    where
        F: Fn(&CbData) + 'static,
    {
        let user_data = Box::into_raw(Box::new(CallbackState {
            callback: Box::new(callback),
            time: None,
            value: None,
        }));

        let handle = unsafe {
            let mut cb_data = vpi_sys::s_cb_data {
                reason: reason as i32,
                cb_rtn: Some(trampoline),
                obj: self.as_raw(),
                time: std::ptr::null_mut(),
                value: std::ptr::null_mut(),
                index: 0,
                user_data: user_data.cast::<vpi_sys::PLI_BYTE8>(),
            };
            vpi_sys::vpi_register_cb(&raw mut cb_data)
        };

        if handle.is_null() {
            unsafe {
                let _ = Box::from_raw(user_data);
            }
        } else {
            #[cfg(not(feature = "cb_info"))]
            register_callback_state(handle, user_data.cast::<CallbackState>());
        }

        Handle::from_raw(handle)
    }

    /// Registers a callback with persistent time/value registration buffers.
    ///
    /// This variant populates `t_cb_data.time` and `t_cb_data.value` at
    /// registration time so simulators can write callback payloads through
    /// those pointers.
    pub fn register_full_cb<F>(&self, reason: CbReason, callback: F) -> Handle
    where
        F: Fn(&CbData) + 'static,
    {
        let state = Box::new(CallbackState {
            callback: Box::new(callback),
            time: Some(Box::new(default_cb_time())),
            value: Some(Box::new(default_cb_value())),
        });
        register_with_state(reason, self.as_raw(), state)
    }

    /// Registers a value-change callback with an explicit value format.
    ///
    /// Some simulators require `t_cb_data.value.format` to match the expected
    /// callback value encoding for `cbValueChange` callbacks. This helper sets
    /// that format during registration.
    pub fn register_value_change_cb<F>(&self, value_type: ValueType, callback: F) -> Handle
    where
        F: Fn(&CbData) + 'static,
    {
        let state = Box::new(CallbackState {
            callback: Box::new(callback),
            time: Some(Box::new(default_cb_time())),
            value: Some(Box::new(cb_value_with_format(value_type))),
        });
        register_with_state(CbReason::ValueChange, self.as_raw(), state)
    }
}

unsafe extern "C" fn trampoline(cb_data: *mut vpi_sys::t_cb_data) -> i32 {
    if cb_data.is_null() {
        return 0; // No callback data, just return
    }

    let user_data = unsafe { (*cb_data).user_data.cast::<CallbackState>() };
    if user_data.is_null() {
        return 0; // No user data, just return
    }

    let cb_data_ref = unsafe { &*cb_data };
    let value = if cb_data_ref.value.is_null() {
        None
    } else {
        Some(unsafe { *cb_data_ref.value })
    };

    let mut data = CbData {
        reason: CbReason::from_u32(cb_data_ref.reason as u32)
            .expect("received unknown callback reason from simulator"),
        obj: Handle::from_raw(cb_data_ref.obj),
        time: if cb_data_ref.time.is_null() {
            None
        } else {
            time_from_cb_data(unsafe { *cb_data_ref.time })
        },
        value: value.and_then(|raw| decode_vpi_value(raw, cb_data_ref.obj)),
        value_type: value.and_then(|raw| ValueType::from_u32(raw.format as u32)),
        index: cb_data_ref.index,
    };

    let state = unsafe { &*user_data };
    (state.callback)(&data);

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
    let user_data = Box::into_raw(Box::new(CallbackState {
        callback: Box::new(callback),
        time: None,
        value: None,
    }));

    let handle = unsafe {
        let mut cb_data = vpi_sys::s_cb_data {
            reason: reason as i32,
            cb_rtn: Some(trampoline),
            obj: std::ptr::null_mut(),
            time: std::ptr::null_mut(),
            value: std::ptr::null_mut(),
            index: 0,
            user_data: user_data.cast::<vpi_sys::PLI_BYTE8>(),
        };
        vpi_sys::vpi_register_cb(&raw mut cb_data)
    };

    if handle.is_null() {
        unsafe {
            let _ = Box::from_raw(user_data);
        }
    } else {
        #[cfg(not(feature = "cb_info"))]
        register_callback_state(handle, user_data.cast::<CallbackState>());
    }

    Handle::from_raw(handle)
}

/// Registers a global callback with persistent time/value registration buffers.
///
/// This variant populates `t_cb_data.time` and `t_cb_data.value` at
/// registration time so simulators can write callback payloads through
/// those pointers.
pub fn register_full_cb<F>(reason: CbReason, callback: F) -> Handle
where
    F: Fn(&CbData) + 'static,
{
    let state = Box::new(CallbackState {
        callback: Box::new(callback),
        time: Some(Box::new(default_cb_time())),
        value: Some(Box::new(default_cb_value())),
    });
    register_with_state(reason, std::ptr::null_mut(), state)
}

/// Registers a time-based callback.
///
/// The callback is scheduled according to `reason` and `time` as interpreted
/// by the simulator.
pub fn register_cb_with_time<F>(reason: CbReason, time: Time, callback: F) -> Handle
where
    F: Fn(&CbData) + 'static,
{
    let state = Box::new(CallbackState {
        callback: Box::new(callback),
        time: Some(Box::new(time.into())),
        value: Some(Box::new(default_cb_value())),
    });
    register_with_state(reason, std::ptr::null_mut(), state)
}

/// Registers a SystemVerilog assertion callback.
///
/// Available only with the `sv` feature.
#[cfg(feature = "sv")]
pub fn register_assertion_cb<F>(assertion: &Handle, reason: CbReason, callback: F) -> Handle
where
    F: Fn(&AssertionCbData) + 'static,
{
    let state_ptr = Box::into_raw(Box::new(AssertionCallbackState {
        callback: Box::new(callback),
    }));

    let handle = unsafe {
        vpi_sys::vpi_register_assertion_cb(
            assertion.as_raw(),
            reason as i32,
            Some(assertion_trampoline),
            state_ptr.cast::<vpi_sys::PLI_BYTE8>(),
        )
    };

    if handle.is_null() {
        unsafe {
            let _ = Box::from_raw(state_ptr);
        }
    } else {
        register_assertion_callback_state(handle, state_ptr);
    }

    Handle::from_raw(handle)
}

/// Removes a previously registered SystemVerilog assertion callback.
///
/// Available only with the `sv` feature.
#[cfg(feature = "sv")]
pub fn remove_assertion_cb(handle: &Handle) {
    if handle.is_null() {
        return;
    }

    let state_ptr = take_assertion_callback_state(handle.as_raw());
    unsafe {
        vpi_sys::vpi_remove_cb(handle.as_raw());
        if let Some(state_ptr) = state_ptr {
            let _ = Box::from_raw(state_ptr);
        }
    }
}

/// Removes a previously registered callback.
///
/// If `handle` is null, this is a no-op.
#[cfg(feature = "cb_info")]
pub fn remove_cb(handle: &Handle) {
    if !handle.is_null() {
        unsafe {
            let mut cb_data = vpi_sys::s_cb_data {
                reason: 0,
                cb_rtn: None,
                obj: std::ptr::null_mut(),
                time: std::ptr::null_mut(),
                value: std::ptr::null_mut(),
                index: 0,
                user_data: std::ptr::null_mut(),
            };
            vpi_sys::vpi_get_cb_info(handle.as_raw(), &raw mut cb_data);
            vpi_sys::vpi_remove_cb(handle.as_raw());
            let trampoline_ptr = trampoline as unsafe extern "C" fn(*mut vpi_sys::t_cb_data) -> i32;
            let is_internal = cb_data
                .cb_rtn
                .is_some_and(|cb| (cb as usize) == (trampoline_ptr as usize));
            if is_internal && !cb_data.user_data.is_null() {
                let _ = Box::from_raw(cb_data.user_data.cast::<CallbackState>());
            }
        }
    }
}

/// Removes a previously registered callback.
///
/// This version does not call `vpi_get_cb_info` and instead uses the
/// callback state captured at registration time.
#[cfg(not(feature = "cb_info"))]
pub fn remove_cb(handle: &Handle) {
    if handle.is_null() {
        return;
    }

    let state_ptr = take_callback_state(handle.as_raw());
    unsafe {
        vpi_sys::vpi_remove_cb(handle.as_raw());
        if let Some(state_ptr) = state_ptr {
            let _ = Box::from_raw(state_ptr);
        }
    }
}
