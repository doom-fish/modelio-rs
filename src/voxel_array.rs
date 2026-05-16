use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::mesh::Mesh;
use crate::object::Object;
use crate::types::{BoundingBox, VoxelArrayInfo, VoxelIndexExtent};
use crate::util::{parse_json, required_handle};

#[derive(Debug, Clone)]
pub struct VoxelArray {
    handle: ObjectHandle,
}

impl VoxelArray {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(
        voxel_indices: &[[i32; 4]],
        bounding_box: BoundingBox,
        voxel_extent: f32,
    ) -> Result<Self> {
        let flattened = voxel_indices
            .iter()
            .flat_map(|index| index.iter().copied())
            .collect::<Vec<_>>();
        let mut out_voxel_array = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_voxel_array_new_with_indices(
                flattened.as_ptr(),
                voxel_indices.len() as u64,
                bounding_box.min[0],
                bounding_box.min[1],
                bounding_box.min[2],
                bounding_box.max[0],
                bounding_box.max[1],
                bounding_box.max[2],
                voxel_extent,
                &mut out_voxel_array,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_voxel_array,
            "MDLVoxelArray",
        )?))
    }

    pub fn info(&self) -> Result<VoxelArrayInfo> {
        parse_json(
            unsafe { ffi::mdl_voxel_array_info_json(self.handle.as_ptr()) },
            "MDLVoxelArray",
        )
    }

    #[must_use]
    pub fn count(&self) -> usize {
        unsafe { ffi::mdl_voxel_array_count(self.handle.as_ptr()) as usize }
    }

    pub fn set_voxel(&self, index: [i32; 4]) {
        unsafe {
            ffi::mdl_voxel_array_set_voxel(
                self.handle.as_ptr(),
                index[0],
                index[1],
                index[2],
                index[3],
            );
        };
    }

    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)]
    pub fn voxel_exists(
        &self,
        index: [i32; 4],
        allow_any_x: bool,
        allow_any_y: bool,
        allow_any_z: bool,
        allow_any_shell: bool,
    ) -> bool {
        unsafe {
            ffi::mdl_voxel_array_voxel_exists(
                self.handle.as_ptr(),
                index[0],
                index[1],
                index[2],
                index[3],
                i32::from(allow_any_x),
                i32::from(allow_any_y),
                i32::from(allow_any_z),
                i32::from(allow_any_shell),
            ) != 0
        }
    }

    #[must_use]
    pub fn voxel_indices(&self) -> Vec<[i32; 4]> {
        let count = self.count();
        if count == 0 {
            return Vec::new();
        }
        let mut flattened = vec![0_i32; count * 4];
        let written = unsafe {
            ffi::mdl_voxel_array_copy_indices(
                self.handle.as_ptr(),
                flattened.as_mut_ptr(),
                count as u64,
            )
        } as usize;
        flattened.truncate(written * 4);
        flattened
            .chunks_exact(4)
            .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
            .collect()
    }

    #[must_use]
    pub fn voxels_within_extent(&self, extent: &VoxelIndexExtent) -> Vec<[i32; 4]> {
        let capacity = self.count();
        if capacity == 0 {
            return Vec::new();
        }
        let mut flattened = vec![0_i32; capacity * 4];
        let written = unsafe {
            ffi::mdl_voxel_array_copy_voxels_within_extent(
                self.handle.as_ptr(),
                extent.minimum_extent[0],
                extent.minimum_extent[1],
                extent.minimum_extent[2],
                extent.minimum_extent[3],
                extent.maximum_extent[0],
                extent.maximum_extent[1],
                extent.maximum_extent[2],
                extent.maximum_extent[3],
                flattened.as_mut_ptr(),
                capacity as u64,
            )
        } as usize;
        flattened.truncate(written * 4);
        flattened
            .chunks_exact(4)
            .map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3]])
            .collect()
    }

    pub fn union_with(&self, other: &Self) {
        unsafe { ffi::mdl_voxel_array_union(self.handle.as_ptr(), other.handle.as_ptr()) };
    }

    pub fn intersect_with(&self, other: &Self) {
        unsafe { ffi::mdl_voxel_array_intersect(self.handle.as_ptr(), other.handle.as_ptr()) };
    }

    pub fn difference_with(&self, other: &Self) {
        unsafe { ffi::mdl_voxel_array_difference(self.handle.as_ptr(), other.handle.as_ptr()) };
    }

    #[must_use]
    pub fn index_of_spatial_location(&self, location: [f32; 3]) -> [i32; 4] {
        let mut values = [0_i32; 4];
        unsafe {
            ffi::mdl_voxel_array_index_of_spatial_location(
                self.handle.as_ptr(),
                location[0],
                location[1],
                location[2],
                values.as_mut_ptr(),
            );
        };
        values
    }

    #[must_use]
    pub fn spatial_location_of_index(&self, index: [i32; 4]) -> [f32; 3] {
        let mut values = [0.0_f32; 3];
        unsafe {
            ffi::mdl_voxel_array_spatial_location_of_index(
                self.handle.as_ptr(),
                index[0],
                index[1],
                index[2],
                index[3],
                &mut values[0],
                &mut values[1],
                &mut values[2],
            );
        };
        values
    }

    #[must_use]
    pub fn voxel_bounding_box_at_index(&self, index: [i32; 4]) -> BoundingBox {
        let mut min = [0.0_f32; 3];
        let mut max = [0.0_f32; 3];
        unsafe {
            ffi::mdl_voxel_array_voxel_bounding_box_at_index(
                self.handle.as_ptr(),
                index[0],
                index[1],
                index[2],
                index[3],
                &mut min[0],
                &mut min[1],
                &mut min[2],
                &mut max[0],
                &mut max[1],
                &mut max[2],
            );
        };
        BoundingBox { min, max }
    }

    pub fn convert_to_signed_shell_field(&self) {
        unsafe { ffi::mdl_voxel_array_convert_to_signed_shell_field(self.handle.as_ptr()) };
    }

    pub fn set_shell_field_interior_thickness(&self, value: f32) {
        unsafe {
            ffi::mdl_voxel_array_set_shell_field_interior_thickness(self.handle.as_ptr(), value);
        };
    }

    pub fn set_shell_field_exterior_thickness(&self, value: f32) {
        unsafe {
            ffi::mdl_voxel_array_set_shell_field_exterior_thickness(self.handle.as_ptr(), value);
        };
    }

    #[must_use]
    pub fn coarse_mesh(&self) -> Option<Mesh> {
        let ptr = unsafe { ffi::mdl_voxel_array_coarse_mesh(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Mesh::from_handle)
    }

    #[must_use]
    pub fn mesh(&self) -> Option<Mesh> {
        let ptr = unsafe { ffi::mdl_voxel_array_mesh(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Mesh::from_handle)
    }

    #[must_use]
    pub fn as_object(&self) -> Object {
        Object::from_handle(self.handle.clone())
    }
}
