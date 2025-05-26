use basic_info::BasicInfo;
use mesh::MeshCollection;
use nom::{IResult, bytes::complete::tag, combinator::eof};

use crate::err::SecError;

mod basic_info;
mod mesh;

#[derive(Debug, Clone)]
pub struct SecTail {
    tail_tag: i32,

    basic_info: BasicInfo,

    mesh: MeshCollection,
}

impl SecTail {
    pub fn from_raw(raw: &[u8]) -> IResult<&[u8], Self, SecError> {
        let (next, tail_tag) = tag("2SAH")(raw)?;

        let tail_tag = i32::from_le_bytes(tail_tag.try_into().map_err(|_| SecError::FromSliceErr)?);

        let (next, basic_info) = BasicInfo::from_raw(next)?;

        let (end, mesh) = MeshCollection::from_raw(next, basic_info.mesh_x * basic_info.mesh_y)?;

        eof(end)?;

        Ok((
            end,
            Self {
                tail_tag,
                basic_info,
                mesh,
            },
        ))
    }
}
