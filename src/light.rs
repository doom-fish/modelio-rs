use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::object::Object;
use crate::types::{LightInfo, LightType};
use crate::util::{c_string, parse_json, required_handle};

#[derive(Debug, Clone)]
pub struct Light {
    handle: ObjectHandle,
}

impl Light {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_light = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_light_new(&mut out_light, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(out_light, "MDLLight")?))
    }

    pub fn info(&self) -> Result<LightInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_light_info_json(self.handle.as_ptr()) },
            "MDLLight",
        )
    }

    pub fn set_light_type(&self, light_type: LightType) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_light_set_light_type(self.handle.as_ptr(), light_type.as_raw()) };
    }

    pub fn set_color_space(&self, color_space: &str) -> Result<()> {
        let color_space = c_string(color_space)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_light_set_color_space(self.handle.as_ptr(), color_space.as_ptr()) };
        Ok(())
    }

    #[must_use]
    pub fn irradiance_at_point(&self, point: [f32; 3]) -> [f32; 4] {
        let mut components = [0.0_f32; 4];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_light_irradiance_at_point(
                self.handle.as_ptr(),
                point[0],
                point[1],
                point[2],
                &mut components[0],
                &mut components[1],
                &mut components[2],
                &mut components[3],
            );
        }
        components
    }

    #[must_use]
    pub fn as_object(&self) -> Object {
        Object::from_handle(self.handle.clone())
    }
}
