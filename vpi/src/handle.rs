use vpi_sys::vpiHandle;

pub struct Handle {
    handle: vpiHandle,
}

impl Default for Handle {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle {
    pub fn null() -> Self {
        Self {
            handle: std::ptr::null_mut(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.handle.is_null()
    }

    pub fn as_raw(&self) -> vpiHandle {
        self.handle
    }

    pub fn clear(&mut self) {
        self.handle = std::ptr::null_mut();
    }

    pub fn from_raw(raw: vpiHandle) -> Self {
        Self { handle: raw }
    }
}
