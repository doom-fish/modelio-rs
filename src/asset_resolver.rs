use std::ffi::CStr;
use std::panic::AssertUnwindSafe;
use std::ptr;

use crate::asset::Asset;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::util::{c_string, required_handle, take_string};

type AssetResolverCallbackFn =
    dyn Fn(AssetResolverEvent) -> AssetResolverResponse + Send + Sync + 'static;

struct AssetResolverCallback {
    callback: Box<AssetResolverCallbackFn>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Describes one `MDLAssetResolver` protocol request routed into Rust.
pub enum AssetResolverEvent {
    /// Asks whether the named asset can be resolved.
    CanResolveAssetNamed(String),
    /// Asks for a URL string for the named asset.
    ResolveAssetNamed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Returns the result of one `MDLAssetResolver` protocol request.
pub enum AssetResolverResponse {
    /// Returns a boolean for `AssetResolverEvent::CanResolveAssetNamed`.
    Bool(bool),
    /// Returns an optional URL string for `AssetResolverEvent::ResolveAssetNamed`.
    Url(Option<String>),
}

fn callback_name(name: *const core::ffi::c_char) -> Option<String> {
    (!name.is_null()).then(|| {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned()
    })
}

fn duplicate_c_string(value: &str) -> *mut core::ffi::c_char {
    let Ok(value) = std::ffi::CString::new(value) else {
        return ptr::null_mut();
    };
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { libc::strdup(value.as_ptr()) }
}

fn callback_response(
    context: *mut core::ffi::c_void,
    event: AssetResolverEvent,
) -> Option<AssetResolverResponse> {
    let context = (!context.is_null()).then_some(context.cast::<AssetResolverCallback>())?;
    std::panic::catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: The unsafe operation is valid in this context.
        (unsafe { &*context }.callback)(event)
    }))
    .ok()
}

#[no_mangle]
pub extern "C" fn mdlx_asset_resolver_can_resolve_named(
    context: *mut core::ffi::c_void,
    name: *const core::ffi::c_char,
) -> i32 {
    let Some(name) = callback_name(name) else {
        return 0;
    };
    match callback_response(context, AssetResolverEvent::CanResolveAssetNamed(name)) {
        Some(AssetResolverResponse::Bool(can_resolve)) => i32::from(can_resolve),
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn mdlx_asset_resolver_resolve_named(
    context: *mut core::ffi::c_void,
    name: *const core::ffi::c_char,
) -> *mut core::ffi::c_char {
    let Some(name) = callback_name(name) else {
        return ptr::null_mut();
    };
    match callback_response(context, AssetResolverEvent::ResolveAssetNamed(name)) {
        Some(AssetResolverResponse::Url(Some(url))) => duplicate_c_string(&url),
        _ => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn mdlx_asset_resolver_release(context: *mut core::ffi::c_void) {
    if context.is_null() {
        return;
    }
    // SAFETY: The unsafe operation is valid in this context.
    unsafe { drop(Box::from_raw(context.cast::<AssetResolverCallback>())) };
}

fn release_callback_context(context: *mut core::ffi::c_void) {
    mdlx_asset_resolver_release(context);
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O asset resolver counterpart.
pub struct AssetResolver {
    handle: ObjectHandle,
}

impl AssetResolver {
    /// Wraps a Rust callback as the corresponding Model I/O asset resolver protocol counterpart.
    pub fn new<F>(callback: F) -> Result<Self>
    where
        F: Fn(AssetResolverEvent) -> AssetResolverResponse + Send + Sync + 'static,
    {
        let callback = Box::new(AssetResolverCallback {
            callback: Box::new(callback),
        });
        let callback_ptr = Box::into_raw(callback).cast::<core::ffi::c_void>();
        let mut out_resolver = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_asset_resolver_new_with_callback(
                callback_ptr,
                &mut out_resolver,
                &mut out_error,
            )
        };
        if let Err(error) = crate::util::status_result(status, out_error) {
            release_callback_context(callback_ptr);
            return Err(error);
        }
        match required_handle(out_resolver, "MDLAssetResolver") {
            Ok(handle) => Ok(Self::from_handle(handle)),
            Err(error) => {
                release_callback_context(callback_ptr);
                Err(error)
            }
        }
    }

    /// Builds this wrapper from the retained handle of the wrapped Model I/O asset resolver counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O asset resolver counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O asset resolver counterpart.
    pub fn can_resolve_asset_named(&self, name: &str) -> Result<bool> {
        let name = c_string(name)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        Ok(unsafe { ffi::mdl_asset_resolver_can_resolve_named(self.as_ptr(), name.as_ptr()) != 0 })
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O asset resolver counterpart.
    pub fn resolve_asset_named(&self, name: &str) -> Result<Option<String>> {
        let name = c_string(name)?;
        // SAFETY: The unsafe operation is valid in this context.
        Ok(take_string(unsafe {
            ffi::mdl_asset_resolver_resolve_named(self.as_ptr(), name.as_ptr())
        }))
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O path asset resolver counterpart.
pub struct PathAssetResolver {
    handle: ObjectHandle,
}

impl PathAssetResolver {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O path asset resolver counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O path asset resolver counterpart.
    pub fn new(path: &str) -> Result<Self> {
        let path = c_string(path)?;
        let mut out_resolver = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_path_asset_resolver_new(path.as_ptr(), &mut out_resolver, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_resolver,
            "MDLPathAssetResolver",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O path asset resolver counterpart.
    pub fn path(&self) -> Option<String> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        take_string(unsafe { ffi::mdl_path_asset_resolver_path(self.handle.as_ptr()) })
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O path asset resolver counterpart.
    pub fn set_path(&self, path: &str) -> Result<()> {
        let path = c_string(path)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_path_asset_resolver_set_path(self.handle.as_ptr(), path.as_ptr()) };
        Ok(())
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O path asset resolver counterpart.
    pub fn as_asset_resolver(&self) -> AssetResolver {
        AssetResolver::from_handle(self.handle.clone())
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O bundle asset resolver counterpart.
pub struct BundleAssetResolver {
    handle: ObjectHandle,
}

impl BundleAssetResolver {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O bundle asset resolver counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O bundle asset resolver counterpart.
    pub fn new(path: &str) -> Result<Self> {
        let path = c_string(path)?;
        let mut out_resolver = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_bundle_asset_resolver_new(path.as_ptr(), &mut out_resolver, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_resolver,
            "MDLBundleAssetResolver",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O bundle asset resolver counterpart.
    pub fn path(&self) -> Option<String> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        take_string(unsafe { ffi::mdl_bundle_asset_resolver_path(self.handle.as_ptr()) })
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O bundle asset resolver counterpart.
    pub fn set_path(&self, path: &str) -> Result<()> {
        let path = c_string(path)?;
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_bundle_asset_resolver_set_path(self.handle.as_ptr(), path.as_ptr()) };
        Ok(())
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O bundle asset resolver counterpart.
    pub fn as_asset_resolver(&self) -> AssetResolver {
        AssetResolver::from_handle(self.handle.clone())
    }
}

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O relative asset resolver counterpart.
pub struct RelativeAssetResolver {
    handle: ObjectHandle,
}

impl RelativeAssetResolver {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O relative asset resolver counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O relative asset resolver counterpart.
    pub fn new(asset: &Asset) -> Result<Self> {
        let mut out_resolver = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_relative_asset_resolver_new(asset.as_ptr(), &mut out_resolver, &mut out_error)
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_resolver,
            "MDLRelativeAssetResolver",
        )?))
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O relative asset resolver counterpart.
    pub fn asset(&self) -> Option<Asset> {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        let ptr = unsafe { ffi::mdl_relative_asset_resolver_asset(self.handle.as_ptr()) };
        // SAFETY: The unsafe operation is valid in this context.
        unsafe { ObjectHandle::from_retained_ptr(ptr) }.map(Asset::from_handle)
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O relative asset resolver counterpart.
    pub fn set_asset(&self, asset: Option<&Asset>) {
        // SAFETY: The unsafe operation is valid in this context.
        unsafe {
            ffi::mdl_relative_asset_resolver_set_asset(
                self.handle.as_ptr(),
                asset.map_or(ptr::null_mut(), Asset::as_ptr),
            );
        }
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O relative asset resolver counterpart.
    pub fn as_asset_resolver(&self) -> AssetResolver {
        AssetResolver::from_handle(self.handle.clone())
    }
}
