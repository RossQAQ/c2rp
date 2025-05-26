use nom::{IResult, bytes::complete::take};
use token::SecToken;
use version::VersionSign;

use crate::{err::SecError, utils::convert_cstr_byte_until_null_to_string};

mod token;
mod version;

/// SEC header part.
#[derive(Debug, Clone)]
pub struct Identify {
    /// version info
    version: VersionSign,

    /// token info
    token: SecToken,

    /// fixed token: "MAP1"
    ///
    /// size: 32 bytes
    fixed_map1: String,
}

impl Identify {
    pub fn from_raw(raw: &[u8]) -> IResult<&[u8], Self, SecError> {
        let (next, version) = VersionSign::parse(raw)?;

        let (next, token) = SecToken::parse(next)?;

        let (ret, fixed_map1) = take(32usize)(next)?;

        Ok((
            ret,
            Self {
                version,
                token,
                fixed_map1: convert_cstr_byte_until_null_to_string(fixed_map1)?,
            },
        ))
    }
}
