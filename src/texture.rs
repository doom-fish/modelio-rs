use std::path::Path;
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::{TextureChannelEncoding, TextureInfo};
use crate::util::{c_string, parse_json, path_to_c_string, required_handle};

#[derive(Debug, Clone)]
/// Wraps the corresponding Model I/O texture counterpart.
pub struct Texture {
    handle: ObjectHandle,
}

impl Texture {
    /// Builds this wrapper from the retained handle of the wrapped Model I/O texture counterpart.
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    /// Returns the opaque pointer used to call the wrapped Model I/O texture counterpart.
    pub(crate) fn as_ptr(&self) -> *mut core::ffi::c_void {
        self.handle.as_ptr()
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture counterpart.
    pub fn from_url(path: impl AsRef<Path>, name: Option<&str>) -> Result<Self> {
        let path = path_to_c_string(path.as_ref())?;
        let name = name.map(c_string).transpose()?;
        let mut out_texture = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_url_texture_new(
                path.as_ptr(),
                name.as_ref().map_or(ptr::null(), |name| name.as_ptr()),
                &mut out_texture,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_texture,
            "MDLURLTexture",
        )?))
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O texture counterpart.
    pub fn new_checkerboard(
        divisions: f32,
        name: Option<&str>,
        dimensions: [i32; 2],
        channel_count: usize,
        channel_encoding: TextureChannelEncoding,
        color1: [f32; 4],
        color2: [f32; 4],
    ) -> Result<Self> {
        let name = name.map(c_string).transpose()?;
        let mut out_texture = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_checkerboard_texture_new(
                divisions,
                name.as_ref().map_or(ptr::null(), |name| name.as_ptr()),
                dimensions[0],
                dimensions[1],
                channel_count as u64,
                channel_encoding as i32,
                color1[0],
                color1[1],
                color1[2],
                color1[3],
                color2[0],
                color2[1],
                color2[2],
                color2[3],
                &mut out_texture,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_texture,
            "MDLCheckerboardTexture",
        )?))
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O texture counterpart.
    pub fn new_color_temperature_gradient(
        color_temperature1: f32,
        color_temperature2: f32,
        name: Option<&str>,
        dimensions: [i32; 2],
    ) -> Result<Self> {
        let name = name.map(c_string).transpose()?;
        let mut out_texture = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_color_swatch_texture_new_temperature_gradient(
                color_temperature1,
                color_temperature2,
                name.as_ref().map_or(ptr::null(), |name| name.as_ptr()),
                dimensions[0],
                dimensions[1],
                &mut out_texture,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_texture,
            "MDLColorSwatchTexture",
        )?))
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O texture counterpart.
    pub fn new_color_gradient(
        color1: [f32; 4],
        color2: [f32; 4],
        name: Option<&str>,
        dimensions: [i32; 2],
    ) -> Result<Self> {
        let name = name.map(c_string).transpose()?;
        let mut out_texture = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_color_swatch_texture_new_color_gradient(
                color1[0],
                color1[1],
                color1[2],
                color1[3],
                color2[0],
                color2[1],
                color2[2],
                color2[3],
                name.as_ref().map_or(ptr::null(), |name| name.as_ptr()),
                dimensions[0],
                dimensions[1],
                &mut out_texture,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_texture,
            "MDLColorSwatchTexture",
        )?))
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O texture counterpart.
    pub fn new_vector_noise(
        smoothness: f32,
        name: Option<&str>,
        dimensions: [i32; 2],
        channel_encoding: TextureChannelEncoding,
    ) -> Result<Self> {
        let name = name.map(c_string).transpose()?;
        let mut out_texture = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_noise_texture_new_vector(
                smoothness,
                name.as_ref().map_or(ptr::null(), |name| name.as_ptr()),
                dimensions[0],
                dimensions[1],
                channel_encoding.as_raw(),
                &mut out_texture,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_texture,
            "MDLNoiseTexture",
        )?))
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O texture counterpart.
    pub fn new_scalar_noise(
        smoothness: f32,
        name: Option<&str>,
        dimensions: [i32; 2],
        channel_count: usize,
        channel_encoding: TextureChannelEncoding,
        grayscale: bool,
    ) -> Result<Self> {
        let name = name.map(c_string).transpose()?;
        let mut out_texture = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_noise_texture_new_scalar(
                smoothness,
                name.as_ref().map_or(ptr::null(), |name| name.as_ptr()),
                dimensions[0],
                dimensions[1],
                channel_count as u64,
                channel_encoding.as_raw(),
                i32::from(grayscale),
                &mut out_texture,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_texture,
            "MDLNoiseTexture",
        )?))
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O texture counterpart.
    pub fn new_cellular_noise(
        frequency: f32,
        name: Option<&str>,
        dimensions: [i32; 2],
        channel_encoding: TextureChannelEncoding,
    ) -> Result<Self> {
        let name = name.map(c_string).transpose()?;
        let mut out_texture = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_noise_texture_new_cellular(
                frequency,
                name.as_ref().map_or(ptr::null(), |name| name.as_ptr()),
                dimensions[0],
                dimensions[1],
                channel_encoding.as_raw(),
                &mut out_texture,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_texture,
            "MDLNoiseTexture",
        )?))
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O texture counterpart.
    pub fn new_normal_map(
        source_texture: &Self,
        name: Option<&str>,
        smoothness: f32,
        contrast: f32,
    ) -> Result<Self> {
        let name = name.map(c_string).transpose()?;
        let mut out_texture = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_normal_map_texture_new(
                source_texture.as_ptr(),
                name.as_ref().map_or(ptr::null(), |name| name.as_ptr()),
                smoothness,
                contrast,
                &mut out_texture,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_texture,
            "MDLNormalMapTexture",
        )?))
    }

    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O texture counterpart.
    pub fn new_sky_cube(
        name: Option<&str>,
        dimensions: [i32; 2],
        channel_encoding: TextureChannelEncoding,
        turbidity: f32,
        sun_elevation: f32,
        upper_atmosphere_scattering: f32,
        ground_albedo: f32,
    ) -> Result<Self> {
        let name = name.map(c_string).transpose()?;
        let mut out_texture = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_sky_cube_texture_new(
                name.as_ref().map_or(ptr::null(), |name| name.as_ptr()),
                channel_encoding.as_raw(),
                dimensions[0],
                dimensions[1],
                turbidity,
                sun_elevation,
                upper_atmosphere_scattering,
                ground_albedo,
                &mut out_texture,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_texture,
            "MDLSkyCubeTexture",
        )?))
    }

    #[allow(clippy::too_many_arguments)]
    /// Wraps the corresponding Model I/O initializer for the wrapped Model I/O texture counterpart.
    pub fn new_sky_cube_with_azimuth(
        name: Option<&str>,
        dimensions: [i32; 2],
        channel_encoding: TextureChannelEncoding,
        turbidity: f32,
        sun_elevation: f32,
        sun_azimuth: f32,
        upper_atmosphere_scattering: f32,
        ground_albedo: f32,
    ) -> Result<Self> {
        let name = name.map(c_string).transpose()?;
        let mut out_texture = ptr::null_mut();
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_sky_cube_texture_new_with_azimuth(
                name.as_ref().map_or(ptr::null(), |name| name.as_ptr()),
                channel_encoding.as_raw(),
                dimensions[0],
                dimensions[1],
                turbidity,
                sun_elevation,
                sun_azimuth,
                upper_atmosphere_scattering,
                ground_albedo,
                &mut out_texture,
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)?;
        Ok(Self::from_handle(required_handle(
            out_texture,
            "MDLSkyCubeTexture",
        )?))
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture counterpart.
    pub fn update_sky_cube(&self) {
        // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
        unsafe { ffi::mdl_sky_cube_texture_update(self.as_ptr()) };
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture counterpart.
    pub fn info(&self) -> Result<TextureInfo> {
        parse_json(
            // SAFETY: ObjectHandle wraps a valid opaque pointer from Swift; FFI function accepts it safely.
            unsafe { ffi::mdl_texture_info_json(self.handle.as_ptr()) },
            "MDLTexture",
        )
    }

    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture counterpart.
    pub fn write_to_url(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path_to_c_string(path.as_ref())?;
        let mut out_error = ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_texture_write_to_url(self.handle.as_ptr(), path.as_ptr(), &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    fn texel_data(&self, top_left_origin: bool) -> Vec<u8> {
        // SAFETY: The unsafe operation is valid in this context.
        let length = unsafe {
            ffi::mdl_texture_texel_data_length(self.handle.as_ptr(), i32::from(top_left_origin))
        } as usize;
        let mut bytes = vec![0_u8; length];
        if length == 0 {
            return bytes;
        }
        // SAFETY: The unsafe operation is valid in this context.
        let written = unsafe {
            ffi::mdl_texture_copy_texel_data(
                self.handle.as_ptr(),
                i32::from(top_left_origin),
                bytes.as_mut_ptr(),
                bytes.len() as u64,
            )
        } as usize;
        bytes.truncate(written);
        bytes
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture counterpart.
    pub fn texel_data_top_left(&self) -> Vec<u8> {
        self.texel_data(true)
    }

    #[must_use]
    /// Calls the corresponding Model I/O method on the wrapped Model I/O texture counterpart.
    pub fn texel_data_bottom_left(&self) -> Vec<u8> {
        self.texel_data(false)
    }
}
