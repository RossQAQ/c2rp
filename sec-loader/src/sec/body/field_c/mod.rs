use nom::{IResult, Parser, bytes::complete::take, multi::count};

use crate::err::SecError;

#[derive(Debug, Clone)]
pub struct FieldC {
    pub(super) n: i32,

    pub(super) terrain: i64,

    pub(super) kx: f32,

    pub(super) ky: f32,

    pub(super) bz: f32,

    pub(super) unknown: Vec<u8>,

    pub(super) min_x: f32,

    pub(super) min_y: f32,

    pub(super) min_z: f32,

    pub(super) max_x: f32,

    pub(super) max_y: f32,

    pub(super) max_z: f32,

    pub(super) indexes: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct FieldCCollection {
    collection: Vec<FieldC>,
}

impl FieldCCollection {
    pub fn from_raw(raw: &[u8], counts: i32) -> IResult<&[u8], Self, SecError> {
        let (ret, collection) = count(field_c_parser, counts as usize).parse(raw)?;

        Ok((ret, FieldCCollection { collection }))
    }
}

fn field_c_parser(raw: &[u8]) -> IResult<&[u8], FieldC, SecError> {
    let (next, n) = take(4usize)(raw)?;

    let n = i32::from_le_bytes(n.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, terrain) = take(8usize)(next)?;

    let terrain = i64::from_le_bytes(terrain.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, kx) = take(4usize)(next)?;

    let kx = f32::from_le_bytes(kx.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, ky) = take(4usize)(next)?;

    let ky = f32::from_le_bytes(ky.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, bz) = take(4usize)(next)?;

    let bz = f32::from_le_bytes(bz.try_into().map_err(|_| SecError::FromSliceErr)?);

    // C2 24 bytes
    let (next, unknown) = take(24usize)(next)?;

    let unknown = unknown.to_vec();

    let (next, min_x) = take(4usize)(next)?;

    let min_x = f32::from_le_bytes(min_x.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, min_y) = take(4usize)(next)?;

    let min_y = f32::from_le_bytes(min_y.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, min_z) = take(4usize)(next)?;

    let min_z = f32::from_le_bytes(min_z.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, max_x) = take(4usize)(next)?;

    let max_x = f32::from_le_bytes(max_x.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, max_y) = take(4usize)(next)?;

    let max_y = f32::from_le_bytes(max_y.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, max_z) = take(4usize)(next)?;

    let max_z = f32::from_le_bytes(max_z.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (ret, indexes_raw) = count(take(4usize), n as usize).parse(next)?;

    let mut indexes = Vec::with_capacity(n as usize);

    for index in indexes_raw {
        indexes.push(i32::from_le_bytes(
            index.try_into().map_err(|_| SecError::FromSliceErr)?,
        ));
    }

    Ok((
        ret,
        FieldC {
            n,
            terrain,
            kx,
            ky,
            bz,
            unknown,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            indexes,
        },
    ))
}
