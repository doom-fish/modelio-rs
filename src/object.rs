use std::ptr;

use crate::camera::Camera;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::light::Light;
use crate::mesh::Mesh;
use crate::physically_plausible_light::PhysicallyPlausibleLight;
use crate::skeleton::Skeleton;
use crate::types::{BoundingBox, ObjectInfo, ObjectKind};
use crate::util::{c_string, parse_json, required_handle, take_string};
use crate::voxel_array::VoxelArray;
use crate::PackedJointAnimation;

#[derive(Debug, Clone)]
pub struct Object {
    handle: ObjectHandle,
}

impl Object {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    pub fn new() -> Result<Self> {
        let mut out_object = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::mdl_object_new(&mut out_object, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(out_object, "MDLObject")?))
    }

    pub fn info(&self) -> Result<ObjectInfo> {
        parse_json(
            unsafe { ffi::mdl_object_info_json(self.handle.as_ptr()) },
            "MDLObject",
        )
    }

    #[must_use]
    pub fn kind(&self) -> ObjectKind {
        ObjectKind::from_raw(unsafe { ffi::mdl_object_kind(self.handle.as_ptr()) })
            .unwrap_or(ObjectKind::Unknown)
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        take_string(unsafe { ffi::mdl_object_name_string(self.handle.as_ptr()) })
    }

    pub fn set_name(&self, name: &str) -> Result<()> {
        let name = c_string(name)?;
        unsafe { ffi::mdl_object_set_name(self.handle.as_ptr(), name.as_ptr()) };
        Ok(())
    }

    #[must_use]
    pub fn path(&self) -> Option<String> {
        take_string(unsafe { ffi::mdl_object_path_string(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn hidden(&self) -> bool {
        unsafe { ffi::mdl_object_hidden(self.handle.as_ptr()) != 0 }
    }

    pub fn set_hidden(&self, hidden: bool) {
        unsafe { ffi::mdl_object_set_hidden(self.handle.as_ptr(), i32::from(hidden)) };
    }

    pub fn add_child(&self, child: &Self) {
        unsafe { ffi::mdl_object_add_child(self.handle.as_ptr(), child.handle.as_ptr()) };
    }

    #[must_use]
    pub fn child_count(&self) -> usize {
        unsafe { ffi::mdl_object_child_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn child_at(&self, index: usize) -> Option<Self> {
        let ptr = unsafe { ffi::mdl_object_child_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Self::from_handle)
    }

    #[must_use]
    pub fn children(&self) -> Vec<Self> {
        (0..self.child_count())
            .filter_map(|index| self.child_at(index))
            .collect()
    }

    pub fn at_path(&self, path: &str) -> Result<Option<Self>> {
        let path = c_string(path)?;
        let ptr = unsafe { ffi::mdl_object_at_path(self.handle.as_ptr(), path.as_ptr()) };
        Ok(unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Self::from_handle))
    }

    #[must_use]
    pub fn bounding_box_at_time(&self, time: f64) -> BoundingBox {
        let mut min = [0.0_f32; 3];
        let mut max = [0.0_f32; 3];
        unsafe {
            ffi::mdl_object_bounding_box_at_time(
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
    pub fn as_mesh(&self) -> Option<Mesh> {
        (self.kind() == ObjectKind::Mesh).then(|| Mesh::from_handle(self.handle.clone()))
    }

    #[must_use]
    pub fn as_light(&self) -> Option<Light> {
        matches!(
            self.kind(),
            ObjectKind::Light | ObjectKind::PhysicallyPlausibleLight
        )
        .then(|| Light::from_handle(self.handle.clone()))
    }

    #[must_use]
    pub fn as_physically_plausible_light(&self) -> Option<PhysicallyPlausibleLight> {
        (self.kind() == ObjectKind::PhysicallyPlausibleLight)
            .then(|| PhysicallyPlausibleLight::from_handle(self.handle.clone()))
    }

    #[must_use]
    pub fn as_camera(&self) -> Option<Camera> {
        (self.kind() == ObjectKind::Camera).then(|| Camera::from_handle(self.handle.clone()))
    }

    #[must_use]
    pub fn as_voxel_array(&self) -> Option<VoxelArray> {
        (self.kind() == ObjectKind::VoxelArray)
            .then(|| VoxelArray::from_handle(self.handle.clone()))
    }

    #[must_use]
    pub fn as_skeleton(&self) -> Option<Skeleton> {
        (self.kind() == ObjectKind::Skeleton).then(|| Skeleton::from_handle(self.handle.clone()))
    }

    #[must_use]
    pub fn as_packed_joint_animation(&self) -> Option<PackedJointAnimation> {
        (self.kind() == ObjectKind::PackedJointAnimation)
            .then(|| PackedJointAnimation::from_handle(self.handle.clone()))
    }
}
