use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::{VertexAttributeDescriptorInfo, VertexDescriptorInfo};
use crate::util::{c_string, parse_json, required_handle};

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O vertex attribute counterpart.
pub struct VertexAttribute {
    handle: ObjectHandle,
}

impl VertexAttribute {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O vertex attribute counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O vertex attribute counterpart.
    pub fn new(name: &str, format: u32, offset: usize, buffer_index: usize) -> Result<Self> {
        let name = c_string(name)?;
        let mut out_attribute = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_vertex_attribute_new(
                name.as_ptr(),
                format,
                offset as u64,
                buffer_index as u64,
                &mut out_attribute,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_attribute,
            "MDLVertexAttribute",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex attribute counterpart.
    pub fn info(&self) -> Result<VertexAttributeDescriptorInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_vertex_attribute_info_json(self.handle.as_ptr()) },
            "MDLVertexAttribute",
        )
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex attribute counterpart.
    pub fn set_name(&self, name: &str) -> Result<()> {
        let name = c_string(name)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_vertex_attribute_set_name(self.handle.as_ptr(), name.as_ptr()) };
        Ok(())
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex attribute counterpart.
    pub fn set_format(&self, format: u32) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_vertex_attribute_set_format(self.handle.as_ptr(), format) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex attribute counterpart.
    pub fn set_offset(&self, offset: usize) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_vertex_attribute_set_offset(self.handle.as_ptr(), offset as u64) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex attribute counterpart.
    pub fn set_buffer_index(&self, buffer_index: usize) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_vertex_attribute_set_buffer_index(self.handle.as_ptr(), buffer_index as u64);
        };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex attribute counterpart.
    pub fn set_time(&self, time: f64) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_vertex_attribute_set_time(self.handle.as_ptr(), time) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex attribute counterpart.
    pub fn set_initialization_value(&self, value: [f32; 4]) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_vertex_attribute_set_initialization_value(
                self.handle.as_ptr(),
                value[0],
                value[1],
                value[2],
                value[3],
            );
        };
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O vertex buffer layout counterpart.
pub struct VertexBufferLayout {
    handle: ObjectHandle,
}

impl VertexBufferLayout {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O vertex buffer layout counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O vertex buffer layout counterpart.
    pub fn new(stride: usize) -> Result<Self> {
        let mut out_layout = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_vertex_buffer_layout_new(stride as u64, &mut out_layout, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_layout,
            "MDLVertexBufferLayout",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex buffer layout counterpart.
    pub fn stride(&self) -> usize {
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        unsafe { ffi::mdl_vertex_buffer_layout_stride(self.handle.as_ptr()) as usize }
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex buffer layout counterpart.
    pub fn set_stride(&self, stride: usize) {
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        unsafe { ffi::mdl_vertex_buffer_layout_set_stride(self.handle.as_ptr(), stride as u64) };
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O vertex descriptor counterpart.
pub struct VertexDescriptor {
    handle: ObjectHandle,
}

impl VertexDescriptor {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O vertex descriptor counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn copy(&self) -> Result<Self> {
        let mut out_descriptor = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_vertex_descriptor_new_copy(
                self.handle.as_ptr(),
                &mut out_descriptor,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_descriptor,
            "MDLVertexDescriptor",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn info(&self) -> Result<VertexDescriptorInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_vertex_descriptor_info_json(self.handle.as_ptr()) },
            "MDLVertexDescriptor",
        )
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn attribute_count(&self) -> usize {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_vertex_descriptor_attribute_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn attribute_at(&self, index: usize) -> Option<VertexAttribute> {
        let ptr =
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_vertex_descriptor_attribute_at(self.handle.as_ptr(), index as u64) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(VertexAttribute::from_handle)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn attribute_named(&self, name: &str) -> Result<Option<VertexAttribute>> {
        let name = c_string(name)?;
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_vertex_descriptor_attribute_named(self.handle.as_ptr(), name.as_ptr())
        };
        // SAFETY: The unsafe operation is valid in this context.
        Ok(unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(VertexAttribute::from_handle))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn attributes(&self) -> Vec<VertexAttribute> {
        (0..self.attribute_count())
            .filter_map(|index| self.attribute_at(index))
            .collect()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn layout_count(&self) -> usize {
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        unsafe { ffi::mdl_vertex_descriptor_layout_count(self.handle.as_ptr()) as usize }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn layout_at(&self, index: usize) -> Option<VertexBufferLayout> {
        let ptr =
            // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
            unsafe { ffi::mdl_vertex_descriptor_layout_at(self.handle.as_ptr(), index as u64) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(VertexBufferLayout::from_handle)
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn layouts(&self) -> Vec<VertexBufferLayout> {
        (0..self.layout_count())
            .filter_map(|index| self.layout_at(index))
            .collect()
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn reset(&self) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_vertex_descriptor_reset(self.handle.as_ptr()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn set_packed_offsets(&self) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_vertex_descriptor_set_packed_offsets(self.handle.as_ptr()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O vertex descriptor counterpart.
    pub fn set_packed_strides(&self) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_vertex_descriptor_set_packed_strides(self.handle.as_ptr()) };
    }
}
