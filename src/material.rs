use std::path::Path;
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::texture::Texture;
use crate::types::{MaterialFace, MaterialInfo, MaterialPropertyInfo, MaterialSemantic};
use crate::util::{c_string, parse_json, path_to_c_string, required_handle, take_string};

#[derive(Debug, Clone)]
pub struct Material {
    handle: ObjectHandle,
}

impl Material {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    pub fn new(name: &str, physically_plausible: bool) -> Result<Self> {
        let name = c_string(name)?;
        let mut out_material = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_material_new(
                name.as_ptr(),
                i32::from(physically_plausible),
                &mut out_material,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_material,
            "MDLMaterial",
        )?))
    }

    pub fn info(&self) -> Result<MaterialInfo> {
        parse_json(
            unsafe { ffi::mdl_material_info_json(self.handle.as_ptr()) },
            "MDLMaterial",
        )
    }

    #[must_use]
    pub fn count(&self) -> usize {
        unsafe { ffi::mdl_material_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        take_string(unsafe { ffi::mdl_material_name_string(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn material_face(&self) -> Option<MaterialFace> {
        MaterialFace::from_raw(unsafe { ffi::mdl_material_material_face(self.handle.as_ptr()) })
    }

    pub fn set_material_face(&self, face: MaterialFace) {
        unsafe { ffi::mdl_material_set_material_face(self.handle.as_ptr(), face.as_raw()) };
    }

    pub fn remove_all_properties(&self) {
        unsafe { ffi::mdl_material_remove_all_properties(self.handle.as_ptr()) };
    }

    #[must_use]
    pub fn property(&self, index: usize) -> Option<MaterialProperty> {
        let ptr = unsafe { ffi::mdl_material_property_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MaterialProperty::from_handle)
    }

    pub fn property_named(&self, name: &str) -> Result<Option<MaterialProperty>> {
        let name = c_string(name)?;
        let ptr = unsafe { ffi::mdl_material_property_named(self.handle.as_ptr(), name.as_ptr()) };
        Ok(unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MaterialProperty::from_handle))
    }

    #[must_use]
    pub fn property_with_semantic(&self, semantic: MaterialSemantic) -> Option<MaterialProperty> {
        let ptr = unsafe {
            ffi::mdl_material_property_with_semantic(self.handle.as_ptr(), semantic as u32)
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MaterialProperty::from_handle)
    }

    #[must_use]
    pub fn properties(&self) -> Vec<MaterialProperty> {
        (0..self.count())
            .filter_map(|index| self.property(index))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct MaterialProperty {
    handle: ObjectHandle,
}

impl MaterialProperty {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn info(&self) -> Result<MaterialPropertyInfo> {
        parse_json(
            unsafe { ffi::mdl_material_property_info_json(self.handle.as_ptr()) },
            "MDLMaterialProperty",
        )
    }

    #[must_use]
    pub fn texture(&self) -> Option<Texture> {
        let ptr = unsafe { ffi::mdl_material_property_texture(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Texture::from_handle)
    }

    pub fn set_float(&self, value: f32) {
        unsafe { ffi::mdl_material_property_set_float(self.handle.as_ptr(), value) };
    }

    pub fn set_float2(&self, value: [f32; 2]) {
        unsafe { ffi::mdl_material_property_set_float2(self.handle.as_ptr(), value[0], value[1]) };
    }

    pub fn set_float3(&self, value: [f32; 3]) {
        unsafe {
            ffi::mdl_material_property_set_float3(
                self.handle.as_ptr(),
                value[0],
                value[1],
                value[2],
            );
        };
    }

    pub fn set_float4(&self, value: [f32; 4]) {
        unsafe {
            ffi::mdl_material_property_set_float4(
                self.handle.as_ptr(),
                value[0],
                value[1],
                value[2],
                value[3],
            );
        };
    }

    pub fn set_matrix4x4(&self, value: [f32; 16]) {
        unsafe { ffi::mdl_material_property_set_matrix4x4(self.handle.as_ptr(), value.as_ptr()) };
    }

    pub fn set_string(&self, value: Option<&str>) -> Result<()> {
        let value = value.map(c_string).transpose()?;
        unsafe {
            ffi::mdl_material_property_set_string(
                self.handle.as_ptr(),
                value.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
        Ok(())
    }

    pub fn set_url(&self, path: Option<impl AsRef<Path>>) -> Result<()> {
        let path = path
            .map(|path| path_to_c_string(path.as_ref()))
            .transpose()?;
        unsafe {
            ffi::mdl_material_property_set_url(
                self.handle.as_ptr(),
                path.as_ref().map_or(ptr::null(), |path| path.as_ptr()),
            );
        };
        Ok(())
    }

    pub fn set_color(&self, color: [f32; 4]) {
        unsafe {
            ffi::mdl_material_property_set_color(
                self.handle.as_ptr(),
                color[0],
                color[1],
                color[2],
                color[3],
            );
        };
    }

    pub fn set_luminance(&self, value: f32) {
        unsafe { ffi::mdl_material_property_set_luminance(self.handle.as_ptr(), value) };
    }
}
