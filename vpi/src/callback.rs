use crate::Handle;

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
    AtEndOfSimTime = vpi_sys::cbAtEndOfSimTime,
}

pub struct Callback {
    pub reason: CbReason,
    pub cb_rtn: Option<unsafe extern "C" fn(*mut vpi_sys::t_cb_data) -> i32>,
    pub obj: Option<crate::Handle>,
    pub time: Option<crate::Time>,
    pub value: Option<crate::Value>,
    pub user_data: Option<*mut i8>,
}

pub struct CbData {
    pub obj: Handle,
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
            user_data: user_data as *mut i8,
        };
        vpi_sys::vpi_register_cb(&mut cb_data)
    };
    Handle::from_raw(handle)
}
