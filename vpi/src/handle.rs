use crate::ObjectType;
use vpi_sys::vpiHandle;

pub struct Handle {
    handle: vpiHandle,
}

impl Default for Handle {
    fn default() -> Self {
        Self::null()
    }
}

impl PartialEq for Handle {
    fn eq(&self, other: &Self) -> bool {
        unsafe { vpi_sys::vpi_compare_objects(self.handle, other.handle) != 0 }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        /*   if !self.is_null() {
            unsafe {
                vpi_sys::vpi_release_handle(self.handle);
            }
        }*/
    }
}

impl Handle {
    #[must_use]
    pub fn null() -> Self {
        Self {
            handle: std::ptr::null_mut(),
        }
    }

    #[must_use]
    pub fn is_null(&self) -> bool {
        self.handle.is_null()
    }

    #[must_use]
    pub fn as_raw(&self) -> vpiHandle {
        self.handle
    }

    pub fn clear(&mut self) {
        self.handle = std::ptr::null_mut();
    }

    pub fn from_raw(raw: vpiHandle) -> Self {
        Self { handle: raw }
    }

    pub fn handle_by_name(name: &str) -> Self {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        let handle = unsafe {
            vpi_sys::vpi_handle_by_name(c_name.as_ptr() as *mut i8, std::ptr::null_mut())
        };
        Self::from_raw(handle)
    }

    #[must_use]
    pub fn iterator(&self, typ: ObjectType) -> HandleIterator {
        let raw = unsafe { vpi_sys::vpi_iterate(typ as i32, self.as_raw()) };
        HandleIterator {
            iter: Handle::from_raw(raw),
        }
    }
}

pub struct HandleIterator {
    pub(crate) iter: Handle,
}

impl Iterator for HandleIterator {
    type Item = Handle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_null() {
            return None;
        }

        let next = Handle::from_raw(unsafe { vpi_sys::vpi_scan(self.iter.as_raw()) });

        if next.is_null() {
            // The handle is automatically released when the iterator is exhausted
            self.iter.clear();
            None
        } else {
            Some(next)
        }
    }
}
