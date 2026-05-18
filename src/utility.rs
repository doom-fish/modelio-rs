use std::path::Path;

use crate::error::Result;
use crate::ffi;
use crate::util::path_to_c_string;

#[derive(Debug, Clone, Copy, Default)]
/// Wraps the corresponding Model I/O utility counterpart.
pub struct Utility;

impl Utility {
    /// Calls the corresponding Model I/O method on the wrapped Model I/O utility counterpart.
    pub fn convert_to_usdz(
        input_url: impl AsRef<Path>,
        output_url: impl AsRef<Path>,
    ) -> Result<()> {
        let input_url = path_to_c_string(input_url.as_ref())?;
        let output_url = path_to_c_string(output_url.as_ref())?;
        let mut out_error = std::ptr::null_mut();
        // SAFETY: The unsafe operation is valid in this context.
        let status = unsafe {
            ffi::mdl_utility_convert_to_usdz(
                input_url.as_ptr(),
                output_url.as_ptr(),
                &mut out_error,
            )
        };
        crate::util::status_result(status, out_error)
    }
}
