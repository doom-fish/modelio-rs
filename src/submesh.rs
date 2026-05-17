use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::material::Material;
use crate::mesh::MeshBuffer;
use crate::protocols::Named;
use crate::types::{GeometryType, IndexBitDepth};
use crate::util::{c_string, take_string};

#[derive(Debug, Clone)]
pub struct Submesh {
    handle: ObjectHandle,
}

impl Named for Submesh {
    fn name(&self) -> Option<String> {
        self.name()
    }

    fn set_name(&self, name: &str) -> Result<()> {
        self.set_name(name)
    }
}

impl Submesh {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
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

    #[must_use]
    pub fn topology(&self) -> Option<SubmeshTopology> {
        let ptr = unsafe { ffi::mdl_submesh_topology(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(SubmeshTopology::from_handle)
    }

    pub fn set_topology(&self, topology: Option<&SubmeshTopology>) {
        unsafe {
            ffi::mdl_submesh_set_topology(
                self.handle.as_ptr(),
                topology.map_or(std::ptr::null_mut(), SubmeshTopology::as_ptr),
            );
        };
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

#[derive(Debug, Clone)]
pub struct SubmeshTopology {
    handle: ObjectHandle,
}

impl SubmeshTopology {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    pub fn new(submesh: &Submesh) -> Result<Self> {
        let mut out_topology = std::ptr::null_mut();
        let mut out_error = std::ptr::null_mut();
        let status = unsafe {
            ffi::mdl_submesh_topology_new(submesh.as_ptr(), &mut out_topology, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(crate::util::required_handle(
            out_topology,
            "MDLSubmeshTopology",
        )?))
    }

    #[must_use]
    pub fn face_topology(&self) -> Option<MeshBuffer> {
        let ptr = unsafe { ffi::mdl_submesh_topology_face_topology(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBuffer::from_handle)
    }

    pub fn set_face_topology(&self, buffer: Option<&MeshBuffer>) {
        unsafe {
            ffi::mdl_submesh_topology_set_face_topology(
                self.handle.as_ptr(),
                buffer.map_or(std::ptr::null_mut(), MeshBuffer::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn face_count(&self) -> usize {
        unsafe { ffi::mdl_submesh_topology_face_count(self.handle.as_ptr()) as usize }
    }

    pub fn set_face_count(&self, count: usize) {
        unsafe { ffi::mdl_submesh_topology_set_face_count(self.handle.as_ptr(), count as u64) };
    }

    #[must_use]
    pub fn vertex_crease_indices(&self) -> Option<MeshBuffer> {
        let ptr = unsafe { ffi::mdl_submesh_topology_vertex_crease_indices(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBuffer::from_handle)
    }

    pub fn set_vertex_crease_indices(&self, buffer: Option<&MeshBuffer>) {
        unsafe {
            ffi::mdl_submesh_topology_set_vertex_crease_indices(
                self.handle.as_ptr(),
                buffer.map_or(std::ptr::null_mut(), MeshBuffer::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn vertex_creases(&self) -> Option<MeshBuffer> {
        let ptr = unsafe { ffi::mdl_submesh_topology_vertex_creases(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBuffer::from_handle)
    }

    pub fn set_vertex_creases(&self, buffer: Option<&MeshBuffer>) {
        unsafe {
            ffi::mdl_submesh_topology_set_vertex_creases(
                self.handle.as_ptr(),
                buffer.map_or(std::ptr::null_mut(), MeshBuffer::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn vertex_crease_count(&self) -> usize {
        unsafe { ffi::mdl_submesh_topology_vertex_crease_count(self.handle.as_ptr()) as usize }
    }

    pub fn set_vertex_crease_count(&self, count: usize) {
        unsafe {
            ffi::mdl_submesh_topology_set_vertex_crease_count(self.handle.as_ptr(), count as u64);
        };
    }

    #[must_use]
    pub fn edge_crease_indices(&self) -> Option<MeshBuffer> {
        let ptr = unsafe { ffi::mdl_submesh_topology_edge_crease_indices(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBuffer::from_handle)
    }

    pub fn set_edge_crease_indices(&self, buffer: Option<&MeshBuffer>) {
        unsafe {
            ffi::mdl_submesh_topology_set_edge_crease_indices(
                self.handle.as_ptr(),
                buffer.map_or(std::ptr::null_mut(), MeshBuffer::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn edge_creases(&self) -> Option<MeshBuffer> {
        let ptr = unsafe { ffi::mdl_submesh_topology_edge_creases(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBuffer::from_handle)
    }

    pub fn set_edge_creases(&self, buffer: Option<&MeshBuffer>) {
        unsafe {
            ffi::mdl_submesh_topology_set_edge_creases(
                self.handle.as_ptr(),
                buffer.map_or(std::ptr::null_mut(), MeshBuffer::as_ptr),
            );
        };
    }

    #[must_use]
    pub fn edge_crease_count(&self) -> usize {
        unsafe { ffi::mdl_submesh_topology_edge_crease_count(self.handle.as_ptr()) as usize }
    }

    pub fn set_edge_crease_count(&self, count: usize) {
        unsafe {
            ffi::mdl_submesh_topology_set_edge_crease_count(self.handle.as_ptr(), count as u64);
        };
    }
}
