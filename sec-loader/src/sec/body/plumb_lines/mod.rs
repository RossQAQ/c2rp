use nom::{IResult, Parser, bytes::complete::take, multi::count};

use crate::err::SecError;

#[derive(Debug, Clone)]
pub struct PlumbLine {
    pub x: f32,

    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct PlumbLines {
    collection: Vec<PlumbLine>,
}

impl PlumbLines {
    pub(crate) fn from_raw(raw: &[u8], counts: i32) -> IResult<&[u8], Self, SecError> {
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

    pub fn get_plumb_line(&self, idx: usize) -> Option<&PlumbLine> {
        self.collection.get(idx)
    }

    pub fn get(&self, idx: usize) -> Option<&PlumbLine> {
        self.collection.get(idx)
    }
}

impl IntoIterator for PlumbLines {
    type Item = PlumbLine;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
}

impl<'a> IntoIterator for &'a PlumbLines {
    type Item = &'a PlumbLine;

    type IntoIter = std::slice::Iter<'a, PlumbLine>;

    fn into_iter(self) -> Self::IntoIter {
        self.collection.iter()
    }
}
