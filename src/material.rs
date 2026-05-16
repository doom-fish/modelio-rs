use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::texture::Texture;
use crate::types::{MaterialPropertyInfo, MaterialSemantic};
use crate::util::{c_string, parse_json, take_string};

#[derive(Debug, Clone)]
pub struct Material {
    handle: ObjectHandle,
}

impl Material {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
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
}
