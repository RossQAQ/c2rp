use nom::{IResult, bytes::complete::take};

use crate::err::SecError;

/// SEC header basic info part.
#[derive(Debug, Clone)]
pub struct BasicInfo {
    /// number of plumb lines
    pub(super) plumb_line_num: i32,

    /// number of borders
    ///
    /// Each two plumb lines will form a border.
    pub(super) side_face_num: i32,

    /// number of polygons
    pub(super) polygon_num: i32,

    /// number of polygon's throughable side
    pub(super) throughable_side_num: i32,
}

impl BasicInfo {
    pub fn from_raw(raw: &[u8]) -> IResult<&[u8], Self, SecError> {
        let (next, plumb_lines) = take(4usize)(raw)?;
        let plumb_lines =
            i32::from_le_bytes(plumb_lines.try_into().map_err(|_| SecError::FromSliceErr)?);

        let (next, side_faces) = take(4usize)(next)?;
        let side_faces =
            i32::from_le_bytes(side_faces.try_into().map_err(|_| SecError::FromSliceErr)?);

        let (next, polygons) = take(4usize)(next)?;
        let polygons = i32::from_le_bytes(polygons.try_into().map_err(|_| SecError::FromSliceErr)?);

        let (ret, through_able) = take(4usize)(next)?;
        let through_able = i32::from_le_bytes(
            through_able
                .try_into()
                .map_err(|_| SecError::FromSliceErr)?,
        );

        Ok((
            ret,
            Self {
                plumb_line_num: plumb_lines,
                side_face_num: side_faces,
                polygon_num: polygons,
                throughable_side_num: through_able,
            },
        ))
    }
}
