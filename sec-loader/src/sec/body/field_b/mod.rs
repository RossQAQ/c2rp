use nom::{IResult, Parser, bytes::complete::take, multi::count};

use crate::err::SecError;

#[derive(Debug, Clone)]
pub struct FieldB {
    pub(super) field_1: i32,

    pub(super) field_2: i32,

    pub(super) field_3: i32,

    pub(super) field_4: i32,

    pub(super) unknown: i32,
}

#[derive(Debug, Clone)]
pub struct FieldBCollection {
    collection: Vec<FieldB>,
}

impl FieldBCollection {
    pub fn from_raw(raw: &[u8], counts: i32) -> IResult<&[u8], Self, SecError> {
        let (ret, field_b) = count(take(20usize), counts as usize).parse(raw)?;

        let mut collection = Vec::with_capacity(counts as usize);

        for field in field_b {
            let field_1 =
                i32::from_le_bytes(field[0..4].try_into().map_err(|_| SecError::FromSliceErr)?);
            let field_2 =
                i32::from_le_bytes(field[4..8].try_into().map_err(|_| SecError::FromSliceErr)?);
            let field_3 = i32::from_le_bytes(
                field[8..12]
                    .try_into()
                    .map_err(|_| SecError::FromSliceErr)?,
            );
            let field_4 = i32::from_le_bytes(
                field[12..16]
                    .try_into()
                    .map_err(|_| SecError::FromSliceErr)?,
            );
            let unknown = i32::from_le_bytes(
                field[16..20]
                    .try_into()
                    .map_err(|_| SecError::FromSliceErr)?,
            );

            collection.push(FieldB {
                field_1,
                field_2,
                field_3,
                field_4,
                unknown,
            });
        }

        Ok((ret, Self { collection }))
    }
}
