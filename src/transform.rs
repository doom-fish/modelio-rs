use std::panic::AssertUnwindSafe;
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

const IDENTITY_MATRIX_F32: [f32; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];
const IDENTITY_MATRIX_F64: [f64; 16] = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
];

type TransformComponentCallbackFn =
    dyn Fn(TransformComponentEvent) -> TransformComponentResponse + Send + Sync + 'static;
type TransformOpCallbackFn = dyn Fn(TransformOpEvent) -> TransformOpResponse + Send + Sync + 'static;

struct TransformComponentCallback {
    callback: Box<TransformComponentCallbackFn>,
}

struct TransformOpCallback {
    callback: Box<TransformOpCallbackFn>,
}

#[derive(Debug, Clone)]
/// Describes one `MDLTransformComponent` protocol request routed into Rust.
pub enum TransformComponentEvent {
    /// Requests the current matrix.
    Matrix,
    /// Updates the current matrix.
    SetMatrix([f32; 16]),
    /// Requests whether the transform resets parent-space transforms.
    ResetsTransform,
    /// Updates whether the transform resets parent-space transforms.
    SetResetsTransform(bool),
    /// Requests the minimum sample time.
    MinimumTime,
    /// Requests the maximum sample time.
    MaximumTime,
    /// Requests the stored key times.
    KeyTimes,
    /// Sets a non-animated local transform.
    SetLocalTransform([f32; 16]),
    /// Sets a sampled local transform at the requested time.
    SetLocalTransformForTime { transform: [f32; 16], time: f64 },
    /// Requests the local transform at the requested time.
    LocalTransformAtTime(f64),
}

#[derive(Debug, Clone)]
/// Returns the result of one `MDLTransformComponent` protocol request.
pub enum TransformComponentResponse {
    /// Returns a matrix result.
    Matrix([f32; 16]),
    /// Returns a boolean result.
    Bool(bool),
    /// Returns a time result.
    Time(f64),
    /// Returns a key-time array result.
    KeyTimes(Vec<f64>),
    /// Indicates that the callback did not provide a value.
    None,
}

#[derive(Debug, Clone)]
/// Describes one `MDLTransformOp` protocol request routed into Rust.
pub enum TransformOpEvent {
    /// Requests the operation name.
    Name,
    /// Requests the single-precision matrix at the requested time.
    Float4x4AtTime(f64),
    /// Requests the double-precision matrix at the requested time.
    Double4x4AtTime(f64),
    /// Requests whether the operation is inverse.
    IsInverseOp,
}

#[derive(Debug, Clone)]
/// Returns the result of one `MDLTransformOp` protocol request.
pub enum TransformOpResponse {
    /// Returns an optional name.
    Name(Option<String>),
    /// Returns a single-precision matrix.
    Float4x4([f32; 16]),
    /// Returns a double-precision matrix.
    Double4x4([f64; 16]),
    /// Returns a boolean result.
    Bool(bool),
    /// Indicates that the callback did not provide a value.
    None,
}

fn duplicate_c_string(value: &str) -> *mut core::ffi::c_char {
    let Ok(value) = std::ffi::CString::new(value) else {
        return ptr::null_mut();
    };
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { libc::strdup(value.as_ptr()) }
}

fn matrix_f32_from_ptr(values: *const f32) -> [f32; 16] {
    if values.is_null() {
        return IDENTITY_MATRIX_F32;
    }
    let mut matrix = [0.0_f32; 16];
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { matrix.as_mut_ptr().copy_from_nonoverlapping(values, matrix.len()) };
    matrix
}

fn write_matrix_f32(out_values: *mut f32, values: [f32; 16]) {
    if out_values.is_null() {
        return;
    }
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { out_values.copy_from_nonoverlapping(values.as_ptr(), values.len()) };
}

fn write_matrix_f64(out_values: *mut f64, values: [f64; 16]) {
    if out_values.is_null() {
        return;
    }
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { out_values.copy_from_nonoverlapping(values.as_ptr(), values.len()) };
}

fn transform_component_response(
    context: *mut core::ffi::c_void,
    event: TransformComponentEvent,
) -> Option<TransformComponentResponse> {
    let context = (!context.is_null()).then_some(context.cast::<TransformComponentCallback>())?;
    std::panic::catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The unsafe operation is valid in this context.
        (unsafe { &*context }.callback)(event)
    }))
    .ok()
}

fn transform_component_matrix_response(context: *mut core::ffi::c_void) -> [f32; 16] {
    match transform_component_response(context, TransformComponentEvent::Matrix) {
        Some(TransformComponentResponse::Matrix(matrix)) => matrix,
        _ => IDENTITY_MATRIX_F32,
    }
}

fn transform_component_key_times_response(context: *mut core::ffi::c_void) -> Vec<f64> {
    match transform_component_response(context, TransformComponentEvent::KeyTimes) {
        Some(TransformComponentResponse::KeyTimes(key_times)) => key_times,
        _ => vec![0.0],
    }
}

fn transform_op_response(
    context: *mut core::ffi::c_void,
    event: TransformOpEvent,
) -> Option<TransformOpResponse> {
    let context = (!context.is_null()).then_some(context.cast::<TransformOpCallback>())?;
    std::panic::catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The unsafe operation is valid in this context.
        (unsafe { &*context }.callback)(event)
    }))
    .ok()
}

fn transform_op_matrix_f32(context: *mut core::ffi::c_void, time: f64) -> [f32; 16] {
    match transform_op_response(context, TransformOpEvent::Float4x4AtTime(time)) {
        Some(TransformOpResponse::Float4x4(matrix)) => matrix,
        Some(TransformOpResponse::Double4x4(matrix)) => matrix.map(|value| value as f32),
        _ => IDENTITY_MATRIX_F32,
    }
}

fn transform_op_matrix_f64(context: *mut core::ffi::c_void, time: f64) -> [f64; 16] {
    match transform_op_response(context, TransformOpEvent::Double4x4AtTime(time)) {
        Some(TransformOpResponse::Double4x4(matrix)) => matrix,
        Some(TransformOpResponse::Float4x4(matrix)) => matrix.map(f64::from),
        _ => IDENTITY_MATRIX_F64,
    }
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_copy_matrix(
    context: *mut core::ffi::c_void,
    out_values: *mut f32,
) {
    write_matrix_f32(out_values, transform_component_matrix_response(context));
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_set_matrix(
    context: *mut core::ffi::c_void,
    values: *const f32,
) {
    let _ = transform_component_response(
        context,
        TransformComponentEvent::SetMatrix(matrix_f32_from_ptr(values)),
    );
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_resets_transform(
    context: *mut core::ffi::c_void,
) -> i32 {
    match transform_component_response(context, TransformComponentEvent::ResetsTransform) {
        Some(TransformComponentResponse::Bool(resets_transform)) => i32::from(resets_transform),
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_set_resets_transform(
    context: *mut core::ffi::c_void,
    resets_transform: i32,
) {
    let _ = transform_component_response(
        context,
        TransformComponentEvent::SetResetsTransform(resets_transform != 0),
    );
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_minimum_time(
    context: *mut core::ffi::c_void,
) -> f64 {
    match transform_component_response(context, TransformComponentEvent::MinimumTime) {
        Some(TransformComponentResponse::Time(time)) => time,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_maximum_time(
    context: *mut core::ffi::c_void,
) -> f64 {
    match transform_component_response(context, TransformComponentEvent::MaximumTime) {
        Some(TransformComponentResponse::Time(time)) => time,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_key_time_count(
    context: *mut core::ffi::c_void,
) -> u64 {
    transform_component_key_times_response(context).len() as u64
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_copy_key_times(
    context: *mut core::ffi::c_void,
    out_values: *mut f64,
    capacity: u64,
) -> u64 {
    let values = transform_component_key_times_response(context);
    let total = values.len();
    if out_values.is_null() || capacity == 0 {
        return total as u64;
    }
    let write_count = total.min(capacity as usize);
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { out_values.copy_from_nonoverlapping(values.as_ptr(), write_count) };
    total as u64
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_set_local_transform(
    context: *mut core::ffi::c_void,
    values: *const f32,
) {
    let _ = transform_component_response(
        context,
        TransformComponentEvent::SetLocalTransform(matrix_f32_from_ptr(values)),
    );
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_set_local_transform_for_time(
    context: *mut core::ffi::c_void,
    values: *const f32,
    time: f64,
) {
    let _ = transform_component_response(
        context,
        TransformComponentEvent::SetLocalTransformForTime {
            transform: matrix_f32_from_ptr(values),
            time,
        },
    );
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_copy_local_transform_at_time(
    context: *mut core::ffi::c_void,
    time: f64,
    out_values: *mut f32,
) {
    let matrix = match transform_component_response(context, TransformComponentEvent::LocalTransformAtTime(time)) {
        Some(TransformComponentResponse::Matrix(matrix)) => matrix,
        _ => transform_component_matrix_response(context),
    };
    write_matrix_f32(out_values, matrix);
}

#[no_mangle]
pub extern "C" fn mdlx_transform_component_release(context: *mut core::ffi::c_void) {
    if context.is_null() {
        return;
    }
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { drop(Box::from_raw(context.cast::<TransformComponentCallback>())) };
}

#[no_mangle]
pub extern "C" fn mdlx_transform_op_name(
    context: *mut core::ffi::c_void,
) -> *mut core::ffi::c_char {
    match transform_op_response(context, TransformOpEvent::Name) {
        Some(TransformOpResponse::Name(Some(name))) => duplicate_c_string(&name),
        _ => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mdlx_transform_op_is_inverse(context: *mut core::ffi::c_void) -> i32 {
    match transform_op_response(context, TransformOpEvent::IsInverseOp) {
        Some(TransformOpResponse::Bool(is_inverse)) => i32::from(is_inverse),
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn mdlx_transform_op_copy_float4x4_at_time(
    context: *mut core::ffi::c_void,
    time: f64,
    out_values: *mut f32,
) {
    write_matrix_f32(out_values, transform_op_matrix_f32(context, time));
}

#[no_mangle]
pub extern "C" fn mdlx_transform_op_copy_double4x4_at_time(
    context: *mut core::ffi::c_void,
    time: f64,
    out_values: *mut f64,
) {
    write_matrix_f64(out_values, transform_op_matrix_f64(context, time));
}

#[no_mangle]
pub extern "C" fn mdlx_transform_op_release(context: *mut core::ffi::c_void) {
    if context.is_null() {
        return;
    }
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { drop(Box::from_raw(context.cast::<TransformOpCallback>())) };
}

fn release_transform_component_callback_context(context: *mut core::ffi::c_void) {
    mdlx_transform_component_release(context);
}

fn release_transform_op_callback_context(context: *mut core::ffi::c_void) {
    mdlx_transform_op_release(context);
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O transform component counterpart.
pub struct TransformComponent {
    handle: ObjectHandle,
}

impl Component for TransformComponent {}

impl TransformComponent {
    /// Wraps a Rust callback as the corresponding Model I/O transform component protocol counterpart.
    pub fn new<F>(callback: F) -> Result<Self>
    where
        F: Fn(TransformComponentEvent) -> TransformComponentResponse + Send + Sync + 'static,
    {
        let callback = Box::new(TransformComponentCallback {
            callback: Box::new(callback),
        });
        let callback_ptr = Box::into_raw(callback).cast::<core::ffi::c_void>();
        let mut out_component = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_transform_component_new_with_callback(
                callback_ptr,
                &mut out_component,
                &mut out_error,
            )
        };
        if let Err(error) = crate::util::status_result(status, out_error) {
            release_transform_component_callback_context(callback_ptr);
            return Err(error);
        }
        match required_handle(out_component, "MDLTransformComponent") {
            Ok(handle) => Ok(Self::from_handle(handle)),
            Err(error) => {
                release_transform_component_callback_context(callback_ptr);
                Err(error)
            }
        }
    }

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
    /// Wraps a Rust callback as the corresponding Model I/O transform op protocol counterpart.
    pub fn new<F>(callback: F) -> Result<Self>
    where
        F: Fn(TransformOpEvent) -> TransformOpResponse + Send + Sync + 'static,
    {
        let callback = Box::new(TransformOpCallback {
            callback: Box::new(callback),
        });
        let callback_ptr = Box::into_raw(callback).cast::<core::ffi::c_void>();
        let mut out_transform_op = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_transform_op_new_with_callback(
                callback_ptr,
                &mut out_transform_op,
                &mut out_error,
            )
        };
        if let Err(error) = crate::util::status_result(status, out_error) {
            release_transform_op_callback_context(callback_ptr);
            return Err(error);
        }
        match required_handle(out_transform_op, "MDLTransformOp") {
            Ok(handle) => Ok(Self::from_handle(handle)),
            Err(error) => {
                release_transform_op_callback_context(callback_ptr);
                Err(error)
            }
        }
    }

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

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O transform op counterpart.
    pub fn double4x4_at_time(&self, time: f64) -> [f64; 16] {
        let mut matrix = [0.0_f64; 16];
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_transform_op_copy_double4x4_at_time(
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
