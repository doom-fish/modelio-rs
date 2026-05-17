use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::Matrix4x4ArrayInfo;
use crate::util::{parse_json, required_handle};

#[derive(Debug, Clone)]
pub struct Matrix4x4Array {
    handle: ObjectHandle,
}

impl Matrix4x4Array {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    #[allow(dead_code)]
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    pub fn new(element_count: usize) -> Result<Self> {
        let mut out_array = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_matrix4x4_array_new(element_count as u64, &mut out_array, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_array,
            "MDLMatrix4x4Array",
        )?))
    }

    pub fn info(&self) -> Result<Matrix4x4ArrayInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_matrix4x4_array_info_json(self.handle.as_ptr()) },
            "MDLMatrix4x4Array",
        )
    }

    pub fn clear(&self) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_matrix4x4_array_clear(self.handle.as_ptr()) };
    }

    pub fn set_float_matrices(&self, values: &[[f32; 16]]) {
        let flattened = values
            .iter()
            .flat_map(|value| value.iter().copied())
            .collect::<Vec<_>>();
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_matrix4x4_array_set_float_matrices(
                self.handle.as_ptr(),
                flattened.as_ptr(),
                values.len() as u64,
            );
        };
    }

    pub fn float_matrices(&self) -> Result<Vec<[f32; 16]>> {
        let count = self.info()?.element_count;
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut flattened = vec![0.0_f32; count * 16];
        // SAFETY: The unsafe operation is valid in this context.
        let written = unsafe {
            ffi::mdl_matrix4x4_array_copy_float_matrices(
                self.handle.as_ptr(),
                flattened.as_mut_ptr(),
                count as u64,
            )
        } as usize;
        flattened.truncate(written * 16);
        Ok(flattened
            .chunks_exact(16)
            .map(|chunk| {
                let mut matrix = [0.0_f32; 16];
                matrix.copy_from_slice(chunk);
                matrix
            })
            .collect())
    }
}
