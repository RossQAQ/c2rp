use nom::IResult;
use plumb_lines::PlumbLines;
use polygons::Polygons;
use sides::Sides;

use crate::err::SecError;

use super::header::SecHeader;

mod plumb_lines;
mod polygons;
mod sides;

/// SEC body part.
#[derive(Debug, Clone)]
pub struct SecBody {
    pub plumb_lines: PlumbLines,
    pub sides: Sides,
    pub field_c: Polygons,
}

impl SecBody {
    pub fn from_raw(raw: &[u8], header: SecHeader) -> IResult<&[u8], Self, SecError> {
        let (next, plumb_lines) = PlumbLines::from_raw(raw, header.plumb_lines())?;
        let (next, sides) = Sides::from_raw(next, header.border_num())?;
        let (ret, field_c) = Polygons::from_raw(next, header.district_num())?;

        Ok((
            ret,
            SecBody {
                plumb_lines,
                sides,
                field_c,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sec_body() {
        let data = std::fs::read("../test_sec/TU01EX.SEC").unwrap();
        let (next, header) = SecHeader::from_raw(&data).unwrap();
        let (_, _body) = SecBody::from_raw(&next, header).unwrap();
    }
}
