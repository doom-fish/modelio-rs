use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::{AnimatedValueInfo, AnimatedValueInterpolation};
use crate::util::{parse_json, required_handle};

fn animated_info(handle: &ObjectHandle, context: &'static str) -> Result<AnimatedValueInfo> {
    parse_json(
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_animated_value_info_json(handle.as_ptr()) },
        context,
    )
}

fn animated_clear(handle: &ObjectHandle) {
    // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
    unsafe { ffi::mdl_animated_value_clear(handle.as_ptr()) };
}

fn animated_set_interpolation(handle: &ObjectHandle, interpolation: AnimatedValueInterpolation) {
    // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
    unsafe { ffi::mdl_animated_value_set_interpolation(handle.as_ptr(), interpolation.as_raw()) };
}

fn convert_vectors<const N: usize>(raw: &[f32]) -> Vec<[f32; N]> {
    raw.chunks_exact(N)
        .map(|chunk| {
            let mut value = [0.0_f32; N];
            value.copy_from_slice(chunk);
            value
        })
        .collect()
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O animated value counterpart.
pub struct AnimatedValue {
    handle: ObjectHandle,
}

impl AnimatedValue {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O animated value counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated value counterpart.
    pub fn info(&self) -> Result<AnimatedValueInfo> {
        animated_info(&self.handle, "MDLAnimatedValue")
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated value counterpart.
    pub fn clear(&self) {
        animated_clear(&self.handle);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated value counterpart.
    pub fn set_interpolation(&self, interpolation: AnimatedValueInterpolation) {
        animated_set_interpolation(&self.handle, interpolation);
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O animated scalar counterpart.
pub struct AnimatedScalar {
    handle: ObjectHandle,
}

impl AnimatedScalar {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O animated scalar counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O animated scalar counterpart.
    pub fn new() -> Result<Self> {
        let mut out_value = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_animated_scalar_new(&mut out_value, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_value,
            "MDLAnimatedScalar",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated scalar counterpart.
    pub fn info(&self) -> Result<AnimatedValueInfo> {
        animated_info(&self.handle, "MDLAnimatedScalar")
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated scalar counterpart.
    pub fn clear(&self) {
        animated_clear(&self.handle);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated scalar counterpart.
    pub fn set_interpolation(&self, interpolation: AnimatedValueInterpolation) {
        animated_set_interpolation(&self.handle, interpolation);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated scalar counterpart.
    pub fn set_float(&self, value: f32, time: f64) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_animated_scalar_set_float(self.handle.as_ptr(), value, time) };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated scalar counterpart.
    pub fn float_value(&self, time: f64) -> f32 {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_animated_scalar_float_value(self.handle.as_ptr(), time) }
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O animated vector2 counterpart.
pub struct AnimatedVector2 {
    handle: ObjectHandle,
}

impl AnimatedVector2 {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O animated vector2 counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O animated vector2 counterpart.
    pub fn new() -> Result<Self> {
        let mut out_value = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_animated_vector2_new(&mut out_value, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_value,
            "MDLAnimatedVector2",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector2 counterpart.
    pub fn info(&self) -> Result<AnimatedValueInfo> {
        animated_info(&self.handle, "MDLAnimatedVector2")
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector2 counterpart.
    pub fn clear(&self) {
        animated_clear(&self.handle);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector2 counterpart.
    pub fn set_interpolation(&self, interpolation: AnimatedValueInterpolation) {
        animated_set_interpolation(&self.handle, interpolation);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector2 counterpart.
    pub fn set_float2(&self, value: [f32; 2], time: f64) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_vector2_set_float2(self.handle.as_ptr(), value[0], value[1], time);
        };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector2 counterpart.
    pub fn float2_value(&self, time: f64) -> [f32; 2] {
        let mut value = [0.0_f32; 2];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_vector2_copy_float2_value(
                self.handle.as_ptr(),
                time,
                &mut value[0],
                &mut value[1],
            );
        };
        value
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O animated vector3 counterpart.
pub struct AnimatedVector3 {
    handle: ObjectHandle,
}

impl AnimatedVector3 {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O animated vector3 counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O animated vector3 counterpart.
    pub fn new() -> Result<Self> {
        let mut out_value = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_animated_vector3_new(&mut out_value, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_value,
            "MDLAnimatedVector3",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector3 counterpart.
    pub fn info(&self) -> Result<AnimatedValueInfo> {
        animated_info(&self.handle, "MDLAnimatedVector3")
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector3 counterpart.
    pub fn clear(&self) {
        animated_clear(&self.handle);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector3 counterpart.
    pub fn set_interpolation(&self, interpolation: AnimatedValueInterpolation) {
        animated_set_interpolation(&self.handle, interpolation);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector3 counterpart.
    pub fn set_float3(&self, value: [f32; 3], time: f64) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_vector3_set_float3(
                self.handle.as_ptr(),
                value[0],
                value[1],
                value[2],
                time,
            );
        };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector3 counterpart.
    pub fn float3_value(&self, time: f64) -> [f32; 3] {
        let mut value = [0.0_f32; 3];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_vector3_copy_float3_value(
                self.handle.as_ptr(),
                time,
                &mut value[0],
                &mut value[1],
                &mut value[2],
            );
        };
        value
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O animated vector4 counterpart.
pub struct AnimatedVector4 {
    handle: ObjectHandle,
}

impl AnimatedVector4 {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O animated vector4 counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O animated vector4 counterpart.
    pub fn new() -> Result<Self> {
        let mut out_value = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_animated_vector4_new(&mut out_value, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_value,
            "MDLAnimatedVector4",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector4 counterpart.
    pub fn info(&self) -> Result<AnimatedValueInfo> {
        animated_info(&self.handle, "MDLAnimatedVector4")
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector4 counterpart.
    pub fn clear(&self) {
        animated_clear(&self.handle);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector4 counterpart.
    pub fn set_interpolation(&self, interpolation: AnimatedValueInterpolation) {
        animated_set_interpolation(&self.handle, interpolation);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector4 counterpart.
    pub fn set_float4(&self, value: [f32; 4], time: f64) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_vector4_set_float4(
                self.handle.as_ptr(),
                value[0],
                value[1],
                value[2],
                value[3],
                time,
            );
        };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector4 counterpart.
    pub fn float4_value(&self, time: f64) -> [f32; 4] {
        let mut value = [0.0_f32; 4];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_vector4_copy_float4_value(
                self.handle.as_ptr(),
                time,
                &mut value[0],
                &mut value[1],
                &mut value[2],
                &mut value[3],
            );
        };
        value
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O animated quaternion counterpart.
pub struct AnimatedQuaternion {
    handle: ObjectHandle,
}

impl AnimatedQuaternion {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O animated quaternion counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O animated quaternion counterpart.
    pub fn new() -> Result<Self> {
        let mut out_value = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_animated_quaternion_new(&mut out_value, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_value,
            "MDLAnimatedQuaternion",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated quaternion counterpart.
    pub fn info(&self) -> Result<AnimatedValueInfo> {
        animated_info(&self.handle, "MDLAnimatedQuaternion")
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated quaternion counterpart.
    pub fn clear(&self) {
        animated_clear(&self.handle);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated quaternion counterpart.
    pub fn set_interpolation(&self, interpolation: AnimatedValueInterpolation) {
        animated_set_interpolation(&self.handle, interpolation);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated quaternion counterpart.
    pub fn set_float_quaternion(&self, value: [f32; 4], time: f64) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_quaternion_set_float(
                self.handle.as_ptr(),
                value[0],
                value[1],
                value[2],
                value[3],
                time,
            );
        };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated quaternion counterpart.
    pub fn float_quaternion_value(&self, time: f64) -> [f32; 4] {
        let mut raw = [0.0_f32; 4];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_quaternion_copy_float_value(
                self.handle.as_ptr(),
                time,
                raw.as_mut_ptr(),
            );
        };
        raw
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O animated matrix4x4 counterpart.
pub struct AnimatedMatrix4x4 {
    handle: ObjectHandle,
}

impl AnimatedMatrix4x4 {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O animated matrix4x4 counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O animated matrix4x4 counterpart.
    pub fn new() -> Result<Self> {
        let mut out_value = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_animated_matrix4x4_new(&mut out_value, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_value,
            "MDLAnimatedMatrix4x4",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated matrix4x4 counterpart.
    pub fn info(&self) -> Result<AnimatedValueInfo> {
        animated_info(&self.handle, "MDLAnimatedMatrix4x4")
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated matrix4x4 counterpart.
    pub fn clear(&self) {
        animated_clear(&self.handle);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated matrix4x4 counterpart.
    pub fn set_interpolation(&self, interpolation: AnimatedValueInterpolation) {
        animated_set_interpolation(&self.handle, interpolation);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated matrix4x4 counterpart.
    pub fn set_float4x4(&self, value: [f32; 16], time: f64) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_matrix4x4_set_float(self.handle.as_ptr(), value.as_ptr(), time);
        };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated matrix4x4 counterpart.
    pub fn float4x4_value(&self, time: f64) -> [f32; 16] {
        let mut raw = [0.0_f32; 16];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_matrix4x4_copy_float_value(
                self.handle.as_ptr(),
                time,
                raw.as_mut_ptr(),
            );
        };
        raw
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O animated scalar array counterpart.
pub struct AnimatedScalarArray {
    handle: ObjectHandle,
}

impl AnimatedScalarArray {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O animated scalar array counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O animated scalar array counterpart.
    pub fn new(element_count: usize) -> Result<Self> {
        let mut out_value = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_animated_scalar_array_new(element_count as u64, &mut out_value, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_value,
            "MDLAnimatedScalarArray",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated scalar array counterpart.
    pub fn info(&self) -> Result<AnimatedValueInfo> {
        animated_info(&self.handle, "MDLAnimatedScalarArray")
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated scalar array counterpart.
    pub fn clear(&self) {
        animated_clear(&self.handle);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated scalar array counterpart.
    pub fn set_interpolation(&self, interpolation: AnimatedValueInterpolation) {
        animated_set_interpolation(&self.handle, interpolation);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated scalar array counterpart.
    pub fn set_float_array(&self, values: &[f32], time: f64) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_scalar_array_set_float(
                self.handle.as_ptr(),
                values.as_ptr(),
                values.len() as u64,
                time,
            );
        };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated scalar array counterpart.
    pub fn float_array_at_time(&self, time: f64) -> Result<Vec<f32>> {
        let element_count = self.info()?.element_count.unwrap_or(0);
        if element_count == 0 {
            return Ok(Vec::new());
        }
        let mut values = vec![0.0_f32; element_count];
        // SAFETY: The unsafe operation is valid in this context.
        let written = unsafe {
            ffi::mdl_animated_scalar_array_copy_float_at_time(
                self.handle.as_ptr(),
                time,
                values.as_mut_ptr(),
                element_count as u64,
            )
        } as usize;
        values.truncate(written);
        Ok(values)
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O animated vector3array counterpart.
pub struct AnimatedVector3Array {
    handle: ObjectHandle,
}

impl AnimatedVector3Array {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O animated vector3array counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O animated vector3array counterpart.
    pub fn new(element_count: usize) -> Result<Self> {
        let mut out_value = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_animated_vector3_array_new(
                element_count as u64,
                &mut out_value,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_value,
            "MDLAnimatedVector3Array",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector3array counterpart.
    pub fn info(&self) -> Result<AnimatedValueInfo> {
        animated_info(&self.handle, "MDLAnimatedVector3Array")
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector3array counterpart.
    pub fn clear(&self) {
        animated_clear(&self.handle);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector3array counterpart.
    pub fn set_interpolation(&self, interpolation: AnimatedValueInterpolation) {
        animated_set_interpolation(&self.handle, interpolation);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector3array counterpart.
    pub fn set_float3_array(&self, values: &[[f32; 3]], time: f64) {
        let flattened = values
            .iter()
            .flat_map(|value| value.iter().copied())
            .collect::<Vec<_>>();
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_vector3_array_set_float(
                self.handle.as_ptr(),
                flattened.as_ptr(),
                values.len() as u64,
                time,
            );
        };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated vector3array counterpart.
    pub fn float3_array_at_time(&self, time: f64) -> Result<Vec<[f32; 3]>> {
        let element_count = self.info()?.element_count.unwrap_or(0);
        if element_count == 0 {
            return Ok(Vec::new());
        }
        let mut values = vec![0.0_f32; element_count * 3];
        // SAFETY: The unsafe operation is valid in this context.
        let written = unsafe {
            ffi::mdl_animated_vector3_array_copy_float_at_time(
                self.handle.as_ptr(),
                time,
                values.as_mut_ptr(),
                element_count as u64,
            )
        } as usize;
        values.truncate(written * 3);
        Ok(convert_vectors::<3>(&values))
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O animated quaternion array counterpart.
pub struct AnimatedQuaternionArray {
    handle: ObjectHandle,
}

impl AnimatedQuaternionArray {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O animated quaternion array counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O animated quaternion array counterpart.
    pub fn new(element_count: usize) -> Result<Self> {
        let mut out_value = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_animated_quaternion_array_new(
                element_count as u64,
                &mut out_value,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_value,
            "MDLAnimatedQuaternionArray",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated quaternion array counterpart.
    pub fn info(&self) -> Result<AnimatedValueInfo> {
        animated_info(&self.handle, "MDLAnimatedQuaternionArray")
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated quaternion array counterpart.
    pub fn clear(&self) {
        animated_clear(&self.handle);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated quaternion array counterpart.
    pub fn set_interpolation(&self, interpolation: AnimatedValueInterpolation) {
        animated_set_interpolation(&self.handle, interpolation);
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated quaternion array counterpart.
    pub fn set_float_quaternion_array(&self, values: &[[f32; 4]], time: f64) {
        let flattened = values
            .iter()
            .flat_map(|value| value.iter().copied())
            .collect::<Vec<_>>();
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_animated_quaternion_array_set_float(
                self.handle.as_ptr(),
                flattened.as_ptr(),
                values.len() as u64,
                time,
            );
        };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O animated quaternion array counterpart.
    pub fn float_quaternion_array_at_time(&self, time: f64) -> Result<Vec<[f32; 4]>> {
        let element_count = self.info()?.element_count.unwrap_or(0);
        if element_count == 0 {
            return Ok(Vec::new());
        }
        let mut values = vec![0.0_f32; element_count * 4];
        // SAFETY: The unsafe operation is valid in this context.
        let written = unsafe {
            ffi::mdl_animated_quaternion_array_copy_float_at_time(
                self.handle.as_ptr(),
                time,
                values.as_mut_ptr(),
                element_count as u64,
            )
        } as usize;
        values.truncate(written * 4);
        Ok(convert_vectors::<4>(&values))
    }
}
