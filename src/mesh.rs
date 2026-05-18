use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::object::Object;
use crate::submesh::Submesh;
use crate::types::{BoundingBox, GeometryType, MeshBufferInfo, VertexAttributeInfo};
use crate::util::{c_string, parse_json, required_handle};
use crate::vertex_attribute::VertexDescriptor;

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O mesh counterpart.
pub struct Mesh {
    handle: ObjectHandle,
}

impl Mesh {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O mesh counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O mesh counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh counterpart.
    pub fn new_box(
        extent: [f32; 3],
        segments: [u32; 3],
        inward_normals: bool,
        geometry_type: GeometryType,
    ) -> Result<Self> {
        let mut out_mesh = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh counterpart.
    pub fn new_ellipsoid(
        extent: [f32; 3],
        segments: [u32; 2],
        inward_normals: bool,
        hemisphere: bool,
        geometry_type: GeometryType,
    ) -> Result<Self> {
        let mut out_mesh = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh counterpart.
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

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh counterpart.
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
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh counterpart.
    pub fn new_plane(
        extent: [f32; 3],
        segments: [u32; 2],
        geometry_type: GeometryType,
    ) -> Result<Self> {
        let mut out_mesh = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh counterpart.
    pub fn new_icosahedron(
        extent: [f32; 3],
        inward_normals: bool,
        geometry_type: GeometryType,
    ) -> Result<Self> {
        let mut out_mesh = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh counterpart.
    pub fn vertex_count(&self) -> usize {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_mesh_vertex_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh counterpart.
    pub fn vertex_buffer_count(&self) -> usize {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_mesh_vertex_buffer_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh counterpart.
    pub fn vertex_buffer(&self, index: usize) -> Option<MeshBuffer> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_mesh_vertex_buffer_at(self.handle.as_ptr(), index as u64) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBuffer::from_handle)
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh counterpart.
    pub fn vertex_buffers(&self) -> Vec<MeshBuffer> {
        (0..self.vertex_buffer_count())
            .filter_map(|index| self.vertex_buffer(index))
            .collect()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh counterpart.
    pub fn submesh_count(&self) -> usize {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_mesh_submesh_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh counterpart.
    pub fn submesh(&self, index: usize) -> Option<Submesh> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_mesh_submesh_at(self.handle.as_ptr(), index as u64) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Submesh::from_handle)
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh counterpart.
    pub fn submeshes(&self) -> Vec<Submesh> {
        (0..self.submesh_count())
            .filter_map(|index| self.submesh(index))
            .collect()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh counterpart.
    pub fn bounding_box(&self) -> BoundingBox {
        let mut min = [0.0_f32; 3];
        let mut max = [0.0_f32; 3];
        // SAFETY: The unsafe operation is valid in this context.
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

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh counterpart.
    pub fn vertex_descriptor(&self) -> Option<VertexDescriptor> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_mesh_vertex_descriptor(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(VertexDescriptor::from_handle)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh counterpart.
    pub fn vertex_attribute_data_named(
        &self,
        attribute_name: &str,
    ) -> Result<Option<VertexAttributeData>> {
        let attribute_name = c_string(attribute_name)?;
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_mesh_vertex_attribute_data(self.handle.as_ptr(), attribute_name.as_ptr())
        };
        // SAFETY: The unsafe operation is valid in this context.
        Ok(unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(VertexAttributeData::from_handle))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh counterpart.
    pub fn as_object(&self) -> Object {
        Object::from_handle(self.handle.clone())
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O mesh buffer counterpart.
pub struct MeshBuffer {
    handle: ObjectHandle,
}

impl MeshBuffer {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O mesh buffer counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O mesh buffer counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer counterpart.
    pub fn info(&self) -> Result<MeshBufferInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_mesh_buffer_info_json(self.handle.as_ptr()) },
            "MDLMeshBuffer",
        )
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer counterpart.
    pub fn bytes(&self) -> Result<Vec<u8>> {
        let info = self.info()?;
        let mut bytes = vec![0_u8; info.length];
        if bytes.is_empty() {
            return Ok(bytes);
        }
        // SAFETY: The unsafe operation is valid in this context.
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
/// Wraps the corresponding Model I/O vertex attribute data counterpart.
pub struct VertexAttributeData {
    handle: ObjectHandle,
}

impl VertexAttributeData {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O vertex attribute data counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex attribute data counterpart.
    pub fn info(&self) -> Result<VertexAttributeInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_vertex_attribute_data_info_json(self.handle.as_ptr()) },
            "MDLVertexAttributeData",
        )
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex attribute data counterpart.
    pub fn bytes(&self) -> Result<Vec<u8>> {
        let info = self.info()?;
        let mut bytes = vec![0_u8; info.buffer_size];
        if bytes.is_empty() {
            return Ok(bytes);
        }
        // SAFETY: The unsafe operation is valid in this context.
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
