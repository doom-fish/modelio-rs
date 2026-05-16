use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::material::Material;
use crate::mesh::MeshBuffer;
use crate::types::{GeometryType, IndexBitDepth};
use crate::util::{c_string, take_string};

#[derive(Debug, Clone)]
pub struct Submesh {
    handle: ObjectHandle,
}

impl Submesh {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        take_string(unsafe { ffi::mdl_submesh_name_string(self.handle.as_ptr()) })
    }

    pub fn set_name(&self, name: &str) -> Result<()> {
        let name = c_string(name)?;
        unsafe { ffi::mdl_submesh_set_name(self.handle.as_ptr(), name.as_ptr()) };
        Ok(())
    }

    #[must_use]
    pub fn index_count(&self) -> usize {
        unsafe { ffi::mdl_submesh_index_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn index_type(&self) -> Option<IndexBitDepth> {
        IndexBitDepth::from_raw(unsafe { ffi::mdl_submesh_index_type(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn geometry_type(&self) -> Option<GeometryType> {
        GeometryType::from_raw(unsafe { ffi::mdl_submesh_geometry_type(self.handle.as_ptr()) })
    }

    #[must_use]
    pub fn index_buffer(&self) -> Option<MeshBuffer> {
        let ptr = unsafe { ffi::mdl_submesh_index_buffer(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBuffer::from_handle)
    }

    #[must_use]
    pub fn index_buffer_as_type(&self, index_type: IndexBitDepth) -> Option<MeshBuffer> {
        let ptr = unsafe {
            ffi::mdl_submesh_index_buffer_as_type(self.handle.as_ptr(), index_type.as_raw())
        };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBuffer::from_handle)
    }

    #[must_use]
    pub fn material(&self) -> Option<Material> {
        let ptr = unsafe { ffi::mdl_submesh_material(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Material::from_handle)
    }

    pub fn set_material(&self, material: Option<&Material>) {
        unsafe {
            ffi::mdl_submesh_set_material(
                self.handle.as_ptr(),
                material.map_or(std::ptr::null_mut(), Material::as_ptr),
            );
        };
    }
}
