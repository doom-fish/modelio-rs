use std::ptr::NonNull;

use crate::ffi;

#[derive(Debug)]
/// Wraps the corresponding Model I/O object handle counterpart.
pub(crate) struct ObjectHandle(NonNull<core::ffi::c_void>);

impl ObjectHandle {
    /// Calls the corresponding Model I/O method on the wrapped Model I/O object handle counterpart.
    ///
    /// # Safety
    ///
    /// The caller must uphold the same preconditions required by the corresponding Model I/O API.
    pub(crate) unsafe fn from_retained_ptr(ptr: *mut core::ffi::c_void) -> Option<Self> {
        NonNull::new(ptr).map(Self)
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O object handle counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.0.as_ptr()
    }
}

impl Clone for ObjectHandle {
    fn clone(&self) -> Self {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let retained = unsafe { ffi::mdl_object_retain(self.as_ptr()) };
        // SAFETY: The unsafe operation is justified by the surrounding context.
        unsafe { Self::from_retained_ptr(retained) }.expect("ModelIO retain returned null")
    }
}

impl Drop for ObjectHandle {
    fn drop(&mut self) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_object_release(self.as_ptr()) };
    }
}
