use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::light::Light;
use crate::object::Object;
use crate::types::PhysicallyPlausibleLightInfo;
use crate::util::{parse_json, required_handle};

#[derive(Debug, Clone)]
pub struct PhysicallyPlausibleLight {
    handle: ObjectHandle,
}

impl PhysicallyPlausibleLight {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_light = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status =
            unsafe { ffi::mdl_physically_plausible_light_new(&mut out_light, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_light,
            "MDLPhysicallyPlausibleLight",
        )?))
    }

    pub fn info(&self) -> Result<PhysicallyPlausibleLightInfo> {
        parse_json(
            unsafe { ffi::mdl_physically_plausible_light_info_json(self.handle.as_ptr()) },
            "MDLPhysicallyPlausibleLight",
        )
    }

    pub fn set_color_temperature(&self, temperature: f32) {
        unsafe {
            ffi::mdl_physically_plausible_light_set_color_temperature(
                self.handle.as_ptr(),
                temperature,
            );
        };
    }

    pub fn set_color(&self, color: [f32; 4]) {
        unsafe {
            ffi::mdl_physically_plausible_light_set_color(
                self.handle.as_ptr(),
                color[0],
                color[1],
                color[2],
                color[3],
            );
        };
    }

    pub fn set_lumens(&self, lumens: f32) {
        unsafe { ffi::mdl_physically_plausible_light_set_lumens(self.handle.as_ptr(), lumens) };
    }

    pub fn set_inner_cone_angle(&self, angle: f32) {
        unsafe {
            ffi::mdl_physically_plausible_light_set_inner_cone_angle(self.handle.as_ptr(), angle);
        };
    }

    pub fn set_outer_cone_angle(&self, angle: f32) {
        unsafe {
            ffi::mdl_physically_plausible_light_set_outer_cone_angle(self.handle.as_ptr(), angle);
        };
    }

    pub fn set_attenuation_start_distance(&self, distance: f32) {
        unsafe {
            ffi::mdl_physically_plausible_light_set_attenuation_start_distance(
                self.handle.as_ptr(),
                distance,
            );
        };
    }

    pub fn set_attenuation_end_distance(&self, distance: f32) {
        unsafe {
            ffi::mdl_physically_plausible_light_set_attenuation_end_distance(
                self.handle.as_ptr(),
                distance,
            );
        };
    }

    #[must_use]
    pub fn as_light(&self) -> Light {
        Light::from_handle(self.handle.clone())
    }

    #[must_use]
    pub fn as_object(&self) -> Object {
        Object::from_handle(self.handle.clone())
    }
}
