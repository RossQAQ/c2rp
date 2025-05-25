use std::ffi::CStr;

use crate::err::SecError;

pub fn convert_cstr_byte_until_null_to_string(bytes: &[u8]) -> Result<String, SecError> {
    CStr::from_bytes_until_nul(bytes)
        .ok()
        .and_then(|c_str| c_str.to_str().ok())
        .map(|s| s.to_owned())
        .ok_or(SecError::ParseTypeErr(
            "Convert token from CStr to String failed".to_string(),
        ))
}
