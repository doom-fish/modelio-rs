use std::path::Path;
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::mesh::Mesh;
use crate::types::BoundingBox;
use crate::util::{path_to_c_string, required_handle, take_string};

#[derive(Debug, Clone)]
pub struct Asset {
    handle: ObjectHandle,
}

impl Asset {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
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
    pub fn url(&self) -> Option<String> {
        take_string(unsafe { ffi::mdl_asset_url_string(self.handle.as_ptr()) })
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
