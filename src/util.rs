use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::error::{ModelIoError, Result};
use crate::ffi;

/// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
pub(crate) fn c_string(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| {
        ModelIoError::new(
            ffi::status::INVALID_ARGUMENT,
            "string contained interior NUL",
        )
    })
}

/// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
pub(crate) fn path_to_c_string(path: &Path) -> Result<CString> {
    c_string(path.to_string_lossy().as_ref())
}

/// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
pub(crate) fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    // SAFETY: The unsafe operation is valid in this context.
    let string = unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() };
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { libc::free(ptr.cast::<c_void>()) };
    Some(string)
}

/// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
pub(crate) fn status_result(status: i32, error_ptr: *mut c_char) -> Result<()> {
    if status == ffi::status::OK {
        return Ok(());
    }

    let message =
        take_string(error_ptr).unwrap_or_else(|| format!("ModelIO bridge error {status}"));
    Err(ModelIoError::new(status, message))
}

/// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
pub(crate) fn required_handle(
    ptr: *mut c_void,
    context: &'static str,
) -> Result<crate::handle::ObjectHandle> {
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { crate::handle::ObjectHandle::from_retained_ptr(ptr) }.ok_or_else(|| {
        ModelIoError::new(ffi::status::NULL_RESULT, format!("{context} returned null"))
    })
}

/// Calls the corresponding Model I/O method on the corresponding Model I/O counterpart.
pub(crate) fn parse_json<T: DeserializeOwned>(
    ptr: *mut c_char,
    context: &'static str,
) -> Result<T> {
    let json = take_string(ptr).ok_or_else(|| {
        ModelIoError::new(ffi::status::NULL_RESULT, format!("{context} returned null"))
    })?;
    serde_json::from_str(&json).map_err(|error| {
        ModelIoError::new(
            ffi::status::FRAMEWORK,
            format!("failed to parse {context} JSON: {error}"),
        )
    })
}
