use std::ptr;

use crate::camera::Camera;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::light::Light;
use crate::mesh::Mesh;
use crate::physically_plausible_light::PhysicallyPlausibleLight;
use crate::protocols::{Component, Named, ObjectContainerComponent};
use crate::skeleton::Skeleton;
use crate::types::{BoundingBox, ObjectInfo, ObjectKind};
use crate::util::{c_string, parse_json, required_handle, take_string};
use crate::voxel_array::VoxelArray;
use crate::PackedJointAnimation;

#[derive(Debug, Clone)]
pub struct Object {
    handle: ObjectHandle,
}

impl Named for Object {
    fn name(&self) -> Option<String> {
        self.name()
    }

    fn set_name(&self, name: &str) -> Result<()> {
        self.set_name(name)
    }
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
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_object_new(&mut out_object, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(out_object, "MDLObject")?))
    }

    pub fn info(&self) -> Result<ObjectInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_object_info_json(self.handle.as_ptr()) },
            "MDLObject",
        )
    }

    #[must_use]
    pub fn kind(&self) -> ObjectKind {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        ObjectKind::from_raw(unsafe { ffi::mdl_object_kind(self.handle.as_ptr()) })
            .unwrap_or(ObjectKind::Unknown)
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        take_string(unsafe { ffi::mdl_object_name_string(self.handle.as_ptr()) })
    }

    pub fn set_name(&self, name: &str) -> Result<()> {
        let name = c_string(name)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_object_set_name(self.handle.as_ptr(), name.as_ptr()) };
        Ok(())
    }

    #[must_use]
    pub fn path(&self) -> Option<String> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        take_string(unsafe { ffi::mdl_object_path_string(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn hidden(&self) -> bool {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_object_hidden(self.handle.as_ptr()) != 0 }
    }

    pub fn set_hidden(&self, hidden: bool) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_object_set_hidden(self.handle.as_ptr(), i32::from(hidden)) };
    }

    pub fn add_child(&self, child: &Self) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_object_add_child(self.handle.as_ptr(), child.handle.as_ptr()) };
    }

    #[must_use]
    pub fn children_container(&self) -> Option<ObjectContainer> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_object_children_container(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(ObjectContainer::from_handle)
    }

    pub fn set_children_container(&self, container: Option<&ObjectContainer>) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_object_set_children_container(
                self.handle.as_ptr(),
                container.map_or(ptr::null_mut(), ObjectContainer::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn child_count(&self) -> usize {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_object_child_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn child_at(&self, index: usize) -> Option<Self> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_object_child_at(self.handle.as_ptr(), index as u64) };
        // SAFETY: The unsafe operation is valid in this context.
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
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_object_at_path(self.handle.as_ptr(), path.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        Ok(unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Self::from_handle))
    }

    #[must_use]
    pub fn bounding_box_at_time(&self, time: f64) -> BoundingBox {
        let mut min = [0.0_f32; 3];
        let mut max = [0.0_f32; 3];
        // SAFETY: The unsafe operation is valid in this context.
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

#[derive(Debug, Clone)]
pub struct ObjectContainer {
    handle: ObjectHandle,
}

impl Component for ObjectContainer {}

impl ObjectContainerComponent for ObjectContainer {
    fn count(&self) -> usize {
        self.count()
    }

    fn object_at(&self, index: usize) -> Option<Object> {
        self.object_at(index)
    }

    fn add_object(&self, object: &Object) {
        self.add_object(object);
    }

    fn remove_object(&self, object: &Object) {
        self.remove_object(object);
    }
}

impl ObjectContainer {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    pub fn new() -> Result<Self> {
        let mut out_container = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_object_container_new(&mut out_container, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_container,
            "MDLObjectContainer",
        )?))
    }

    #[must_use]
    pub fn count(&self) -> usize {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_object_container_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn object_at(&self, index: usize) -> Option<Object> {
        let ptr =
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_object_container_object_at(self.handle.as_ptr(), index as u64) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Object::from_handle)
    }

    #[must_use]
    pub fn objects(&self) -> Vec<Object> {
        (0..self.count())
            .filter_map(|index| self.object_at(index))
            .collect()
    }

    pub fn add_object(&self, object: &Object) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_object_container_add_object(self.handle.as_ptr(), object.as_ptr()) };
    }

    pub fn remove_object(&self, object: &Object) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_object_container_remove_object(self.handle.as_ptr(), object.as_ptr()) };
    }
}
