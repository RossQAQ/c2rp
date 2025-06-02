use nom::{IResult, Parser, bytes::complete::take, multi::count};

use crate::err::SecError;

/// Side of the polygon.
#[derive(Debug, Clone)]
pub struct Side {
    /// start plumb line id
    pub start: i32,

    /// end plumb line id
    pub end: i32,

    /// belongs to polygon id
    pub belongs_to_polygon_id: i32,

    /// neighbor polygon id
    pub neighbor_polygon_id: i32,

    /// unknown field
    pub(super) unknown: i32,
}

#[derive(Debug, Clone)]
pub struct Sides {
    collection: Vec<Side>,
}

impl IntoIterator for Sides {
    type Item = Side;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}

impl<'a> IntoIterator for &'a Sides {
    type Item = &'a Side;

    type IntoIter = std::slice::Iter<'a, Side>;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.iter()
    }
}

impl Sides {
    pub(crate) fn from_raw(raw: &[u8], counts: i32) -> IResult<&[u8], Self, SecError> {
        let (ret, sides) = count(take(20usize), counts as usize).parse(raw)?;

        let mut collection = Vec::with_capacity(counts as usize);

        for side in sides {
            let start =
                i32::from_le_bytes(side[0..4].try_into().map_err(|_| SecError::FromSliceErr)?);
            let end =
                i32::from_le_bytes(side[4..8].try_into().map_err(|_| SecError::FromSliceErr)?);
            let belongs_to_polygon_id =
                i32::from_le_bytes(side[8..12].try_into().map_err(|_| SecError::FromSliceErr)?);
            let neighber_polygon_id = i32::from_le_bytes(
                side[12..16]
                    .try_into()
                    .map_err(|_| SecError::FromSliceErr)?,
            );
            let unknown = i32::from_le_bytes(
                side[16..20]
                    .try_into()
                    .map_err(|_| SecError::FromSliceErr)?,
            );

            collection.push(Side {
                start,
                end,
                belongs_to_polygon_id,
                neighbor_polygon_id: neighber_polygon_id,
                unknown,
            });
        }

        Ok((ret, Self { collection }))
    }

    pub fn get(&self, idx: usize) -> Option<&Side> {
        self.collection.get(idx)
    }
}
