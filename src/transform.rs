use std::ptr;

use crate::animated_value_types::{
    AnimatedMatrix4x4, AnimatedQuaternion, AnimatedScalar, AnimatedValue, AnimatedVector3,
};
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::object::Object;
use crate::protocols::Component;
use crate::types::TransformOpRotationOrder;
use crate::util::{c_string, required_handle, take_string};

fn copy_matrix(
    handle: *mut core::ffi::c_void,
    getter: unsafe extern "C" fn(*mut core::ffi::c_void, *mut f32),
) -> [f32; 16] {
    let mut matrix = [0.0_f32; 16];
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { getter(handle, matrix.as_mut_ptr()) };
    matrix
}

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
/// Wraps the corresponding Model I/O transform component counterpart.
pub struct TransformComponent {
    handle: ObjectHandle,
}

impl TransformComponent {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O transform component counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O transform component counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform component counterpart.
    pub fn matrix(&self) -> [f32; 16] {
        copy_matrix(self.handle.as_ptr(), ffi::mdl_transform_component_matrix)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform component counterpart.
    pub fn set_matrix(&self, matrix: [f32; 16]) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_transform_component_set_matrix(self.handle.as_ptr(), matrix.as_ptr()) };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform component counterpart.
    pub fn resets_transform(&self) -> bool {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_transform_component_resets_transform(self.handle.as_ptr()) != 0 }
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform component counterpart.
    pub fn set_resets_transform(&self, resets_transform: bool) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_component_set_resets_transform(
                self.handle.as_ptr(),
                i32::from(resets_transform),
            );
        }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform component counterpart.
    pub fn minimum_time(&self) -> f64 {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_transform_component_minimum_time(self.handle.as_ptr()) }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform component counterpart.
    pub fn maximum_time(&self) -> f64 {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_transform_component_maximum_time(self.handle.as_ptr()) }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform component counterpart.
    pub fn key_times(&self) -> Vec<f64> {
        let count =
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_transform_component_key_time_count(self.handle.as_ptr()) as usize };
        let mut values = vec![0.0_f64; count];
        if values.is_empty() {
            return values;
        }
        // SAFETY: The unsafe operation is valid in this context.
        let written = unsafe {
            ffi::mdl_transform_component_copy_key_times(
                self.handle.as_ptr(),
                values.as_mut_ptr(),
                values.len() as u64,
            )
        } as usize;
        values.truncate(written);
        values
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform component counterpart.
    pub fn local_transform_at_time(&self, time: f64) -> [f32; 16] {
        let mut matrix = [0.0_f32; 16];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_component_local_transform_at_time(
                self.handle.as_ptr(),
                time,
                matrix.as_mut_ptr(),
            );
        }
        matrix
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform component counterpart.
    pub fn as_transform(&self) -> Option<Transform> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        (unsafe { ffi::mdl_transform_component_is_transform(self.handle.as_ptr()) != 0 })
            .then(|| Transform::from_handle(self.handle.clone()))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform component counterpart.
    pub fn as_transform_stack(&self) -> Option<TransformStack> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        (unsafe { ffi::mdl_transform_component_is_transform_stack(self.handle.as_ptr()) != 0 })
            .then(|| TransformStack::from_handle(self.handle.clone()))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform component counterpart.
    pub fn global_transform_with_object(object: &Object, time: f64) -> [f32; 16] {
        let mut matrix = [0.0_f32; 16];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_component_global_transform_with_object(
                object.as_ptr(),
                time,
                matrix.as_mut_ptr(),
            );
        }
        matrix
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O transform counterpart.
pub struct Transform {
    handle: ObjectHandle,
}

impl Component for Transform {}

impl Transform {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O transform counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O transform counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O transform counterpart.
    pub fn new() -> Result<Self> {
        let mut out_transform = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_transform_new(&mut out_transform, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_transform,
            "MDLTransform",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn from_component(component: &TransformComponent) -> Result<Self> {
        let mut out_transform = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_transform_new_with_component(
                component.as_ptr(),
                &mut out_transform,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_transform,
            "MDLTransform",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn from_component_with_resets_transform(
        component: &TransformComponent,
        resets_transform: bool,
    ) -> Result<Self> {
        let mut out_transform = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_transform_new_with_component_resets_transform(
                component.as_ptr(),
                i32::from(resets_transform),
                &mut out_transform,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_transform,
            "MDLTransform",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn from_matrix(matrix: [f32; 16]) -> Result<Self> {
        let mut out_transform = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_transform_new_with_matrix(matrix.as_ptr(), &mut out_transform, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_transform,
            "MDLTransform",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn from_matrix_with_resets_transform(
        matrix: [f32; 16],
        resets_transform: bool,
    ) -> Result<Self> {
        let mut out_transform = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_transform_new_with_matrix_resets_transform(
                matrix.as_ptr(),
                i32::from(resets_transform),
                &mut out_transform,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_transform,
            "MDLTransform",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn matrix(&self) -> [f32; 16] {
        self.as_transform_component().matrix()
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_matrix(&self, matrix: [f32; 16]) {
        self.as_transform_component().set_matrix(matrix);
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn resets_transform(&self) -> bool {
        self.as_transform_component().resets_transform()
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_resets_transform(&self, resets_transform: bool) {
        self.as_transform_component()
            .set_resets_transform(resets_transform);
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn minimum_time(&self) -> f64 {
        self.as_transform_component().minimum_time()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn maximum_time(&self) -> f64 {
        self.as_transform_component().maximum_time()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn key_times(&self) -> Vec<f64> {
        self.as_transform_component().key_times()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn local_transform_at_time(&self, time: f64) -> [f32; 16] {
        self.as_transform_component().local_transform_at_time(time)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_identity(&self) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_transform_set_identity(self.handle.as_ptr()) };
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn translation_at_time(&self, time: f64) -> [f32; 3] {
        let mut value = [0.0_f32; 3];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_translation_at_time(self.handle.as_ptr(), time, value.as_mut_ptr());
        }
        value
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn rotation_at_time(&self, time: f64) -> [f32; 3] {
        let mut value = [0.0_f32; 3];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_rotation_at_time(self.handle.as_ptr(), time, value.as_mut_ptr());
        }
        value
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn shear_at_time(&self, time: f64) -> [f32; 3] {
        let mut value = [0.0_f32; 3];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_shear_at_time(self.handle.as_ptr(), time, value.as_mut_ptr());
        }
        value
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn scale_at_time(&self, time: f64) -> [f32; 3] {
        let mut value = [0.0_f32; 3];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_scale_at_time(self.handle.as_ptr(), time, value.as_mut_ptr());
        }
        value
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_matrix_for_time(&self, matrix: [f32; 16], time: f64) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_set_matrix_for_time(self.handle.as_ptr(), matrix.as_ptr(), time);
        };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_translation_for_time(&self, translation: [f32; 3], time: f64) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_set_translation_for_time(
                self.handle.as_ptr(),
                translation[0],
                translation[1],
                translation[2],
                time,
            );
        }
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_rotation_for_time(&self, rotation: [f32; 3], time: f64) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_set_rotation_for_time(
                self.handle.as_ptr(),
                rotation[0],
                rotation[1],
                rotation[2],
                time,
            );
        }
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_shear_for_time(&self, shear: [f32; 3], time: f64) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_set_shear_for_time(
                self.handle.as_ptr(),
                shear[0],
                shear[1],
                shear[2],
                time,
            );
        }
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_scale_for_time(&self, scale: [f32; 3], time: f64) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_set_scale_for_time(
                self.handle.as_ptr(),
                scale[0],
                scale[1],
                scale[2],
                time,
            );
        }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn rotation_matrix_at_time(&self, time: f64) -> [f32; 16] {
        let mut matrix = [0.0_f32; 16];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_rotation_matrix_at_time(
                self.handle.as_ptr(),
                time,
                matrix.as_mut_ptr(),
            );
        }
        matrix
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn translation(&self) -> [f32; 3] {
        let mut value = [0.0_f32; 3];
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_transform_translation(self.handle.as_ptr(), value.as_mut_ptr()) };
        value
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_translation(&self, translation: [f32; 3]) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_set_translation(
                self.handle.as_ptr(),
                translation[0],
                translation[1],
                translation[2],
            );
        }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn rotation(&self) -> [f32; 3] {
        let mut value = [0.0_f32; 3];
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_transform_rotation(self.handle.as_ptr(), value.as_mut_ptr()) };
        value
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_rotation(&self, rotation: [f32; 3]) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_set_rotation(
                self.handle.as_ptr(),
                rotation[0],
                rotation[1],
                rotation[2],
            );
        }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn shear(&self) -> [f32; 3] {
        let mut value = [0.0_f32; 3];
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_transform_shear(self.handle.as_ptr(), value.as_mut_ptr()) };
        value
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_shear(&self, shear: [f32; 3]) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_set_shear(self.handle.as_ptr(), shear[0], shear[1], shear[2]);
        }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn scale(&self) -> [f32; 3] {
        let mut value = [0.0_f32; 3];
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_transform_scale(self.handle.as_ptr(), value.as_mut_ptr()) };
        value
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn set_scale(&self, scale: [f32; 3]) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_set_scale(self.handle.as_ptr(), scale[0], scale[1], scale[2]);
        }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform counterpart.
    pub fn as_transform_component(&self) -> TransformComponent {
        TransformComponent::from_handle(self.handle.clone())
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O transform op counterpart.
pub struct TransformOp {
    handle: ObjectHandle,
}

impl TransformOp {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O transform op counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform op counterpart.
    pub fn name(&self) -> Option<String> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        take_string(unsafe { ffi::mdl_transform_op_name_string(self.handle.as_ptr()) })
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform op counterpart.
    pub fn is_inverse(&self) -> bool {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_transform_op_is_inverse(self.handle.as_ptr()) != 0 }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform op counterpart.
    pub fn float4x4_at_time(&self, time: f64) -> [f32; 16] {
        let mut matrix = [0.0_f32; 16];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_op_copy_float4x4_at_time(
                self.handle.as_ptr(),
                time,
                matrix.as_mut_ptr(),
            );
        };
        matrix
    }
}

macro_rules! define_transform_op {
    ($name:ident, $ffi_name:ident, $animated:ty) => {
        #[derive(Debug, Clone)]
        /// Wraps the corresponding Model I/O counterpart.
        pub struct $name {
            handle: ObjectHandle,
        }

        impl $name {
            /// Builds this wrapper from the retained handle of the wrapped Model I/O name counterpart.
            pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
                Self { handle }
            }

            #[must_use]
            /// Calls the corresponding Model I/O method on the wrapped Model I/O name counterpart.
            pub fn name(&self) -> Option<String> {
                TransformOp::from_handle(self.handle.clone()).name()
            }

            #[must_use]
            /// Calls the corresponding Model I/O method on the wrapped Model I/O name counterpart.
            pub fn is_inverse(&self) -> bool {
                TransformOp::from_handle(self.handle.clone()).is_inverse()
            }

            #[must_use]
            /// Calls the corresponding Model I/O method on the wrapped Model I/O name counterpart.
            pub fn float4x4_at_time(&self, time: f64) -> [f32; 16] {
                TransformOp::from_handle(self.handle.clone()).float4x4_at_time(time)
            }

            #[must_use]
            /// Calls the corresponding Model I/O method on the wrapped Model I/O name counterpart.
            pub fn animated_value(&self) -> Option<$animated> {
                // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
                let ptr = unsafe { ffi::$ffi_name(self.handle.as_ptr()) };
                // SAFETY: The unsafe operation is valid in this context.
                unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(<$animated>::from_handle)
            }

            #[must_use]
            /// Calls the corresponding Model I/O method on the wrapped Model I/O name counterpart.
            pub fn as_transform_op(&self) -> TransformOp {
                TransformOp::from_handle(self.handle.clone())
            }
        }
    };
}

define_transform_op!(
    TransformRotateXOp,
    mdl_transform_rotate_x_op_animated_value,
    AnimatedScalar
);
define_transform_op!(
    TransformRotateYOp,
    mdl_transform_rotate_y_op_animated_value,
    AnimatedScalar
);
define_transform_op!(
    TransformRotateZOp,
    mdl_transform_rotate_z_op_animated_value,
    AnimatedScalar
);
define_transform_op!(
    TransformRotateOp,
    mdl_transform_rotate_op_animated_value,
    AnimatedVector3
);
define_transform_op!(
    TransformTranslateOp,
    mdl_transform_translate_op_animated_value,
    AnimatedVector3
);
define_transform_op!(
    TransformScaleOp,
    mdl_transform_scale_op_animated_value,
    AnimatedVector3
);
define_transform_op!(
    TransformMatrixOp,
    mdl_transform_matrix_op_animated_value,
    AnimatedMatrix4x4
);
define_transform_op!(
    TransformOrientOp,
    mdl_transform_orient_op_animated_value,
    AnimatedQuaternion
);

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O transform stack counterpart.
pub struct TransformStack {
    handle: ObjectHandle,
}

impl TransformStack {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O transform stack counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O transform stack counterpart.
    pub fn new() -> Result<Self> {
        let mut out_stack = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: Output pointers are initialized and managed; FFI function is called safely.
        let status = unsafe { ffi::mdl_transform_stack_new(&mut out_stack, &mut out_error) };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_stack,
            "MDLTransformStack",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn matrix(&self) -> [f32; 16] {
        self.as_transform_component().matrix()
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn set_matrix(&self, matrix: [f32; 16]) {
        self.as_transform_component().set_matrix(matrix);
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn resets_transform(&self) -> bool {
        self.as_transform_component().resets_transform()
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn set_resets_transform(&self, resets_transform: bool) {
        self.as_transform_component()
            .set_resets_transform(resets_transform);
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn minimum_time(&self) -> f64 {
        self.as_transform_component().minimum_time()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn maximum_time(&self) -> f64 {
        self.as_transform_component().maximum_time()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn key_times(&self) -> Vec<f64> {
        self.as_transform_component().key_times()
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn local_transform_at_time(&self, time: f64) -> [f32; 16] {
        self.as_transform_component().local_transform_at_time(time)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn add_translate_op(&self, name: &str, inverse: bool) -> Result<TransformTranslateOp> {
        let name = c_string(name)?;
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_transform_stack_add_translate_op(
                self.handle.as_ptr(),
                name.as_ptr(),
                i32::from(inverse),
            )
        };
        Ok(TransformTranslateOp::from_handle(required_handle(
            ptr,
            "MDLTransformTranslateOp",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn add_rotate_x_op(&self, name: &str, inverse: bool) -> Result<TransformRotateXOp> {
        let name = c_string(name)?;
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_transform_stack_add_rotate_x_op(
                self.handle.as_ptr(),
                name.as_ptr(),
                i32::from(inverse),
            )
        };
        Ok(TransformRotateXOp::from_handle(required_handle(
            ptr,
            "MDLTransformRotateXOp",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn add_rotate_y_op(&self, name: &str, inverse: bool) -> Result<TransformRotateYOp> {
        let name = c_string(name)?;
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_transform_stack_add_rotate_y_op(
                self.handle.as_ptr(),
                name.as_ptr(),
                i32::from(inverse),
            )
        };
        Ok(TransformRotateYOp::from_handle(required_handle(
            ptr,
            "MDLTransformRotateYOp",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn add_rotate_z_op(&self, name: &str, inverse: bool) -> Result<TransformRotateZOp> {
        let name = c_string(name)?;
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_transform_stack_add_rotate_z_op(
                self.handle.as_ptr(),
                name.as_ptr(),
                i32::from(inverse),
            )
        };
        Ok(TransformRotateZOp::from_handle(required_handle(
            ptr,
            "MDLTransformRotateZOp",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn add_rotate_op(
        &self,
        name: &str,
        rotation_order: TransformOpRotationOrder,
        inverse: bool,
    ) -> Result<TransformRotateOp> {
        let name = c_string(name)?;
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_transform_stack_add_rotate_op(
                self.handle.as_ptr(),
                name.as_ptr(),
                rotation_order.as_raw(),
                i32::from(inverse),
            )
        };
        Ok(TransformRotateOp::from_handle(required_handle(
            ptr,
            "MDLTransformRotateOp",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn add_scale_op(&self, name: &str, inverse: bool) -> Result<TransformScaleOp> {
        let name = c_string(name)?;
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_transform_stack_add_scale_op(
                self.handle.as_ptr(),
                name.as_ptr(),
                i32::from(inverse),
            )
        };
        Ok(TransformScaleOp::from_handle(required_handle(
            ptr,
            "MDLTransformScaleOp",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn add_matrix_op(&self, name: &str, inverse: bool) -> Result<TransformMatrixOp> {
        let name = c_string(name)?;
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_transform_stack_add_matrix_op(
                self.handle.as_ptr(),
                name.as_ptr(),
                i32::from(inverse),
            )
        };
        Ok(TransformMatrixOp::from_handle(required_handle(
            ptr,
            "MDLTransformMatrixOp",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn add_orient_op(&self, name: &str, inverse: bool) -> Result<TransformOrientOp> {
        let name = c_string(name)?;
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_transform_stack_add_orient_op(
                self.handle.as_ptr(),
                name.as_ptr(),
                i32::from(inverse),
            )
        };
        Ok(TransformOrientOp::from_handle(required_handle(
            ptr,
            "MDLTransformOrientOp",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn animated_value_named(&self, name: &str) -> Result<Option<AnimatedValue>> {
        let name = c_string(name)?;
        // SAFETY: The unsafe operation is valid in this context.
        let ptr = unsafe {
            ffi::mdl_transform_stack_animated_value_named(self.handle.as_ptr(), name.as_ptr())
        };
        // SAFETY: The unsafe operation is valid in this context.
        Ok(unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(AnimatedValue::from_handle))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn float4x4_at_time(&self, time: f64) -> [f32; 16] {
        let mut matrix = [0.0_f32; 16];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_stack_copy_float4x4_at_time(
                self.handle.as_ptr(),
                time,
                matrix.as_mut_ptr(),
            );
        };
        matrix
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn count(&self) -> usize {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_transform_stack_count(self.handle.as_ptr()) as usize }
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn transform_ops(&self) -> Result<Vec<TransformOp>> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_transform_stack_transform_ops(self.handle.as_ptr()) };
        if ptr.is_null() {
            return Ok(Vec::new());
        }
        array_objects(
            ptr,
            "MDLTransformStack transformOps",
            TransformOp::from_handle,
        )
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform stack counterpart.
    pub fn as_transform_component(&self) -> TransformComponent {
        TransformComponent::from_handle(self.handle.clone())
    }
}

impl Object {
    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O object counterpart.
    pub fn transform_component(&self) -> Option<TransformComponent> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_object_transform_component(self.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(TransformComponent::from_handle)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O object counterpart.
    pub fn set_transform_component(&self, component: Option<&TransformComponent>) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_object_set_transform_component(
                self.as_ptr(),
                component.map_or(ptr::null_mut(), TransformComponent::as_ptr),
            );
        }
    }
}
