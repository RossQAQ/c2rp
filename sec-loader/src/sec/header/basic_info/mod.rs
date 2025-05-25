use nom::{IResult, bytes::complete::take};

use crate::err::SecError;

/// SEC header basic info part.
///
/// ! fix: 这里可能变量名错误
#[derive(Debug)]
pub struct BasicInfo {
    /// number of vertices
    vertex_num: i32,

    border_num: i32,

    district_num: i32,

    special: i32,
}

impl BasicInfo {
    pub fn from_raw(raw: &[u8]) -> IResult<&[u8], Self, SecError> {
        let (next, vertex_num) = take(4usize)(raw)?;
        let vertex_num =
            i32::from_le_bytes(vertex_num.try_into().map_err(|_| SecError::FromSliceErr)?);

        let (next, border_num) = take(4usize)(next)?;
        let border_num =
            i32::from_le_bytes(border_num.try_into().map_err(|_| SecError::FromSliceErr)?);

        let (next, district_num) = take(4usize)(next)?;
        let district_num = i32::from_le_bytes(
            district_num
                .try_into()
                .map_err(|_| SecError::FromSliceErr)?,
        );

        let (ret, special) = take(4usize)(next)?;
        let special = i32::from_le_bytes(special.try_into().map_err(|_| SecError::FromSliceErr)?);

        Ok((
            ret,
            Self {
                vertex_num,
                border_num,
                district_num,
                special,
            },
        ))
    }
}
