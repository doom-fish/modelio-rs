use std::path::Path;
use std::ptr;

use crate::error::Result;
use crate::ffi;
use crate::handle::ObjectHandle;
use crate::types::{TextureChannelEncoding, TextureInfo};
use crate::util::{c_string, parse_json, path_to_c_string, required_handle};

#[derive(Debug, Clone)]
pub struct Texture {
    handle: ObjectHandle,
}

impl Texture {
    pub(crate) fn from_handle(handle: ObjectHandle) -> Self {
        Self { handle }
    }

    pub fn from_url(path: impl AsRef<Path>, name: Option<&str>) -> Result<Self> {
        let path = path_to_c_string(path.as_ref())?;
        let name = name.map(c_string).transpose()?;
        let mut out_texture = ptr::null_mut();
        let mut out_error = ptr::null_mut();
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

    pub fn info(&self) -> Result<TextureInfo> {
        parse_json(
            unsafe { ffi::mdl_texture_info_json(self.handle.as_ptr()) },
            "MDLTexture",
        )
    }

    pub fn write_to_url(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path_to_c_string(path.as_ref())?;
        let mut out_error = ptr::null_mut();
        let status = unsafe {
            ffi::mdl_texture_write_to_url(self.handle.as_ptr(), path.as_ptr(), &mut out_error)
        };
        crate::util::status_result(status, out_error)
    }

    fn texel_data(&self, top_left_origin: bool) -> Vec<u8> {
        let length = unsafe {
            ffi::mdl_texture_texel_data_length(self.handle.as_ptr(), i32::from(top_left_origin))
        } as usize;
        let mut bytes = vec![0_u8; length];
        if length == 0 {
            return bytes;
        }
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
    pub fn texel_data_top_left(&self) -> Vec<u8> {
        self.texel_data(true)
    }

    #[must_use]
    pub fn texel_data_bottom_left(&self) -> Vec<u8> {
        self.texel_data(false)
    }
}
