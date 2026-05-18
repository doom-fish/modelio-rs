use std::ffi::CString;
use std::ptr;

use crate::animated_value_types::{AnimatedQuaternionArray, AnimatedVector3Array};
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::object::Object;
use crate::protocols::{Component, JointAnimation};
use crate::skeleton::Skeleton;
use crate::types::{AnimationBindComponentInfo, PackedJointAnimationInfo};
use crate::util::{c_string, parse_json, required_handle};

fn c_string_vec(values: &[&str]) -> Result<(Vec<CString>, Vec<*const i8>)> {
    let c_strings = values
        .iter()
        .map(|value| c_string(value))
        .collect::<Result<Vec<_>>>()?;
    let raw = c_strings.iter().map(|value| value.as_ptr()).collect();
    Ok((c_strings, raw))
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O packed joint animation counterpart.
pub struct PackedJointAnimation {
    handle: ObjectHandle,
}

impl JointAnimation for PackedJointAnimation {}

impl PackedJointAnimation {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O packed joint animation counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O packed joint animation counterpart.
    pub fn new(name: &str, joint_paths: &[&str]) -> Result<Self> {
        let name = c_string(name)?;
        let (_joint_paths, raw_joint_paths) = c_string_vec(joint_paths)?;
        let mut out_animation = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_packed_joint_animation_new(
                name.as_ptr(),
                raw_joint_paths.as_ptr(),
                raw_joint_paths.len() as u64,
                &mut out_animation,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_animation,
            "MDLPackedJointAnimation",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O packed joint animation counterpart.
    pub fn info(&self) -> Result<PackedJointAnimationInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_packed_joint_animation_info_json(self.handle.as_ptr()) },
            "MDLPackedJointAnimation",
        )
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O packed joint animation counterpart.
    pub fn translations(&self) -> Result<AnimatedVector3Array> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_packed_joint_animation_translations(self.handle.as_ptr()) };
        Ok(AnimatedVector3Array::from_handle(required_handle(
            ptr,
            "MDLPackedJointAnimation translations",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O packed joint animation counterpart.
    pub fn rotations(&self) -> Result<AnimatedQuaternionArray> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_packed_joint_animation_rotations(self.handle.as_ptr()) };
        Ok(AnimatedQuaternionArray::from_handle(required_handle(
            ptr,
            "MDLPackedJointAnimation rotations",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O packed joint animation counterpart.
    pub fn scales(&self) -> Result<AnimatedVector3Array> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_packed_joint_animation_scales(self.handle.as_ptr()) };
        Ok(AnimatedVector3Array::from_handle(required_handle(
            ptr,
            "MDLPackedJointAnimation scales",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O packed joint animation counterpart.
    pub fn as_object(&self) -> Object {
        Object::from_handle(self.handle.clone())
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O animation bind component counterpart.
pub struct AnimationBindComponent {
    handle: ObjectHandle,
}

impl Component for AnimationBindComponent {}

impl AnimationBindComponent {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O animation bind component counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O animation bind component counterpart.
    pub fn new() -> Result<Self> {
        let mut out_component = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status =
            // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
            unsafe { ffi::mdl_animation_bind_component_new(&mut out_component, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_component,
            "MDLAnimationBindComponent",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animation bind component counterpart.
    pub fn info(&self) -> Result<AnimationBindComponentInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_animation_bind_component_info_json(self.handle.as_ptr()) },
            "MDLAnimationBindComponent",
        )
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animation bind component counterpart.
    pub fn set_skeleton(&self, skeleton: &Skeleton) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animation_bind_component_set_skeleton(
                self.handle.as_ptr(),
                skeleton.handle.as_ptr(),
            );
        };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O animation bind component counterpart.
    pub fn skeleton(&self) -> Option<Skeleton> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_animation_bind_component_skeleton(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Skeleton::from_handle)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animation bind component counterpart.
    pub fn set_packed_joint_animation(&self, animation: &PackedJointAnimation) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animation_bind_component_set_packed_joint_animation(
                self.handle.as_ptr(),
                animation.handle.as_ptr(),
            );
        };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O animation bind component counterpart.
    pub fn packed_joint_animation(&self) -> Option<PackedJointAnimation> {
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_animation_bind_component_packed_joint_animation(self.handle.as_ptr())
        };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PackedJointAnimation::from_handle)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animation bind component counterpart.
    pub fn set_joint_paths(&self, joint_paths: &[&str]) -> Result<()> {
        let (_joint_paths, raw_joint_paths) = c_string_vec(joint_paths)?;
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animation_bind_component_set_joint_paths(
                self.handle.as_ptr(),
                raw_joint_paths.as_ptr(),
                raw_joint_paths.len() as u64,
            );
        };
        Ok(())
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animation bind component counterpart.
    pub fn set_geometry_bind_transform(&self, matrix: [f32; 16]) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animation_bind_component_set_geometry_bind_transform(
                self.handle.as_ptr(),
                matrix.as_ptr(),
            );
        };
    }
}
