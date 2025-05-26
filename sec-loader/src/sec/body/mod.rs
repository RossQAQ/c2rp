use field_a::FieldACollection;
use field_b::FieldBCollection;
use field_c::FieldCCollection;
use nom::IResult;

use crate::err::SecError;

use super::header::SecHeader;

mod field_a;
mod field_b;
mod field_c;

#[derive(Debug, Clone)]
pub struct SecBody {
    pub field_a: FieldACollection,
    pub field_b: FieldBCollection,
    pub field_c: FieldCCollection,
}

impl SecBody {
    pub fn from_raw(raw: &[u8], header: SecHeader) -> IResult<&[u8], Self, SecError> {
        let (next, field_a) = FieldACollection::from_raw(raw, header.plumb_lines())?;
        let (next, field_b) = FieldBCollection::from_raw(next, header.border_num())?;
        let (ret, field_c) = FieldCCollection::from_raw(next, header.district_num())?;

        Ok((
            ret,
            SecBody {
                field_a,
                field_b,
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
