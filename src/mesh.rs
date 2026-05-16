use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::material::Material;
use crate::types::{BoundingBox, GeometryType, IndexBitDepth, MeshBufferInfo, VertexAttributeInfo};
use crate::util::{c_string, parse_json, required_handle, take_string};

#[derive(Debug, Clone)]
pub struct Mesh {
    handle: ObjectHandle,
}

impl Mesh {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new_box(
        extent: [f32; 3],
        segments: [u32; 3],
        inward_normals: bool,
        geometry_type: GeometryType,
    ) -> Result<Self> {
        let mut out_mesh = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_new_box(
                extent[0],
                extent[1],
                extent[2],
                segments[0],
                segments[1],
                segments[2],
                i32::from(inward_normals),
                geometry_type.as_raw(),
                &mut out_mesh,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(out_mesh, "MDLMesh box")?))
    }

    pub fn new_ellipsoid(
        extent: [f32; 3],
        segments: [u32; 2],
        inward_normals: bool,
        hemisphere: bool,
        geometry_type: GeometryType,
    ) -> Result<Self> {
        let mut out_mesh = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_new_ellipsoid(
                extent[0],
                extent[1],
                extent[2],
                segments[0],
                segments[1],
                i32::from(inward_normals),
                i32::from(hemisphere),
                geometry_type.as_raw(),
                &mut out_mesh,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_mesh,
            "MDLMesh ellipsoid",
        )?))
    }

    pub fn new_sphere(
        radius: f32,
        segments: [u32; 2],
        inward_normals: bool,
        geometry_type: GeometryType,
    ) -> Result<Self> {
        Self::new_ellipsoid(
            [radius, radius, radius],
            segments,
            inward_normals,
            false,
            geometry_type,
        )
    }

    pub fn new_cylinder(
        extent: [f32; 3],
        segments: [u32; 2],
        inward_normals: bool,
        top_cap: bool,
        bottom_cap: bool,
        geometry_type: GeometryType,
    ) -> Result<Self> {
        let mut out_mesh = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_new_cylinder(
                extent[0],
                extent[1],
                extent[2],
                segments[0],
                segments[1],
                i32::from(inward_normals),
                i32::from(top_cap),
                i32::from(bottom_cap),
                geometry_type.as_raw(),
                &mut out_mesh,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_mesh,
            "MDLMesh cylinder",
        )?))
    }

    pub fn new_plane(
        extent: [f32; 3],
        segments: [u32; 2],
        geometry_type: GeometryType,
    ) -> Result<Self> {
        let mut out_mesh = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_new_plane(
                extent[0],
                extent[1],
                extent[2],
                segments[0],
                segments[1],
                geometry_type.as_raw(),
                &mut out_mesh,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_mesh,
            "MDLMesh plane",
        )?))
    }

    pub fn new_icosahedron(
        extent: [f32; 3],
        inward_normals: bool,
        geometry_type: GeometryType,
    ) -> Result<Self> {
        let mut out_mesh = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_new_icosahedron(
                extent[0],
                extent[1],
                extent[2],
                i32::from(inward_normals),
                geometry_type.as_raw(),
                &mut out_mesh,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_mesh,
            "MDLMesh icosahedron",
        )?))
    }

    #[must_use]
    pub fn vertex_count(&self) -> usize {
        unsafe { ffi::mdl_mesh_vertex_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn vertex_buffer_count(&self) -> usize {
        unsafe { ffi::mdl_mesh_vertex_buffer_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn vertex_buffer(&self, index: usize) -> Option<MeshBuffer> {
        let ptr = unsafe { ffi::mdl_mesh_vertex_buffer_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBuffer::from_handle)
    }

    #[must_use]
    pub fn vertex_buffers(&self) -> Vec<MeshBuffer> {
        (0..self.vertex_buffer_count())
            .filter_map(|index| self.vertex_buffer(index))
            .collect()
    }

    #[must_use]
    pub fn submesh_count(&self) -> usize {
        unsafe { ffi::mdl_mesh_submesh_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn submesh(&self, index: usize) -> Option<Submesh> {
        let ptr = unsafe { ffi::mdl_mesh_submesh_at(self.handle.as_ptr(), index as u64) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Submesh::from_handle)
    }

    #[must_use]
    pub fn submeshes(&self) -> Vec<Submesh> {
        (0..self.submesh_count())
            .filter_map(|index| self.submesh(index))
            .collect()
    }

    #[must_use]
    pub fn bounding_box(&self) -> BoundingBox {
        let mut min = [0.0_f32; 3];
        let mut max = [0.0_f32; 3];
        unsafe {
            ffi::mdl_mesh_bounding_box(
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

    pub fn vertex_attribute_data_named(
        &self,
        attribute_name: &str,
    ) -> Result<Option<VertexAttributeData>> {
        let attribute_name = c_string(attribute_name)?;
        let ptr = unsafe {
            ffi::mdl_mesh_vertex_attribute_data(self.handle.as_ptr(), attribute_name.as_ptr())
        };
        Ok(unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(VertexAttributeData::from_handle))
    }
}

#[derive(Debug, Clone)]
pub struct MeshBuffer {
    handle: ObjectHandle,
}

impl MeshBuffer {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn info(&self) -> Result<MeshBufferInfo> {
        parse_json(
            unsafe { ffi::mdl_mesh_buffer_info_json(self.handle.as_ptr()) },
            "MDLMeshBuffer",
        )
    }

    pub fn bytes(&self) -> Result<Vec<u8>> {
        let info = self.info()?;
        let mut bytes = vec![0_u8; info.length];
        if bytes.is_empty() {
            return Ok(bytes);
        }
        let written = unsafe {
            ffi::mdl_mesh_buffer_copy_bytes(
                self.handle.as_ptr(),
                bytes.as_mut_ptr(),
                bytes.len() as u64,
            )
        } as usize;
        bytes.truncate(written);
        Ok(bytes)
    }
}

#[derive(Debug, Clone)]
pub struct VertexAttributeData {
    handle: ObjectHandle,
}

impl VertexAttributeData {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn info(&self) -> Result<VertexAttributeInfo> {
        parse_json(
            unsafe { ffi::mdl_vertex_attribute_data_info_json(self.handle.as_ptr()) },
            "MDLVertexAttributeData",
        )
    }

    pub fn bytes(&self) -> Result<Vec<u8>> {
        let info = self.info()?;
        let mut bytes = vec![0_u8; info.buffer_size];
        if bytes.is_empty() {
            return Ok(bytes);
        }
        let written = unsafe {
            ffi::mdl_vertex_attribute_data_copy_bytes(
                self.handle.as_ptr(),
                bytes.as_mut_ptr(),
                bytes.len() as u64,
            )
        } as usize;
        bytes.truncate(written);
        Ok(bytes)
    }
}

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
    pub fn material(&self) -> Option<Material> {
        let ptr = unsafe { ffi::mdl_submesh_material(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Material::from_handle)
    }
}
