use nom::{IResult, Parser, bytes::complete::take, multi::count};

use crate::err::SecError;

/// A polygon (if c equles to 0, it's a plane in x-y plane)
///
/// Every plumb line will intersect with that `ax + by + c = 0` plane
///
/// `ax + by + c = 0` is the top-face of this polygon
#[derive(Debug, Clone)]
pub struct Polygon {
    /// Number of side of the polygon
    pub(super) sides_num: i32,

    /// Terrain description
    pub(super) terrain: i64,

    /// A coefficient of the x-axis
    pub(super) ax: f32,

    /// A coefficient of the y-axis
    pub(super) by: f32,

    /// a constant
    pub(super) c: f32,

    /// Unknown data
    pub(super) unknown: Vec<u8>,

    pub(super) min_x: f32,

    pub(super) min_y: f32,

    pub(super) min_z: f32,

    pub(super) max_x: f32,

    pub(super) max_y: f32,

    pub(super) max_z: f32,

    /// Indexes of the polygon sides
    pub(super) indexes: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct Polygons {
    collection: Vec<Polygon>,
}

impl Polygons {
    pub fn from_raw(raw: &[u8], counts: i32) -> IResult<&[u8], Self, SecError> {
        let (ret, collection) = count(field_c_parser, counts as usize).parse(raw)?;

        Ok((ret, Polygons { collection }))
    }
}

fn field_c_parser(raw: &[u8]) -> IResult<&[u8], Polygon, SecError> {
    let (next, n) = take(4usize)(raw)?;

    let n = i32::from_le_bytes(n.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, terrain) = take(8usize)(next)?;

    let terrain = i64::from_le_bytes(terrain.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, ax) = take(4usize)(next)?;

    let ax = f32::from_le_bytes(ax.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, by) = take(4usize)(next)?;

    let by = f32::from_le_bytes(by.try_into().map_err(|_| SecError::FromSliceErr)?);

    let (next, c) = take(4usize)(next)?;

    let c = f32::from_le_bytes(c.try_into().map_err(|_| SecError::FromSliceErr)?);

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
        Polygon {
            sides_num: n,
            terrain,
            ax,
            by,
            c,
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
