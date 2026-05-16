use std::ffi::CString;
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::object::Object;
use crate::types::SkeletonInfo;
use crate::util::{c_string, parse_json, required_handle};

fn c_string_vec(values: &[&str]) -> Result<(Vec<CString>, Vec<*const i8>)> {
    let c_strings = values
        .iter()
        .map(|value| c_string(value))
        .collect::<Result<Vec<_>>>()?;
    let raw = c_strings.iter().map(|value| value.as_ptr()).collect();
    Ok((c_strings, raw))
}

fn copy_matrices(
    handle: &ObjectHandle,
    count: usize,
    copier: unsafe extern "C" fn(*mut std::ffi::c_void, *mut f32, u64) -> u64,
) -> Vec<[f32; 16]> {
    if count == 0 {
        return Vec::new();
    }
    let mut flattened = vec![0.0_f32; count * 16];
    let written = unsafe { copier(handle.as_ptr(), flattened.as_mut_ptr(), count as u64) } as usize;
    flattened.truncate(written * 16);
    flattened
        .chunks_exact(16)
        .map(|chunk| {
            let mut matrix = [0.0_f32; 16];
            matrix.copy_from_slice(chunk);
            matrix
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub(crate) handle: ObjectHandle,
}

impl Skeleton {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(name: &str, joint_paths: &[&str]) -> Result<Self> {
        let name = c_string(name)?;
        let (_joint_paths, raw_joint_paths) = c_string_vec(joint_paths)?;
        let mut out_skeleton = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_skeleton_new(
                name.as_ptr(),
                raw_joint_paths.as_ptr(),
                raw_joint_paths.len() as u64,
                &mut out_skeleton,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_skeleton,
            "MDLSkeleton",
        )?))
    }

    pub fn info(&self) -> Result<SkeletonInfo> {
        parse_json(
            unsafe { ffi::mdl_skeleton_info_json(self.handle.as_ptr()) },
            "MDLSkeleton",
        )
    }

    pub fn joint_bind_transforms(&self) -> Result<Vec<[f32; 16]>> {
        let count = self.info()?.joint_bind_transform_count;
        Ok(copy_matrices(
            &self.handle,
            count,
            ffi::mdl_skeleton_copy_joint_bind_transforms,
        ))
    }

    pub fn joint_rest_transforms(&self) -> Result<Vec<[f32; 16]>> {
        let count = self.info()?.joint_rest_transform_count;
        Ok(copy_matrices(
            &self.handle,
            count,
            ffi::mdl_skeleton_copy_joint_rest_transforms,
        ))
    }

    #[must_use]
    pub fn as_object(&self) -> Object {
        Object::from_handle(self.handle.clone())
    }
}
