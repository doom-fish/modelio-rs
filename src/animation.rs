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
pub struct PackedJointAnimation {
    handle: ObjectHandle,
}

impl JointAnimation for PackedJointAnimation {}

impl PackedJointAnimation {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(name: &str, joint_paths: &[&str]) -> Result<Self> {
        let name = c_string(name)?;
        let (_joint_paths, raw_joint_paths) = c_string_vec(joint_paths)?;
        let mut out_animation = ptr::null_mut();
        let mut out_error = ptr::null_mut();
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

    pub fn info(&self) -> Result<PackedJointAnimationInfo> {
        parse_json(
            unsafe { ffi::mdl_packed_joint_animation_info_json(self.handle.as_ptr()) },
            "MDLPackedJointAnimation",
        )
    }

    pub fn translations(&self) -> Result<AnimatedVector3Array> {
        let ptr = unsafe { ffi::mdl_packed_joint_animation_translations(self.handle.as_ptr()) };
        Ok(AnimatedVector3Array::from_handle(required_handle(
            ptr,
            "MDLPackedJointAnimation translations",
        )?))
    }

    pub fn rotations(&self) -> Result<AnimatedQuaternionArray> {
        let ptr = unsafe { ffi::mdl_packed_joint_animation_rotations(self.handle.as_ptr()) };
        Ok(AnimatedQuaternionArray::from_handle(required_handle(
            ptr,
            "MDLPackedJointAnimation rotations",
        )?))
    }

    pub fn scales(&self) -> Result<AnimatedVector3Array> {
        let ptr = unsafe { ffi::mdl_packed_joint_animation_scales(self.handle.as_ptr()) };
        Ok(AnimatedVector3Array::from_handle(required_handle(
            ptr,
            "MDLPackedJointAnimation scales",
        )?))
    }

    #[must_use]
    pub fn as_object(&self) -> Object {
        Object::from_handle(self.handle.clone())
    }
}

#[derive(Debug, Clone)]
pub struct AnimationBindComponent {
    handle: ObjectHandle,
}

impl Component for AnimationBindComponent {}

impl AnimationBindComponent {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_component = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status =
            unsafe { ffi::mdl_animation_bind_component_new(&mut out_component, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_component,
            "MDLAnimationBindComponent",
        )?))
    }

    pub fn info(&self) -> Result<AnimationBindComponentInfo> {
        parse_json(
            unsafe { ffi::mdl_animation_bind_component_info_json(self.handle.as_ptr()) },
            "MDLAnimationBindComponent",
        )
    }

    pub fn set_skeleton(&self, skeleton: &Skeleton) {
        unsafe {
            ffi::mdl_animation_bind_component_set_skeleton(
                self.handle.as_ptr(),
                skeleton.handle.as_ptr(),
            );
        };
    }

    #[must_use]
    pub fn skeleton(&self) -> Option<Skeleton> {
        let ptr = unsafe { ffi::mdl_animation_bind_component_skeleton(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Skeleton::from_handle)
    }

    pub fn set_packed_joint_animation(&self, animation: &PackedJointAnimation) {
        unsafe {
            ffi::mdl_animation_bind_component_set_packed_joint_animation(
                self.handle.as_ptr(),
                animation.handle.as_ptr(),
            );
        };
    }

    #[must_use]
    pub fn packed_joint_animation(&self) -> Option<PackedJointAnimation> {
        let ptr = unsafe {
            ffi::mdl_animation_bind_component_packed_joint_animation(self.handle.as_ptr())
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(PackedJointAnimation::from_handle)
    }

    pub fn set_joint_paths(&self, joint_paths: &[&str]) -> Result<()> {
        let (_joint_paths, raw_joint_paths) = c_string_vec(joint_paths)?;
        unsafe {
            ffi::mdl_animation_bind_component_set_joint_paths(
                self.handle.as_ptr(),
                raw_joint_paths.as_ptr(),
                raw_joint_paths.len() as u64,
            );
        };
        Ok(())
    }

    pub fn set_geometry_bind_transform(&self, matrix: [f32; 16]) {
        unsafe {
            ffi::mdl_animation_bind_component_set_geometry_bind_transform(
                self.handle.as_ptr(),
                matrix.as_ptr(),
            );
        };
    }
}
