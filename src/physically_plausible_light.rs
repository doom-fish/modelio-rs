use std::path::Path;
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::light::Light;
use crate::object::Object;
use crate::texture::Texture;
use crate::types::{AreaLightInfo, PhotometricLightInfo, PhysicallyPlausibleLightInfo};
use crate::util::{parse_json, path_to_c_string, required_handle};

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
            // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
            unsafe { ffi::mdl_physically_plausible_light_new(&mut out_light, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_light,
            "MDLPhysicallyPlausibleLight",
        )?))
    }

    pub fn info(&self) -> Result<PhysicallyPlausibleLightInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_physically_plausible_light_info_json(self.handle.as_ptr()) },
            "MDLPhysicallyPlausibleLight",
        )
    }

    pub fn set_color_temperature(&self, temperature: f32) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_physically_plausible_light_set_color_temperature(
                self.handle.as_ptr(),
                temperature,
            );
        };
    }

    pub fn set_color(&self, color: [f32; 4]) {
        // SAFETY: The unsafe operation is valid in this context.
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
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_physically_plausible_light_set_lumens(self.handle.as_ptr(), lumens) };
    }

    pub fn set_inner_cone_angle(&self, angle: f32) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_physically_plausible_light_set_inner_cone_angle(self.handle.as_ptr(), angle);
        };
    }

    pub fn set_outer_cone_angle(&self, angle: f32) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_physically_plausible_light_set_outer_cone_angle(self.handle.as_ptr(), angle);
        };
    }

    pub fn set_attenuation_start_distance(&self, distance: f32) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_physically_plausible_light_set_attenuation_start_distance(
                self.handle.as_ptr(),
                distance,
            );
        };
    }

    pub fn set_attenuation_end_distance(&self, distance: f32) {
        // SAFETY: The unsafe operation is valid in this context.
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

#[derive(Debug, Clone)]
pub struct AreaLight {
    handle: ObjectHandle,
}

impl AreaLight {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_light = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_area_light_new(&mut out_light, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_light,
            "MDLAreaLight",
        )?))
    }

    pub fn info(&self) -> Result<AreaLightInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_area_light_info_json(self.handle.as_ptr()) },
            "MDLAreaLight",
        )
    }

    pub fn set_area_radius(&self, value: f32) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_area_light_set_area_radius(self.handle.as_ptr(), value) };
    }

    pub fn set_super_elliptic_power(&self, value: [f32; 2]) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_area_light_set_super_elliptic_power(self.handle.as_ptr(), value[0], value[1]);
        };
    }

    pub fn set_aspect(&self, value: f32) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_area_light_set_aspect(self.handle.as_ptr(), value) };
    }

    #[must_use]
    pub fn as_physically_plausible_light(&self) -> PhysicallyPlausibleLight {
        PhysicallyPlausibleLight::from_handle(self.handle.clone())
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

#[derive(Debug, Clone)]
pub struct PhotometricLight {
    handle: ObjectHandle,
}

impl PhotometricLight {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_light = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_photometric_light_new(&mut out_light, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_light,
            "MDLPhotometricLight",
        )?))
    }

    pub fn from_ies_profile(path: impl AsRef<Path>) -> Result<Self> {
        let path = path_to_c_string(path.as_ref())?;
        let mut out_light = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_photometric_light_new_with_ies_profile(
                path.as_ptr(),
                &mut out_light,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_light,
            "MDLPhotometricLight",
        )?))
    }

    pub fn info(&self) -> Result<PhotometricLightInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_photometric_light_info_json(self.handle.as_ptr()) },
            "MDLPhotometricLight",
        )
    }

    pub fn generate_spherical_harmonics_from_light(&self, level: usize) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_photometric_light_generate_spherical_harmonics_from_light(
                self.handle.as_ptr(),
                level as u64,
            );
        };
    }

    pub fn generate_cubemap_from_light(&self, texture_size: usize) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_photometric_light_generate_cubemap_from_light(
                self.handle.as_ptr(),
                texture_size as u64,
            );
        };
    }

    #[must_use]
    pub fn generate_texture(&self, texture_size: usize) -> Option<Texture> {
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_photometric_light_generate_texture(self.handle.as_ptr(), texture_size as u64)
        };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Texture::from_handle)
    }

    #[must_use]
    pub fn light_cube_map(&self) -> Option<Texture> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_photometric_light_light_cube_map(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Texture::from_handle)
    }

    #[must_use]
    pub fn as_physically_plausible_light(&self) -> PhysicallyPlausibleLight {
        PhysicallyPlausibleLight::from_handle(self.handle.clone())
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
