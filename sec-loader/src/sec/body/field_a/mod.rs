use nom::{IResult, Parser, bytes::complete::take, multi::count};

use crate::err::SecError;

#[derive(Debug, Clone)]
struct FieldA {
    field_1: f32,

    field_2: f32,
}

#[derive(Debug, Clone)]
pub struct FieldACollection {
    collection: Vec<FieldA>,
}

impl FieldACollection {
    pub fn from_raw(raw: &[u8], counts: i32) -> IResult<&[u8], Self, SecError> {
        let (ret, field_a) = count(take(8usize), counts as usize).parse(raw)?;

        let mut collection = Vec::with_capacity(counts as usize);
        for field in field_a {
            let (field_1, field_2) = field.split_at(4);

            let field_1 =
                f32::from_le_bytes(field_1.try_into().map_err(|_| SecError::FromSliceErr)?);
            let field_2 =
                f32::from_le_bytes(field_2.try_into().map_err(|_| SecError::FromSliceErr)?);

            collection.push(FieldA { field_1, field_2 });
        }

        Ok((ret, Self { collection }))
    }
}
