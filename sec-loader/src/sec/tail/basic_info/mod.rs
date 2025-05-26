use nom::{IResult, bytes::complete::take};

use crate::err::SecError;

#[derive(Debug, Clone)]
pub struct BasicInfo {
    pub(super) total_mesh: i32,

    pub(super) unknown: i32,

    pub(super) mesh_x: i32,

    pub(super) mesh_y: i32,
}

impl BasicInfo {
    pub fn from_raw(raw: &[u8]) -> IResult<&[u8], Self, SecError> {
        let (next, total_mesh) = take(4usize)(raw)?;

        let total_mesh =
            i32::from_le_bytes(total_mesh.try_into().map_err(|_| SecError::FromSliceErr)?);

        let (next, unknown) = take(4usize)(next)?;

        let unknown = i32::from_le_bytes(unknown.try_into().map_err(|_| SecError::FromSliceErr)?);

        let (next, mesh_x) = take(4usize)(next)?;

        let mesh_x = i32::from_le_bytes(mesh_x.try_into().map_err(|_| SecError::FromSliceErr)?);

        let (next, mesh_y) = take(4usize)(next)?;

        let mesh_y = i32::from_le_bytes(mesh_y.try_into().map_err(|_| SecError::FromSliceErr)?);

        Ok((
            next,
            BasicInfo {
                total_mesh,
                unknown,
                mesh_x,
                mesh_y,
            },
        ))
    }
}
