use std::ptr;

use crate::asset::Asset;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::light::Light;
use crate::object::Object;
use crate::texture::Texture;
use crate::types::{BoundingBox, ProbePlacement};
use crate::util::required_handle;

type IrradianceCallbackFn = dyn Fn([f32; 3]) -> Vec<f32> + Send + Sync + 'static;

struct IrradianceCallback {
    callback: Box<IrradianceCallbackFn>,
}

#[no_mangle]
pub extern "C" fn mdlx_light_probe_irradiance_data_source_coefficients(
    context: *mut core::ffi::c_void,
    x: f32,
    y: f32,
    z: f32,
    out_values: *mut f32,
    capacity: u64,
) -> u64 {
    let Some(context) = (!context.is_null()).then_some(context.cast::<IrradianceCallback>()) else {
        return 0;
    };
    let values = (unsafe { &*context }.callback)([x, y, z]);
    let total = values.len();
    if out_values.is_null() || capacity == 0 {
        return total as u64;
    }
    let write_count = total.min(capacity as usize);
    unsafe { out_values.copy_from_nonoverlapping(values.as_ptr(), write_count) };
    total as u64
}

#[no_mangle]
pub extern "C" fn mdlx_light_probe_irradiance_data_source_release(context: *mut core::ffi::c_void) {
    if context.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(context.cast::<IrradianceCallback>())) };
}

fn release_callback_context(context: *mut core::ffi::c_void) {
    mdlx_light_probe_irradiance_data_source_release(context);
}

fn array_objects<T, F>(array_ptr: *mut core::ffi::c_void, context: &'static str, mut map: F) -> Result<Vec<T>>
where
    F: FnMut(ObjectHandle) -> T,
{
    let array = required_handle(array_ptr, context)?;
    let count = unsafe { ffi::mdl_array_count(array.as_ptr()) as usize };
    let mut values = Vec::with_capacity(count);
    for index in 0..count {
        let ptr = unsafe { ffi::mdl_array_object_at(array.as_ptr(), index as u64) };
        if let Some(handle) = unsafe { ObjectHandle::from_retained_ptr(ptr) } {
            values.push(map(handle));
        }
    }
    Ok(values)
}

#[derive(Debug, Clone)]
pub struct LightProbe {
    handle: ObjectHandle,
}

impl LightProbe {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(
        reflective_texture: Option<&Texture>,
        irradiance_texture: Option<&Texture>,
    ) -> Result<Self> {
        let mut out_probe = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_light_probe_new(
                reflective_texture.map_or(ptr::null_mut(), Texture::as_ptr),
                irradiance_texture.map_or(ptr::null_mut(), Texture::as_ptr),
                &mut out_probe,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(out_probe, "MDLLightProbe")?))
    }

    pub fn generate_spherical_harmonics_from_irradiance(&self, level: usize) {
        unsafe {
            ffi::mdl_light_probe_generate_spherical_harmonics_from_irradiance(
                self.handle.as_ptr(),
                level as u64,
            );
        }
    }

    #[must_use]
    pub fn reflective_texture(&self) -> Option<Texture> {
        let ptr = unsafe { ffi::mdl_light_probe_reflective_texture(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Texture::from_handle)
    }

    #[must_use]
    pub fn irradiance_texture(&self) -> Option<Texture> {
        let ptr = unsafe { ffi::mdl_light_probe_irradiance_texture(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Texture::from_handle)
    }

    #[must_use]
    pub fn spherical_harmonics_level(&self) -> usize {
        unsafe { ffi::mdl_light_probe_spherical_harmonics_level(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn spherical_harmonics_coefficients(&self) -> Vec<f32> {
        let count = unsafe {
            ffi::mdl_light_probe_spherical_harmonics_coefficient_count(self.handle.as_ptr()) as usize
        };
        let mut values = vec![0.0_f32; count];
        if values.is_empty() {
            return values;
        }
        let written = unsafe {
            ffi::mdl_light_probe_copy_spherical_harmonics_coefficients(
                self.handle.as_ptr(),
                values.as_mut_ptr(),
                values.len() as u64,
            )
        } as usize;
        values.truncate(written);
        values
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
pub struct LightProbeIrradianceDataSource {
    handle: ObjectHandle,
}

impl LightProbeIrradianceDataSource {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    pub fn new<F>(
        bounding_box: BoundingBox,
        spherical_harmonics_level: usize,
        coefficients_at_position: F,
    ) -> Result<Self>
    where
        F: Fn([f32; 3]) -> Vec<f32> + Send + Sync + 'static,
    {
        let callback = Box::new(IrradianceCallback {
            callback: Box::new(coefficients_at_position),
        });
        let callback_ptr = Box::into_raw(callback).cast::<core::ffi::c_void>();
        let mut out_data_source = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_light_probe_irradiance_data_source_new(
                bounding_box.min[0],
                bounding_box.min[1],
                bounding_box.min[2],
                bounding_box.max[0],
                bounding_box.max[1],
                bounding_box.max[2],
                spherical_harmonics_level as u64,
                callback_ptr,
                &mut out_data_source,
                &mut out_error,
            )
        };
        if let Err(error) = crate::util::status_result(status, out_error) {
            release_callback_context(callback_ptr);
            return Err(error);
        }
        match required_handle(out_data_source, "MDLLightProbeIrradianceDataSource") {
            Ok(handle) => Ok(Self::from_handle(handle)),
            Err(error) => {
                release_callback_context(callback_ptr);
                Err(error)
            }
        }
    }

    #[must_use]
    pub fn bounding_box(&self) -> BoundingBox {
        let mut min = [0.0_f32; 3];
        let mut max = [0.0_f32; 3];
        unsafe {
            ffi::mdl_light_probe_irradiance_data_source_bounding_box(
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

    pub fn set_bounding_box(&self, bounding_box: BoundingBox) {
        unsafe {
            ffi::mdl_light_probe_irradiance_data_source_set_bounding_box(
                self.handle.as_ptr(),
                bounding_box.min[0],
                bounding_box.min[1],
                bounding_box.min[2],
                bounding_box.max[0],
                bounding_box.max[1],
                bounding_box.max[2],
            );
        }
    }

    #[must_use]
    pub fn spherical_harmonics_level(&self) -> usize {
        unsafe {
            ffi::mdl_light_probe_irradiance_data_source_spherical_harmonics_level(
                self.handle.as_ptr(),
            ) as usize
        }
    }

    pub fn set_spherical_harmonics_level(&self, spherical_harmonics_level: usize) {
        unsafe {
            ffi::mdl_light_probe_irradiance_data_source_set_spherical_harmonics_level(
                self.handle.as_ptr(),
                spherical_harmonics_level as u64,
            );
        }
    }
}

impl Asset {
    pub fn place_light_probes(
        density: f32,
        heuristic: ProbePlacement,
        data_source: &LightProbeIrradianceDataSource,
    ) -> Result<Vec<LightProbe>> {
        let ptr = unsafe {
            ffi::mdl_asset_place_light_probes(
                density,
                heuristic.as_raw(),
                data_source.as_ptr(),
            )
        };
        if ptr.is_null() {
            return Ok(Vec::new());
        }
        array_objects(ptr, "MDLAsset light probes", LightProbe::from_handle)
    }
}
