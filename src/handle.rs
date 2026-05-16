use std::ptr::NonNull;

use crate::ffi;

#[derive(Debug)]
pub(crate) struct ObjectHandle(NonNull<core::ffi::c_void>);

impl ObjectHandle {
    pub(crate) unsafe fn from_retained_ptr(ptr: *mut core::ffi::c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.0.as_ptr()
    }
}

impl Clone for ObjectHandle {
    fn clone(&self) -> Self {
        let retained = unsafe { ffi::mdl_object_retain(self.as_ptr()) };
        unsafe { Self::from_retained_ptr(retained) }.expect("ModelIO retain returned null")
    }
}

impl Drop for ObjectHandle {
    fn drop(&mut self) {
        unsafe { ffi::mdl_object_release(self.as_ptr()) };
    }
}
