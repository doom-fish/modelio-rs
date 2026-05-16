use std::path::Path;
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::mesh::Mesh;
use crate::object::Object;
use crate::types::{AssetInfo, BoundingBox};
use crate::util::{c_string, parse_json, path_to_c_string, required_handle, take_string};

#[derive(Debug, Clone)]
pub struct Asset {
    handle: ObjectHandle,
}

impl Asset {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    pub fn new() -> Result<Self> {
        let mut out_asset = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::mdl_asset_new_empty(&mut out_asset, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(out_asset, "MDLAsset")?))
    }

    pub fn from_url(path: impl AsRef<Path>) -> Result<Self> {
        let path = path_to_c_string(path.as_ref())?;
        let mut out_asset = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status =
            unsafe { ffi::mdl_asset_new_with_url(path.as_ptr(), &mut out_asset, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(out_asset, "MDLAsset")?))
    }

    #[must_use]
    pub fn can_import_file_extension(path_extension: &str) -> bool {
        let trimmed = path_extension.trim_start_matches('.');
        crate::util::c_string(trimmed)
            .is_ok_and(|ext| unsafe { ffi::mdl_asset_can_import_file_extension(ext.as_ptr()) != 0 })
    }

    #[must_use]
    pub fn can_export_file_extension(path_extension: &str) -> bool {
        let trimmed = path_extension.trim_start_matches('.');
        crate::util::c_string(trimmed)
            .is_ok_and(|ext| unsafe { ffi::mdl_asset_can_export_file_extension(ext.as_ptr()) != 0 })
    }

    pub fn info(&self) -> Result<AssetInfo> {
        parse_json(
            unsafe { ffi::mdl_asset_info_json(self.handle.as_ptr()) },
            "MDLAsset",
        )
    }

    pub fn export_to_url(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path_to_c_string(path.as_ref())?;
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_asset_export_to_url(self.handle.as_ptr(), path.as_ptr(), &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    #[must_use]
    pub fn count(&self) -> usize {
        unsafe { ffi::mdl_asset_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn bounding_box(&self) -> BoundingBox {
        let mut min = [0.0_f32; 3];
        let mut max = [0.0_f32; 3];
        unsafe {
            ffi::mdl_asset_bounding_box(
                self.handle.as_ptr(),
                &mut min[0],
                &mut min[1],
                &mut min[2],
                &mut max[0],
                &mut max[1],
                &mut max[2],
            );
        }
        BoundingBox { min, max }
    }

    #[must_use]
    pub fn bounding_box_at_time(&self, time: f64) -> BoundingBox {
        let mut min = [0.0_f32; 3];
        let mut max = [0.0_f32; 3];
        unsafe {
            ffi::mdl_asset_bounding_box_at_time(
                self.handle.as_ptr(),
                time,
                &mut min[0],
                &mut min[1],
                &mut min[2],
                &mut max[0],
                &mut max[1],
                &mut max[2],
            );
        }
        BoundingBox { min, max }
    }

    #[must_use]
    pub fn url(&self) -> Option<String> {
        take_string(unsafe { ffi::mdl_asset_url_string(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn frame_interval(&self) -> f64 {
        unsafe { ffi::mdl_asset_frame_interval(self.handle.as_ptr()) }
    }

    pub fn set_frame_interval(&self, value: f64) {
        unsafe { ffi::mdl_asset_set_frame_interval(self.handle.as_ptr(), value) };
    }

    #[must_use]
    pub fn start_time(&self) -> f64 {
        unsafe { ffi::mdl_asset_start_time(self.handle.as_ptr()) }
    }

    pub fn set_start_time(&self, value: f64) {
        unsafe { ffi::mdl_asset_set_start_time(self.handle.as_ptr(), value) };
    }

    #[must_use]
    pub fn end_time(&self) -> f64 {
        unsafe { ffi::mdl_asset_end_time(self.handle.as_ptr()) }
    }

    pub fn set_end_time(&self, value: f64) {
        unsafe { ffi::mdl_asset_set_end_time(self.handle.as_ptr(), value) };
    }

    #[must_use]
    pub fn up_axis(&self) -> [f32; 3] {
        let mut axis = [0.0_f32; 3];
        unsafe {
            ffi::mdl_asset_up_axis(
                self.handle.as_ptr(),
                &mut axis[0],
                &mut axis[1],
                &mut axis[2],
            );
        };
        axis
    }

    pub fn set_up_axis(&self, axis: [f32; 3]) {
        unsafe { ffi::mdl_asset_set_up_axis(self.handle.as_ptr(), axis[0], axis[1], axis[2]) };
    }

    #[must_use]
    pub fn object_at(&self, index: usize) -> Option<Object> {
        let ptr = unsafe { ffi::mdl_asset_object_at_index(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Object::from_handle)
    }

    pub fn object_at_path(&self, path: &str) -> Result<Option<Object>> {
        let path = c_string(path)?;
        let ptr = unsafe { ffi::mdl_asset_object_at_path(self.handle.as_ptr(), path.as_ptr()) };
        Ok(unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Object::from_handle))
    }

    pub fn add_object(&self, object: &Object) {
        unsafe { ffi::mdl_asset_add_object(self.handle.as_ptr(), object.as_ptr()) };
    }

    pub fn remove_object(&self, object: &Object) {
        unsafe { ffi::mdl_asset_remove_object(self.handle.as_ptr(), object.as_ptr()) };
    }

    #[must_use]
    pub fn mesh_at(&self, index: usize) -> Option<Mesh> {
        let ptr = unsafe { ffi::mdl_asset_mesh_at_index(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Mesh::from_handle)
    }

    #[must_use]
    pub fn meshes(&self) -> Vec<Mesh> {
        (0..self.count())
            .filter_map(|index| self.mesh_at(index))
            .collect()
    }

    pub fn load_textures(&self) {
        unsafe { ffi::mdl_asset_load_textures(self.handle.as_ptr()) };
    }
}
