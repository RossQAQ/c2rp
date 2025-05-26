use nom::{IResult, Parser, bytes::complete::take, multi::count};

use crate::err::SecError;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub(super) n: i32,

    pub(super) indexes: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct MeshCollection {
    collection: Vec<Mesh>,
}

impl MeshCollection {
    pub fn from_raw(raw: &[u8], counts: i32) -> IResult<&[u8], Self, SecError> {
        let (ret, collection) = count(mesh_parser, counts as usize).parse(raw)?;

        Ok((ret, Self { collection }))
    }
}

fn mesh_parser(raw: &[u8]) -> IResult<&[u8], Mesh, SecError> {
    let (next, n) = take(4usize)(raw)?;

    let n = i32::from_le_bytes(n.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (ret, indexes_raw) = count(take(4usize), n as usize).parse(next)?;

    let mut indexes = Vec::with_capacity(n as usize);

    for index in indexes_raw {
        indexes.push(i32::from_le_bytes(
            index.try_into().map_err(|_| SecError::FromSliceErr)?,
        ));
    }

    Ok((ret, Mesh { n, indexes }))
}
