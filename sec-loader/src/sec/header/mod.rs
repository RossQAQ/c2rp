use basic_info::BasicInfo;
use identifying::Identify;
use nom::IResult;

use crate::err::SecError;

mod basic_info;
mod identifying;

/// SEC header part.
#[derive(Debug, Clone)]
pub struct SecHeader {
    /// identifying part
    identifying: Identify,

    /// basic info part
    basic_info: BasicInfo,
}

impl SecHeader {
    pub fn from_raw(raw: &[u8]) -> IResult<&[u8], Self, SecError> {
        let (next, identifying) = Identify::from_raw(raw)?;
        let (ret, basic_info) = BasicInfo::from_raw(next)?;

        Ok((
            ret,
            Self {
                identifying,
                basic_info,
            },
        ))
    }

    /// number of plumb lines
    pub fn plumb_lines(&self) -> i32 {
        self.basic_info.plumb_line_num
    }

    pub fn border_num(&self) -> i32 {
        self.basic_info.side_face_num
    }

    pub fn district_num(&self) -> i32 {
        self.basic_info.polygon_num
    }

    pub fn special(&self) -> i32 {
        self.basic_info.throughable_side_num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sec_header() {
        let data = std::fs::read("../test_sec/TU01EX.SEC").unwrap();
        let (_, _header) = SecHeader::from_raw(&data).unwrap();
    }
}
