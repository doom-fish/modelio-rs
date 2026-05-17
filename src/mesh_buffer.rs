use std::ptr;

use crate::error::{ModelIoError, Result};
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::mesh::MeshBuffer;
use crate::types::MeshBufferType;
use crate::util::required_handle;

#[derive(Debug, Clone)]
pub struct MeshBufferMap {
    handle: ObjectHandle,
    length: usize,
}

impl MeshBufferMap {
    fn from_handle(handle: ObjectHandle, length: usize) -> Self {
        Self { handle, length }
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    #[must_use]
    pub fn length(&self) -> usize {
        self.length
    }

    #[must_use]
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0_u8; self.length];
        if bytes.is_empty() {
            return bytes;
        }
        let written = unsafe {
            ffi::mdl_mesh_buffer_map_copy_bytes(
                self.as_ptr(),
                self.length as u64,
                bytes.as_mut_ptr(),
                bytes.len() as u64,
            )
        } as usize;
        bytes.truncate(written);
        bytes
    }

    pub fn write(&self, offset: usize, bytes: &[u8]) -> usize {
        unsafe {
            ffi::mdl_mesh_buffer_map_write_bytes(
                self.as_ptr(),
                self.length as u64,
                bytes.as_ptr(),
                bytes.len() as u64,
                offset as u64,
            ) as usize
        }
    }
}

#[derive(Debug, Clone)]
pub struct MeshBufferAllocator {
    handle: ObjectHandle,
}

impl MeshBufferAllocator {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    pub fn new_zone(&self, capacity: usize) -> Result<MeshBufferZone> {
        let mut out_zone = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_buffer_allocator_new_zone(
                self.as_ptr(),
                capacity as u64,
                &mut out_zone,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(MeshBufferZone::from_handle(required_handle(
            out_zone,
            "MDLMeshBufferZone",
        )?))
    }

    pub fn new_zone_for_buffers(
        &self,
        sizes: &[usize],
        types: &[MeshBufferType],
    ) -> Result<MeshBufferZone> {
        if sizes.len() != types.len() {
            return Err(ModelIoError::new(
                ffi::status::INVALID_ARGUMENT,
                "mesh buffer sizes and types must have the same length",
            ));
        }
        let raw_sizes = sizes.iter().map(|size| *size as u64).collect::<Vec<_>>();
        let raw_types = types
            .iter()
            .map(|buffer_type| buffer_type.as_raw())
            .collect::<Vec<_>>();
        let mut out_zone = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_buffer_allocator_new_zone_for_buffers_with_size(
                self.as_ptr(),
                raw_sizes.as_ptr(),
                raw_types.as_ptr(),
                raw_sizes.len() as u64,
                &mut out_zone,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(MeshBufferZone::from_handle(required_handle(
            out_zone,
            "MDLMeshBufferZone",
        )?))
    }

    pub fn new_buffer(&self, length: usize, buffer_type: MeshBufferType) -> Result<MeshBuffer> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_buffer_allocator_new_buffer(
                self.as_ptr(),
                length as u64,
                buffer_type.as_raw(),
                &mut out_buffer,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(MeshBuffer::from_handle(required_handle(
            out_buffer,
            "MDLMeshBuffer",
        )?))
    }

    pub fn new_buffer_with_data(
        &self,
        data: &[u8],
        buffer_type: MeshBufferType,
    ) -> Result<MeshBuffer> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_buffer_allocator_new_buffer_with_data(
                self.as_ptr(),
                data.as_ptr(),
                data.len() as u64,
                buffer_type.as_raw(),
                &mut out_buffer,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(MeshBuffer::from_handle(required_handle(
            out_buffer,
            "MDLMeshBuffer",
        )?))
    }

    pub fn new_buffer_from_zone(
        &self,
        zone: Option<&MeshBufferZone>,
        length: usize,
        buffer_type: MeshBufferType,
    ) -> Result<Option<MeshBuffer>> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_buffer_allocator_new_buffer_from_zone_length(
                self.as_ptr(),
                zone.map_or(ptr::null_mut(), MeshBufferZone::as_ptr),
                length as u64,
                buffer_type.as_raw(),
                &mut out_buffer,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(unsafe { ObjectHandle::from_retained_ptr(out_buffer) }.map(MeshBuffer::from_handle))
    }

    pub fn new_buffer_from_zone_with_data(
        &self,
        zone: Option<&MeshBufferZone>,
        data: &[u8],
        buffer_type: MeshBufferType,
    ) -> Result<Option<MeshBuffer>> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_buffer_allocator_new_buffer_from_zone_data(
                self.as_ptr(),
                zone.map_or(ptr::null_mut(), MeshBufferZone::as_ptr),
                data.as_ptr(),
                data.len() as u64,
                buffer_type.as_raw(),
                &mut out_buffer,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(unsafe { ObjectHandle::from_retained_ptr(out_buffer) }.map(MeshBuffer::from_handle))
    }
}

#[derive(Debug, Clone)]
pub struct MeshBufferZone {
    handle: ObjectHandle,
}

impl MeshBufferZone {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        unsafe { ffi::mdl_mesh_buffer_zone_capacity(self.as_ptr()) as usize }
    }

    #[must_use]
    pub fn allocator(&self) -> Option<MeshBufferAllocator> {
        let ptr = unsafe { ffi::mdl_mesh_buffer_zone_allocator(self.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBufferAllocator::from_handle)
    }

    #[must_use]
    pub fn as_default(&self) -> Option<MeshBufferZoneDefault> {
        (unsafe { ffi::mdl_mesh_buffer_zone_is_default(self.as_ptr()) != 0 })
            .then(|| MeshBufferZoneDefault::from_handle(self.handle.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct MeshBufferZoneDefault {
    handle: ObjectHandle,
}

impl MeshBufferZoneDefault {
    fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new() -> Result<Self> {
        let mut out_zone = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status =
            unsafe { ffi::mdl_mesh_buffer_zone_default_new(&mut out_zone, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_zone,
            "MDLMeshBufferZoneDefault",
        )?))
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        unsafe { ffi::mdl_mesh_buffer_zone_capacity(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    pub fn allocator(&self) -> Option<MeshBufferAllocator> {
        let ptr = unsafe { ffi::mdl_mesh_buffer_zone_allocator(self.handle.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBufferAllocator::from_handle)
    }

    #[must_use]
    pub fn as_mesh_buffer_zone(&self) -> MeshBufferZone {
        MeshBufferZone::from_handle(self.handle.clone())
    }
}

#[derive(Debug, Clone)]
pub struct MeshBufferData {
    handle: ObjectHandle,
}

impl MeshBufferData {
    fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn new(length: usize, buffer_type: MeshBufferType) -> Result<Self> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_buffer_data_new(
                length as u64,
                buffer_type.as_raw(),
                &mut out_buffer,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_buffer,
            "MDLMeshBufferData",
        )?))
    }

    pub fn from_bytes(data: &[u8], buffer_type: MeshBufferType) -> Result<Self> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_mesh_buffer_data_new_with_bytes(
                data.as_ptr(),
                data.len() as u64,
                buffer_type.as_raw(),
                &mut out_buffer,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_buffer,
            "MDLMeshBufferData",
        )?))
    }

    #[must_use]
    pub fn data(&self) -> Vec<u8> {
        let info = self.as_mesh_buffer().info().ok();
        let mut bytes = vec![0_u8; info.map_or(0, |buffer| buffer.length)];
        if bytes.is_empty() {
            return bytes;
        }
        let written = unsafe {
            ffi::mdl_mesh_buffer_data_copy_data(
                self.handle.as_ptr(),
                bytes.as_mut_ptr(),
                bytes.len() as u64,
            )
        } as usize;
        bytes.truncate(written);
        bytes
    }

    pub fn map(&self) -> Result<MeshBufferMap> {
        self.as_mesh_buffer().map()
    }

    #[must_use]
    pub fn as_mesh_buffer(&self) -> MeshBuffer {
        MeshBuffer::from_handle(self.handle.clone())
    }
}

#[derive(Debug, Clone)]
pub struct MeshBufferDataAllocator {
    handle: ObjectHandle,
}

impl MeshBufferDataAllocator {
    pub fn new() -> Result<Self> {
        let mut out_allocator = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status =
            unsafe { ffi::mdl_mesh_buffer_data_allocator_new(&mut out_allocator, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self {
            handle: required_handle(out_allocator, "MDLMeshBufferDataAllocator")?,
        })
    }

    #[must_use]
    pub fn as_mesh_buffer_allocator(&self) -> MeshBufferAllocator {
        MeshBufferAllocator::from_handle(self.handle.clone())
    }

    pub fn new_default_zone(&self, capacity: usize) -> Result<MeshBufferZoneDefault> {
        self.as_mesh_buffer_allocator()
            .new_zone(capacity)?
            .as_default()
            .ok_or_else(|| {
                ModelIoError::new(
                    ffi::status::NULL_RESULT,
                    "MDLMeshBufferDataAllocator zone was not MDLMeshBufferZoneDefault",
                )
            })
    }
}

impl MeshBuffer {
    pub fn fill_data(&self, data: &[u8], offset: usize) {
        unsafe {
            ffi::mdl_mesh_buffer_fill_data(
                self.as_ptr(),
                data.as_ptr(),
                data.len() as u64,
                offset as u64,
            );
        }
    }

    pub fn map(&self) -> Result<MeshBufferMap> {
        let length = self.info()?.length;
        let ptr = unsafe { ffi::mdl_mesh_buffer_map(self.as_ptr()) };
        Ok(MeshBufferMap::from_handle(
            required_handle(ptr, "MDLMeshBufferMap")?,
            length,
        ))
    }

    #[must_use]
    pub fn allocator(&self) -> Option<MeshBufferAllocator> {
        let ptr = unsafe { ffi::mdl_mesh_buffer_allocator(self.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBufferAllocator::from_handle)
    }

    #[must_use]
    pub fn zone(&self) -> Option<MeshBufferZone> {
        let ptr = unsafe { ffi::mdl_mesh_buffer_zone(self.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBufferZone::from_handle)
    }

    #[must_use]
    pub fn as_data_buffer(&self) -> Option<MeshBufferData> {
        if unsafe { ffi::mdl_mesh_buffer_is_data(self.as_ptr()) == 0 } {
            return None;
        }
        let retained = unsafe { ffi::mdl_object_retain(self.as_ptr()) };
        unsafe { ObjectHandle::from_retained_ptr(retained) }.map(MeshBufferData::from_handle)
    }
}
