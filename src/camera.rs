use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::object::Object;
use crate::texture::Texture;
use crate::types::{BoundingBox, CameraInfo, CameraProjection};
use crate::util::{parse_json, required_handle};

#[derive(Debug, Clone)]
pub struct Camera {
    handle: ObjectHandle,
}

impl Camera {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_camera = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe { ffi::mdl_camera_new(&mut out_camera, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(out_camera, "MDLCamera")?))
    }

    pub fn info(&self) -> Result<CameraInfo> {
        parse_json(
            unsafe { ffi::mdl_camera_info_json(self.handle.as_ptr()) },
            "MDLCamera",
        )
    }

    pub fn set_projection(&self, projection: CameraProjection) {
        unsafe { ffi::mdl_camera_set_projection(self.handle.as_ptr(), projection.as_raw()) };
    }

    pub fn set_near_visibility_distance(&self, value: f32) {
        unsafe { ffi::mdl_camera_set_near_visibility_distance(self.handle.as_ptr(), value) };
    }

    pub fn set_far_visibility_distance(&self, value: f32) {
        unsafe { ffi::mdl_camera_set_far_visibility_distance(self.handle.as_ptr(), value) };
    }

    pub fn set_world_to_meters_conversion_scale(&self, value: f32) {
        unsafe {
            ffi::mdl_camera_set_world_to_meters_conversion_scale(self.handle.as_ptr(), value);
        };
    }

    pub fn set_focal_length(&self, value: f32) {
        unsafe { ffi::mdl_camera_set_focal_length(self.handle.as_ptr(), value) };
    }

    pub fn set_focus_distance(&self, value: f32) {
        unsafe { ffi::mdl_camera_set_focus_distance(self.handle.as_ptr(), value) };
    }

    pub fn set_field_of_view(&self, value: f32) {
        unsafe { ffi::mdl_camera_set_field_of_view(self.handle.as_ptr(), value) };
    }

    pub fn look_at(&self, focus_position: [f32; 3]) {
        unsafe {
            ffi::mdl_camera_look_at(
                self.handle.as_ptr(),
                focus_position[0],
                focus_position[1],
                focus_position[2],
            );
        };
    }

    pub fn look_at_from(&self, focus_position: [f32; 3], camera_position: [f32; 3]) {
        unsafe {
            ffi::mdl_camera_look_at_from(
                self.handle.as_ptr(),
                focus_position[0],
                focus_position[1],
                focus_position[2],
                camera_position[0],
                camera_position[1],
                camera_position[2],
            );
        };
    }

    pub fn frame_bounding_box(&self, bounding_box: BoundingBox, set_near_and_far: bool) {
        unsafe {
            ffi::mdl_camera_frame_bounding_box(
                self.handle.as_ptr(),
                bounding_box.min[0],
                bounding_box.min[1],
                bounding_box.min[2],
                bounding_box.max[0],
                bounding_box.max[1],
                bounding_box.max[2],
                i32::from(set_near_and_far),
            );
        };
    }

    #[must_use]
    pub fn ray_to(&self, pixel: [i32; 2], viewport: [i32; 2]) -> [f32; 3] {
        let mut ray = [0.0_f32; 3];
        unsafe {
            ffi::mdl_camera_ray_to(
                self.handle.as_ptr(),
                pixel[0],
                pixel[1],
                viewport[0],
                viewport[1],
                &mut ray[0],
                &mut ray[1],
                &mut ray[2],
            );
        };
        ray
    }

    #[must_use]
    pub fn bokeh_kernel(&self, size: [i32; 2]) -> Option<Texture> {
        let ptr = unsafe { ffi::mdl_camera_bokeh_kernel(self.handle.as_ptr(), size[0], size[1]) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Texture::from_handle)
    }

    #[must_use]
    pub fn as_object(&self) -> Object {
        Object::from_handle(self.handle.clone())
    }
}
