use std::panic::AssertUnwindSafe;
use std::ptr;

use crate::error::{ModelIoError, Result};
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::mesh::MeshBuffer;
use crate::types::MeshBufferType;
use crate::util::required_handle;

type MeshBufferAllocatorCallbackFn =
    dyn Fn(MeshBufferAllocatorEvent) -> MeshBufferAllocatorResponse + Send + Sync + 'static;

struct MeshBufferAllocatorCallback {
    callback: Box<MeshBufferAllocatorCallbackFn>,
}

#[derive(Debug, Clone)]
/// Describes one `MDLMeshBufferAllocator` protocol request routed into Rust.
pub enum MeshBufferAllocatorEvent {
    /// Allocates a new zone with the requested capacity.
    NewZone { capacity: usize },
    /// Allocates a zone sized for the requested buffer sizes and types.
    NewZoneForBuffers {
        sizes: Vec<usize>,
        types: Vec<MeshBufferType>,
    },
    /// Allocates a buffer with the requested length and type.
    NewBuffer {
        length: usize,
        buffer_type: MeshBufferType,
    },
    /// Allocates a buffer initialized from the provided bytes.
    NewBufferWithData {
        data: Vec<u8>,
        buffer_type: MeshBufferType,
    },
    /// Allocates a buffer from the provided zone.
    NewBufferFromZone {
        zone: Option<MeshBufferZone>,
        length: usize,
        buffer_type: MeshBufferType,
    },
    /// Allocates a buffer from the provided zone and initial bytes.
    NewBufferFromZoneWithData {
        zone: Option<MeshBufferZone>,
        data: Vec<u8>,
        buffer_type: MeshBufferType,
    },
}

#[derive(Debug, Clone)]
/// Returns the result of one `MDLMeshBufferAllocator` protocol request.
pub enum MeshBufferAllocatorResponse {
    /// Returns an optional zone result.
    Zone(Option<MeshBufferZone>),
    /// Returns an optional buffer result.
    Buffer(Option<MeshBuffer>),
    /// Indicates that the callback did not provide a custom result.
    None,
}

fn callback_response(
    context: *mut core::ffi::c_void,
    event: MeshBufferAllocatorEvent,
) -> Option<MeshBufferAllocatorResponse> {
    let context = (!context.is_null()).then_some(context.cast::<MeshBufferAllocatorCallback>())?;
    std::panic::catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The unsafe operation is valid in this context.
        (unsafe { &*context }.callback)(event)
    }))
    .ok()
}

fn zone_from_retained_ptr(ptr: *mut core::ffi::c_void) -> Option<MeshBufferZone> {
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBufferZone::from_handle)
}

fn zone_ptr_from_response(response: Option<MeshBufferAllocatorResponse>) -> *mut core::ffi::c_void {
    match response {
        Some(MeshBufferAllocatorResponse::Zone(Some(zone))) =>
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_object_retain(zone.as_ptr()) },
        _ => ptr::null_mut(),
    }
}

fn buffer_ptr_from_response(
    response: Option<MeshBufferAllocatorResponse>,
) -> *mut core::ffi::c_void {
    match response {
        Some(MeshBufferAllocatorResponse::Buffer(Some(buffer))) =>
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_object_retain(buffer.as_ptr()) },
        _ => ptr::null_mut(),
    }
}

fn mesh_buffer_type(raw: u32) -> Option<MeshBufferType> {
    MeshBufferType::from_raw(raw)
}

#[no_mangle]
pub extern "C" fn mdlx_mesh_buffer_allocator_new_zone(
    context: *mut core::ffi::c_void,
    capacity: u64,
) -> *mut core::ffi::c_void {
    zone_ptr_from_response(callback_response(
        context,
        MeshBufferAllocatorEvent::NewZone {
            capacity: capacity as usize,
        },
    ))
}

#[no_mangle]
pub extern "C" fn mdlx_mesh_buffer_allocator_new_zone_for_buffers_with_size(
    context: *mut core::ffi::c_void,
    sizes: *const u64,
    types: *const u32,
    count: u64,
) -> *mut core::ffi::c_void {
    let sizes = if count == 0 {
        Vec::new()
    } else if sizes.is_null() {
        return ptr::null_mut();
    } else {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { std::slice::from_raw_parts(sizes, count as usize) }
            .iter()
            .map(|size| *size as usize)
            .collect::<Vec<_>>()
    };
    let Some(types) = (if count == 0 {
        Some(Vec::new())
    } else if types.is_null() {
        None
    } else {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { std::slice::from_raw_parts(types, count as usize) }
            .iter()
            .copied()
            .map(mesh_buffer_type)
            .collect::<Option<Vec<_>>>()
    }) else {
        return ptr::null_mut();
    };
    zone_ptr_from_response(callback_response(
        context,
        MeshBufferAllocatorEvent::NewZoneForBuffers { sizes, types },
    ))
}

#[no_mangle]
pub extern "C" fn mdlx_mesh_buffer_allocator_new_buffer(
    context: *mut core::ffi::c_void,
    length: u64,
    buffer_type: u32,
) -> *mut core::ffi::c_void {
    let Some(buffer_type) = mesh_buffer_type(buffer_type) else {
        return ptr::null_mut();
    };
    buffer_ptr_from_response(callback_response(
        context,
        MeshBufferAllocatorEvent::NewBuffer {
            length: length as usize,
            buffer_type,
        },
    ))
}

#[no_mangle]
pub extern "C" fn mdlx_mesh_buffer_allocator_new_buffer_with_data(
    context: *mut core::ffi::c_void,
    bytes: *const u8,
    count: u64,
    buffer_type: u32,
) -> *mut core::ffi::c_void {
    let Some(buffer_type) = mesh_buffer_type(buffer_type) else {
        return ptr::null_mut();
    };
    let data = if count == 0 || bytes.is_null() {
        Vec::new()
    } else {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { std::slice::from_raw_parts(bytes, count as usize) }.to_vec()
    };
    buffer_ptr_from_response(callback_response(
        context,
        MeshBufferAllocatorEvent::NewBufferWithData { data, buffer_type },
    ))
}

#[no_mangle]
pub extern "C" fn mdlx_mesh_buffer_allocator_new_buffer_from_zone_length(
    context: *mut core::ffi::c_void,
    zone: *mut core::ffi::c_void,
    length: u64,
    buffer_type: u32,
) -> *mut core::ffi::c_void {
    let Some(buffer_type) = mesh_buffer_type(buffer_type) else {
        return ptr::null_mut();
    };
    buffer_ptr_from_response(callback_response(
        context,
        MeshBufferAllocatorEvent::NewBufferFromZone {
            zone: zone_from_retained_ptr(zone),
            length: length as usize,
            buffer_type,
        },
    ))
}

#[no_mangle]
pub extern "C" fn mdlx_mesh_buffer_allocator_new_buffer_from_zone_data(
    context: *mut core::ffi::c_void,
    zone: *mut core::ffi::c_void,
    bytes: *const u8,
    count: u64,
    buffer_type: u32,
) -> *mut core::ffi::c_void {
    let Some(buffer_type) = mesh_buffer_type(buffer_type) else {
        return ptr::null_mut();
    };
    let data = if count == 0 || bytes.is_null() {
        Vec::new()
    } else {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { std::slice::from_raw_parts(bytes, count as usize) }.to_vec()
    };
    buffer_ptr_from_response(callback_response(
        context,
        MeshBufferAllocatorEvent::NewBufferFromZoneWithData {
            zone: zone_from_retained_ptr(zone),
            data,
            buffer_type,
        },
    ))
}

#[no_mangle]
pub extern "C" fn mdlx_mesh_buffer_allocator_release(context: *mut core::ffi::c_void) {
    if context.is_null() {
        return;
    }
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { drop(Box::from_raw(context.cast::<MeshBufferAllocatorCallback>())) };
}

fn release_callback_context(context: *mut core::ffi::c_void) {
    mdlx_mesh_buffer_allocator_release(context);
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O mesh buffer map counterpart.
pub struct MeshBufferMap {
    handle: ObjectHandle,
    length: usize,
}

impl MeshBufferMap {
    fn from_handle(handle: ObjectHandle, length: usize) -> Self {
        Self { handle, length }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O mesh buffer map counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer map counterpart.
    pub fn length(&self) -> usize {
        self.length
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer map counterpart.
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0_u8; self.length];
        if bytes.is_empty() {
            return bytes;
        }
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer map counterpart.
    pub fn write(&self, offset: usize, bytes: &[u8]) -> usize {
        // SAFETY: The unsafe operation is valid in this context.
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
/// Wraps the corresponding Model I/O mesh buffer allocator counterpart.
pub struct MeshBufferAllocator {
    handle: ObjectHandle,
}

impl MeshBufferAllocator {
    /// Wraps a Rust callback as the corresponding Model I/O mesh buffer allocator protocol counterpart.
    pub fn new<F>(callback: F) -> Result<Self>
    where
        F: Fn(MeshBufferAllocatorEvent) -> MeshBufferAllocatorResponse + Send + Sync + 'static,
    {
        let callback = Box::new(MeshBufferAllocatorCallback {
            callback: Box::new(callback),
        });
        let callback_ptr = Box::into_raw(callback).cast::<core::ffi::c_void>();
        let mut out_allocator = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_mesh_buffer_allocator_new_with_callback(
                callback_ptr,
                &mut out_allocator,
                &mut out_error,
            )
        };
        if let Err(error) = crate::util::status_result(status, out_error) {
            release_callback_context(callback_ptr);
            return Err(error);
        }
        match required_handle(out_allocator, "MDLMeshBufferAllocator") {
            Ok(handle) => Ok(Self::from_handle(handle)),
            Err(error) => {
                release_callback_context(callback_ptr);
                Err(error)
            }
        }
    }

    /// Builds this wrapper from the retained handle of the wrapped Model I/O mesh buffer allocator counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O mesh buffer allocator counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh buffer allocator counterpart.
    pub fn new_zone(&self, capacity: usize) -> Result<MeshBufferZone> {
        let mut out_zone = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh buffer allocator counterpart.
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
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh buffer allocator counterpart.
    pub fn new_buffer(&self, length: usize, buffer_type: MeshBufferType) -> Result<MeshBuffer> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh buffer allocator counterpart.
    pub fn new_buffer_with_data(
        &self,
        data: &[u8],
        buffer_type: MeshBufferType,
    ) -> Result<MeshBuffer> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh buffer allocator counterpart.
    pub fn new_buffer_from_zone(
        &self,
        zone: Option<&MeshBufferZone>,
        length: usize,
        buffer_type: MeshBufferType,
    ) -> Result<Option<MeshBuffer>> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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
        // SAFETY: The unsafe operation is valid in this context.
        Ok(unsafe { ObjectHandle::from_retained_ptr(out_buffer) }.map(MeshBuffer::from_handle))
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh buffer allocator counterpart.
    pub fn new_buffer_from_zone_with_data(
        &self,
        zone: Option<&MeshBufferZone>,
        data: &[u8],
        buffer_type: MeshBufferType,
    ) -> Result<Option<MeshBuffer>> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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
        // SAFETY: The unsafe operation is valid in this context.
        Ok(unsafe { ObjectHandle::from_retained_ptr(out_buffer) }.map(MeshBuffer::from_handle))
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O mesh buffer zone counterpart.
pub struct MeshBufferZone {
    handle: ObjectHandle,
}

impl MeshBufferZone {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O mesh buffer zone counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O mesh buffer zone counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer zone counterpart.
    pub fn capacity(&self) -> usize {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_mesh_buffer_zone_capacity(self.as_ptr()) as usize }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer zone counterpart.
    pub fn allocator(&self) -> Option<MeshBufferAllocator> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_mesh_buffer_zone_allocator(self.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBufferAllocator::from_handle)
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer zone counterpart.
    pub fn as_default(&self) -> Option<MeshBufferZoneDefault> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        (unsafe { ffi::mdl_mesh_buffer_zone_is_default(self.as_ptr()) != 0 })
            .then(|| MeshBufferZoneDefault::from_handle(self.handle.clone()))
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O mesh buffer zone default counterpart.
pub struct MeshBufferZoneDefault {
    handle: ObjectHandle,
}

impl MeshBufferZoneDefault {
    fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh buffer zone default counterpart.
    pub fn new() -> Result<Self> {
        let mut out_zone = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status =
            // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
            unsafe { ffi::mdl_mesh_buffer_zone_default_new(&mut out_zone, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_zone,
            "MDLMeshBufferZoneDefault",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer zone default counterpart.
    pub fn capacity(&self) -> usize {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_mesh_buffer_zone_capacity(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer zone default counterpart.
    pub fn allocator(&self) -> Option<MeshBufferAllocator> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_mesh_buffer_zone_allocator(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBufferAllocator::from_handle)
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer zone default counterpart.
    pub fn as_mesh_buffer_zone(&self) -> MeshBufferZone {
        MeshBufferZone::from_handle(self.handle.clone())
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O mesh buffer data counterpart.
pub struct MeshBufferData {
    handle: ObjectHandle,
}

impl MeshBufferData {
    fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh buffer data counterpart.
    pub fn new(length: usize, buffer_type: MeshBufferType) -> Result<Self> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer data counterpart.
    pub fn from_bytes(data: &[u8], buffer_type: MeshBufferType) -> Result<Self> {
        let mut out_buffer = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
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
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer data counterpart.
    pub fn data(&self) -> Vec<u8> {
        let info = self.as_mesh_buffer().info().ok();
        let mut bytes = vec![0_u8; info.map_or(0, |buffer| buffer.length)];
        if bytes.is_empty() {
            return bytes;
        }
        // SAFETY: The unsafe operation is valid in this context.
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

    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer data counterpart.
    pub fn map(&self) -> Result<MeshBufferMap> {
        self.as_mesh_buffer().map()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer data counterpart.
    pub fn as_mesh_buffer(&self) -> MeshBuffer {
        MeshBuffer::from_handle(self.handle.clone())
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O mesh buffer data allocator counterpart.
pub struct MeshBufferDataAllocator {
    handle: ObjectHandle,
}

impl MeshBufferDataAllocator {
    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh buffer data allocator counterpart.
    pub fn new() -> Result<Self> {
        let mut out_allocator = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        let status =
            // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
            unsafe { ffi::mdl_mesh_buffer_data_allocator_new(&mut out_allocator, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self {
            handle: required_handle(out_allocator, "MDLMeshBufferDataAllocator")?,
        })
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer data allocator counterpart.
    pub fn as_mesh_buffer_allocator(&self) -> MeshBufferAllocator {
        MeshBufferAllocator::from_handle(self.handle.clone())
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O mesh buffer data allocator counterpart.
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
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer counterpart.
    pub fn fill_data(&self, data: &[u8], offset: usize) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_mesh_buffer_fill_data(
                self.as_ptr(),
                data.as_ptr(),
                data.len() as u64,
                offset as u64,
            );
        }
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer counterpart.
    pub fn map(&self) -> Result<MeshBufferMap> {
        let length = self.info()?.length;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_mesh_buffer_map(self.as_ptr()) };
        Ok(MeshBufferMap::from_handle(
            required_handle(ptr, "MDLMeshBufferMap")?,
            length,
        ))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer counterpart.
    pub fn allocator(&self) -> Option<MeshBufferAllocator> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_mesh_buffer_allocator(self.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBufferAllocator::from_handle)
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer counterpart.
    pub fn zone(&self) -> Option<MeshBufferZone> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_mesh_buffer_zone(self.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(MeshBufferZone::from_handle)
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O mesh buffer counterpart.
    pub fn as_data_buffer(&self) -> Option<MeshBufferData> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        if unsafe { ffi::mdl_mesh_buffer_is_data(self.as_ptr()) == 0 } {
            return None;
        }
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let retained = unsafe { ffi::mdl_object_retain(self.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(retained) }.map(MeshBufferData::from_handle)
    }
}
