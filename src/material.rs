use std::path::Path;
use std::ptr;

use crate::asset_resolver::AssetResolver;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::protocols::Named;
use crate::texture::Texture;
use crate::transform::Transform;
use crate::types::{
    MaterialFace, MaterialInfo, MaterialMipMapFilterMode, MaterialPropertyInfo, MaterialSemantic,
    MaterialTextureFilterMode, MaterialTextureWrapMode, TextureFilterInfo, TextureSamplerInfo,
};
use crate::util::{c_string, parse_json, path_to_c_string, required_handle, take_string};

fn array_objects<T, F>(
    array_ptr: *mut core::ffi::c_void,
    context: &'static str,
    mut map: F,
) -> Result<Vec<T>>
where
    F: FnMut(ObjectHandle) -> T,
{
    let array = required_handle(array_ptr, context)?;
    // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
    let count = unsafe { ffi::mdl_array_count(array.as_ptr()) as usize };
    let mut values = Vec::with_capacity(count);
    for index in 0..count {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_array_object_at(array.as_ptr(), index as u64) };
        // SAFETY: The unsafe operation is valid in this context.
        if let Some(handle) = unsafe { ObjectHandle::from_retained_ptr(ptr) } {
            values.push(map(handle));
        }
    }
    Ok(values)
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O material counterpart.
pub struct Material {
    handle: ObjectHandle,
}

impl Named for Material {
    fn name(&self) -> Option<String> {
        self.name()
    }

    fn set_name(&self, name: &str) -> Result<()> {
        self.set_name(name)
    }
}

impl Material {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O material counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O material counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O material counterpart.
    pub fn new(name: &str, physically_plausible: bool) -> Result<Self> {
        let name = c_string(name)?;
        let mut out_material = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn info(&self) -> Result<MaterialInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_material_info_json(self.handle.as_ptr()) },
            "MDLMaterial",
        )
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn count(&self) -> usize {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_material_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn name(&self) -> Option<String> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        take_string(unsafe { ffi::mdl_material_name_string(self.handle.as_ptr()) })
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn set_name(&self, name: &str) -> Result<()> {
        let name = c_string(name)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_material_set_name(self.handle.as_ptr(), name.as_ptr()) };
        Ok(())
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn material_face(&self) -> Option<MaterialFace> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        MaterialFace::from_raw(unsafe { ffi::mdl_material_material_face(self.handle.as_ptr()) })
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn set_material_face(&self, face: MaterialFace) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_material_set_material_face(self.handle.as_ptr(), face.as_raw()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn remove_all_properties(&self) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_material_remove_all_properties(self.handle.as_ptr()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn load_textures_using_resolver(&self, resolver: &AssetResolver) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_material_load_textures_using_resolver(self.handle.as_ptr(), resolver.as_ptr());
        };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn property(&self, index: usize) -> Option<MaterialProperty> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_material_property_at(self.handle.as_ptr(), index as u64) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MaterialProperty::from_handle)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn property_named(&self, name: &str) -> Result<Option<MaterialProperty>> {
        let name = c_string(name)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_material_property_named(self.handle.as_ptr(), name.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        Ok(unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MaterialProperty::from_handle))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn property_with_semantic(&self, semantic: MaterialSemantic) -> Option<MaterialProperty> {
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_material_property_with_semantic(self.handle.as_ptr(), semantic as u32)
        };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MaterialProperty::from_handle)
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material counterpart.
    pub fn properties(&self) -> Vec<MaterialProperty> {
        (0..self.count())
            .filter_map(|index| self.property(index))
            .collect()
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O material property counterpart.
pub struct MaterialProperty {
    handle: ObjectHandle,
}

impl Named for MaterialProperty {
    fn name(&self) -> Option<String> {
        self.name()
    }

    fn set_name(&self, name: &str) -> Result<()> {
        self.set_name(name)
    }
}

impl MaterialProperty {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O material property counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O material property counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O material property counterpart.
    pub fn new(name: &str, semantic: MaterialSemantic) -> Result<Self> {
        let name = c_string(name)?;
        let mut out_property = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_material_property_new(
                name.as_ptr(),
                semantic as u32,
                &mut out_property,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_property,
            "MDLMaterialProperty",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn name(&self) -> Option<String> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        take_string(unsafe { ffi::mdl_named_name_string(self.handle.as_ptr()) })
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn set_name(&self, name: &str) -> Result<()> {
        let name = c_string(name)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_named_set_name(self.handle.as_ptr(), name.as_ptr()) };
        Ok(())
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn info(&self) -> Result<MaterialPropertyInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_material_property_info_json(self.handle.as_ptr()) },
            "MDLMaterialProperty",
        )
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn texture(&self) -> Option<Texture> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_material_property_texture(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Texture::from_handle)
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn texture_sampler(&self) -> Option<TextureSampler> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_material_property_texture_sampler(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(TextureSampler::from_handle)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn set_texture_sampler(&self, sampler: Option<&TextureSampler>) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_material_property_set_texture_sampler(
                self.handle.as_ptr(),
                sampler.map_or(ptr::null_mut(), TextureSampler::as_ptr),
            );
        };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn set_float(&self, value: f32) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_material_property_set_float(self.handle.as_ptr(), value) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn set_float2(&self, value: [f32; 2]) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_material_property_set_float2(self.handle.as_ptr(), value[0], value[1]) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn set_float3(&self, value: [f32; 3]) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_material_property_set_float3(
                self.handle.as_ptr(),
                value[0],
                value[1],
                value[2],
            );
        };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn set_float4(&self, value: [f32; 4]) {
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn set_matrix4x4(&self, value: [f32; 16]) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_material_property_set_matrix4x4(self.handle.as_ptr(), value.as_ptr()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn set_string(&self, value: Option<&str>) -> Result<()> {
        let value = value.map(c_string).transpose()?;
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_material_property_set_string(
                self.handle.as_ptr(),
                value.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
            );
        };
        Ok(())
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn set_url(&self, path: Option<impl AsRef<Path>>) -> Result<()> {
        let path = path
            .map(|path| path_to_c_string(path.as_ref()))
            .transpose()?;
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_material_property_set_url(
                self.handle.as_ptr(),
                path.as_ref().map_or(ptr::null(), |path| path.as_ptr()),
            );
        };
        Ok(())
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn set_color(&self, color: [f32; 4]) {
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property counterpart.
    pub fn set_luminance(&self, value: f32) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_material_property_set_luminance(self.handle.as_ptr(), value) };
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O texture filter counterpart.
pub struct TextureFilter {
    handle: ObjectHandle,
}

impl TextureFilter {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O texture filter counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O texture filter counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O texture filter counterpart.
    pub fn new() -> Result<Self> {
        let mut out_filter = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_texture_filter_new(&mut out_filter, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_filter,
            "MDLTextureFilter",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture filter counterpart.
    pub fn info(&self) -> Result<TextureFilterInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_texture_filter_info_json(self.handle.as_ptr()) },
            "MDLTextureFilter",
        )
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture filter counterpart.
    pub fn set_s_wrap_mode(&self, value: MaterialTextureWrapMode) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_texture_filter_set_s_wrap_mode(self.handle.as_ptr(), value.as_raw()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture filter counterpart.
    pub fn set_t_wrap_mode(&self, value: MaterialTextureWrapMode) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_texture_filter_set_t_wrap_mode(self.handle.as_ptr(), value.as_raw()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture filter counterpart.
    pub fn set_r_wrap_mode(&self, value: MaterialTextureWrapMode) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_texture_filter_set_r_wrap_mode(self.handle.as_ptr(), value.as_raw()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture filter counterpart.
    pub fn set_min_filter(&self, value: MaterialTextureFilterMode) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_texture_filter_set_min_filter(self.handle.as_ptr(), value.as_raw()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture filter counterpart.
    pub fn set_mag_filter(&self, value: MaterialTextureFilterMode) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_texture_filter_set_mag_filter(self.handle.as_ptr(), value.as_raw()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture filter counterpart.
    pub fn set_mip_filter(&self, value: MaterialMipMapFilterMode) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_texture_filter_set_mip_filter(self.handle.as_ptr(), value.as_raw()) };
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O texture sampler counterpart.
pub struct TextureSampler {
    handle: ObjectHandle,
}

impl TextureSampler {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O texture sampler counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O texture sampler counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O texture sampler counterpart.
    pub fn new() -> Result<Self> {
        let mut out_sampler = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_texture_sampler_new(&mut out_sampler, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_sampler,
            "MDLTextureSampler",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture sampler counterpart.
    pub fn info(&self) -> Result<TextureSamplerInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_texture_sampler_info_json(self.handle.as_ptr()) },
            "MDLTextureSampler",
        )
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture sampler counterpart.
    pub fn texture(&self) -> Option<Texture> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_texture_sampler_texture(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Texture::from_handle)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture sampler counterpart.
    pub fn set_texture(&self, texture: Option<&Texture>) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_texture_sampler_set_texture(
                self.handle.as_ptr(),
                texture.map_or(ptr::null_mut(), Texture::as_ptr),
            );
        };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture sampler counterpart.
    pub fn hardware_filter(&self) -> Option<TextureFilter> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_texture_sampler_hardware_filter(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(TextureFilter::from_handle)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture sampler counterpart.
    pub fn set_hardware_filter(&self, filter: Option<&TextureFilter>) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_texture_sampler_set_hardware_filter(
                self.handle.as_ptr(),
                filter.map_or(ptr::null_mut(), TextureFilter::as_ptr),
            );
        };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture sampler counterpart.
    pub fn transform(&self) -> Option<Transform> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_texture_sampler_transform(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Transform::from_handle)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture sampler counterpart.
    pub fn set_transform(&self, transform: Option<&Transform>) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_texture_sampler_set_transform(
                self.handle.as_ptr(),
                transform.map_or(ptr::null_mut(), Transform::as_ptr),
            );
        };
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O material property connection counterpart.
pub struct MaterialPropertyConnection {
    handle: ObjectHandle,
}

impl Named for MaterialPropertyConnection {
    fn name(&self) -> Option<String> {
        self.name()
    }

    fn set_name(&self, name: &str) -> Result<()> {
        self.set_name(name)
    }
}

impl MaterialPropertyConnection {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O material property connection counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O material property connection counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O material property connection counterpart.
    pub fn new(output: &MaterialProperty, input: &MaterialProperty) -> Result<Self> {
        let mut out_connection = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_material_property_connection_new(
                output.as_ptr(),
                input.as_ptr(),
                &mut out_connection,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_connection,
            "MDLMaterialPropertyConnection",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property connection counterpart.
    pub fn name(&self) -> Option<String> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        take_string(unsafe { ffi::mdl_named_name_string(self.handle.as_ptr()) })
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property connection counterpart.
    pub fn set_name(&self, name: &str) -> Result<()> {
        let name = c_string(name)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_named_set_name(self.handle.as_ptr(), name.as_ptr()) };
        Ok(())
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property connection counterpart.
    pub fn output(&self) -> Option<MaterialProperty> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_material_property_connection_output(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MaterialProperty::from_handle)
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property connection counterpart.
    pub fn input(&self) -> Option<MaterialProperty> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_material_property_connection_input(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MaterialProperty::from_handle)
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O material property node counterpart.
pub struct MaterialPropertyNode {
    handle: ObjectHandle,
}

impl Named for MaterialPropertyNode {
    fn name(&self) -> Option<String> {
        self.name()
    }

    fn set_name(&self, name: &str) -> Result<()> {
        self.set_name(name)
    }
}

impl MaterialPropertyNode {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O material property node counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O material property node counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O material property node counterpart.
    pub fn new(inputs: &[&MaterialProperty], outputs: &[&MaterialProperty]) -> Result<Self> {
        let input_ptrs = inputs
            .iter()
            .map(|property| property.as_ptr())
            .collect::<Vec<_>>();
        let output_ptrs = outputs
            .iter()
            .map(|property| property.as_ptr())
            .collect::<Vec<_>>();
        let mut out_node = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_material_property_node_new(
                input_ptrs.as_ptr(),
                input_ptrs.len() as u64,
                output_ptrs.as_ptr(),
                output_ptrs.len() as u64,
                &mut out_node,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_node,
            "MDLMaterialPropertyNode",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property node counterpart.
    pub fn name(&self) -> Option<String> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        take_string(unsafe { ffi::mdl_named_name_string(self.handle.as_ptr()) })
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property node counterpart.
    pub fn set_name(&self, name: &str) -> Result<()> {
        let name = c_string(name)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_named_set_name(self.handle.as_ptr(), name.as_ptr()) };
        Ok(())
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property node counterpart.
    pub fn inputs(&self) -> Result<Vec<MaterialProperty>> {
        array_objects(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_material_property_node_inputs(self.handle.as_ptr()) },
            "MDLMaterialPropertyNode inputs",
            MaterialProperty::from_handle,
        )
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property node counterpart.
    pub fn outputs(&self) -> Result<Vec<MaterialProperty>> {
        array_objects(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_material_property_node_outputs(self.handle.as_ptr()) },
            "MDLMaterialPropertyNode outputs",
            MaterialProperty::from_handle,
        )
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O material property graph counterpart.
pub struct MaterialPropertyGraph {
    handle: ObjectHandle,
}

impl Named for MaterialPropertyGraph {
    fn name(&self) -> Option<String> {
        self.name()
    }

    fn set_name(&self, name: &str) -> Result<()> {
        self.set_name(name)
    }
}

impl MaterialPropertyGraph {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O material property graph counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O material property graph counterpart.
    pub fn new(
        nodes: &[&MaterialPropertyNode],
        connections: &[&MaterialPropertyConnection],
    ) -> Result<Self> {
        let node_ptrs = nodes.iter().map(|node| node.as_ptr()).collect::<Vec<_>>();
        let connection_ptrs = connections
            .iter()
            .map(|connection| connection.as_ptr())
            .collect::<Vec<_>>();
        let mut out_graph = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_material_property_graph_new(
                node_ptrs.as_ptr(),
                node_ptrs.len() as u64,
                connection_ptrs.as_ptr(),
                connection_ptrs.len() as u64,
                &mut out_graph,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_graph,
            "MDLMaterialPropertyGraph",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property graph counterpart.
    pub fn name(&self) -> Option<String> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        take_string(unsafe { ffi::mdl_named_name_string(self.handle.as_ptr()) })
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property graph counterpart.
    pub fn set_name(&self, name: &str) -> Result<()> {
        let name = c_string(name)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_named_set_name(self.handle.as_ptr(), name.as_ptr()) };
        Ok(())
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property graph counterpart.
    pub fn evaluate(&self) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_material_property_graph_evaluate(self.handle.as_ptr()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property graph counterpart.
    pub fn nodes(&self) -> Result<Vec<MaterialPropertyNode>> {
        array_objects(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_material_property_graph_nodes(self.handle.as_ptr()) },
            "MDLMaterialPropertyGraph nodes",
            MaterialPropertyNode::from_handle,
        )
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property graph counterpart.
    pub fn connections(&self) -> Result<Vec<MaterialPropertyConnection>> {
        array_objects(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_material_property_graph_connections(self.handle.as_ptr()) },
            "MDLMaterialPropertyGraph connections",
            MaterialPropertyConnection::from_handle,
        )
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O material property graph counterpart.
    pub fn as_node(&self) -> MaterialPropertyNode {
        MaterialPropertyNode::from_handle(self.handle.clone())
    }
}
