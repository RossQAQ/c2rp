use nom::{IResult, Parser, bytes::complete::take, multi::count};

use crate::err::SecError;

#[derive(Debug, Clone)]
struct PlumbLine {
    x: f32,

    y: f32,
}

#[derive(Debug, Clone)]
pub struct PlumbLines {
    collection: Vec<PlumbLine>,
}

impl PlumbLines {
    pub fn from_raw(raw: &[u8], counts: i32) -> IResult<&[u8], Self, SecError> {
        let (ret, plumb_lines) = count(take(8usize), counts as usize).parse(raw)?;

        let mut collection = Vec::with_capacity(counts as usize);
        for plumb_line in plumb_lines {
            let (x, y) = plumb_line.split_at(4);

            let x = f32::from_le_bytes(x.try_into().map_err(|_| SecError::FromSliceErr)?);
            let y = f32::from_le_bytes(y.try_into().map_err(|_| SecError::FromSliceErr)?);

            collection.push(PlumbLine { x, y });
        }

        Ok((ret, Self { collection }))
    }
}
