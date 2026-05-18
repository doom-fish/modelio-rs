use std::ptr;

use crate::asset::Asset;
use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::util::{c_string, required_handle, take_string};

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O asset resolver counterpart.
pub struct AssetResolver {
    handle: ObjectHandle,
}

impl AssetResolver {
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
