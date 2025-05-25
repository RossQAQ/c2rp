use std::ffi::CStr;

use crate::err::SecError;
use crate::utils::convert_cstr_byte_until_null_to_string;
use nom::bytes::complete::take;
use nom::multi::count;
use nom::{IResult, Parser};

/// SEC header Token part.
#[derive(Debug)]
pub(crate) struct SecToken {
    /// number of tokens
    count: i32,

    /// tokens
    ///
    /// each token size is 32 bytes.
    tokens: Vec<String>,
}

impl SecToken {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self, SecError> {
        let (next, token_count) = take(4usize)(raw)?;

        let token_count =
            i32::from_le_bytes(token_count.try_into().map_err(|_| SecError::FromSliceErr)?);

        let (ret, tokens) = count(take(32usize), token_count as usize).parse(next)?;

        let tokens = tokens
            .into_iter()
            .map(convert_cstr_byte_until_null_to_string)
            .collect::<Result<Vec<String>, SecError>>()?;

        Ok((
            ret,
            Self {
                count: token_count,
                tokens,
            },
        ))
    }
}
