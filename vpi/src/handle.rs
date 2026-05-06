use crate::ObjectType;
use vpi_sys::{vpiHandle, PLI_INT32};

/// Wrapper around a raw VPI object handle.
///
/// This type provides convenience helpers for common handle operations and
/// iteration over child objects.
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
    /// Creates a null handle.
    #[must_use]
    pub fn null() -> Self {
        Self {
            handle: std::ptr::null_mut(),
        }
    }

    /// Returns `true` if this handle is null.
    #[must_use]
    pub fn is_null(&self) -> bool {
        self.handle.is_null()
    }

    /// Returns the underlying raw VPI handle.
    #[must_use]
    pub fn as_raw(&self) -> vpiHandle {
        self.handle
    }

    /// Replaces the current handle with null.
    ///
    /// This is used when ownership of the raw handle is not held by this type.
    pub fn clear(&mut self) {
        self.handle = std::ptr::null_mut();
    }

    /// Constructs a [`Handle`] from a raw VPI handle pointer.
    pub fn from_raw(raw: vpiHandle) -> Self {
        Self { handle: raw }
    }

    /// Looks up a handle by hierarchical name.
    ///
    /// Returns a null handle when the object is not found.
    #[must_use]
    pub fn handle_by_name(name: &str) -> Self {
        let c_name = std::ffi::CString::new(name).expect("CString::new failed");
        let handle = unsafe {
            vpi_sys::vpi_handle_by_name(c_name.as_ptr().cast_mut(), std::ptr::null_mut())
        };
        Self::from_raw(handle)
    }

    /// Returns an iterator handle for objects of `typ` under this handle.
    #[must_use]
    pub fn iterator(&self, typ: ObjectType) -> HandleIterator {
        let raw = unsafe { vpi_sys::vpi_iterate(typ as PLI_INT32, self.as_raw()) };
        HandleIterator {
            iter: Handle::from_raw(raw),
        }
    }

    /// Returns a child handle by index.
    ///
    /// Returns a null handle when `index` is out of range.
    #[must_use]
    pub fn handle_by_index(&self, index: i32) -> Self {
        let handle = unsafe { vpi_sys::vpi_handle_by_index(self.as_raw(), index) };
        Self::from_raw(handle)
    }

    /// Iterates across multiple object kinds and flattens all resulting handles.
    pub fn iterators<'a>(&'a self, typ: &'a [ObjectType]) -> impl Iterator<Item = Handle> + 'a {
        typ.iter().copied().flat_map(move |t| self.iterator(t))
    }
}

/// Iterator over VPI scan results from `vpi_iterate`/`vpi_scan`.
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
