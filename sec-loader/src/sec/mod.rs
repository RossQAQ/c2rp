pub mod body;
pub mod header;
pub mod tail;

use nom::IResult;
use std::path::PathBuf;

use body::SecBody;
use header::SecHeader;
use tail::SecTail;

use crate::err::SecError;

/// Raw SEC file data.
///
/// Just the path and SEC bytes.
struct SecRaw {
    path: PathBuf,

    data: Vec<u8>,
}

impl SecRaw {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let data = std::fs::read(&path).map_err(SecError::OpenFile)?;

        Ok(Self { path, data })
    }

    pub fn parse(&mut self) -> anyhow::Result<Sec> {
        let (next, header) = self.parse_header()?;

        let (next, body) = Self::parse_body(next, header.clone())?;

        let (_, tail) = Self::parse_tail(next)?;

        Ok(Sec {
            path: self.path.clone(),
            header,
            main: body,
            tail,
        })
    }

    fn parse_header(&mut self) -> IResult<&[u8], SecHeader, SecError> {
        SecHeader::from_raw(&self.data)
    }

    fn parse_body(data: &[u8], header: SecHeader) -> IResult<&[u8], SecBody, SecError> {
        SecBody::from_raw(data, header)
    }

    fn parse_tail(data: &[u8]) -> IResult<&[u8], SecTail, SecError> {
        SecTail::from_raw(data)
    }
}

/// SEC file structure
///
/// please check cmdt docs
///
/// https://github.com/IanusInferus/cmdt
#[derive(Debug, Clone)]
pub struct Sec {
    /// sec file path
    pub path: PathBuf,

    /// sec header
    pub header: SecHeader,

    /// sec main body
    pub main: SecBody,

    /// sec tail
    pub tail: SecTail,
}

impl Sec {
    /// Read SEC file from path and then try parse it.
    pub fn from_file(path: PathBuf) -> anyhow::Result<Self> {
        let mut raw = SecRaw::new(path)?;

        let sec = raw.parse()?;

        Ok(sec)
    }

    /// Write SEC file to the original path.
    pub fn write_back_raw(&self) -> Result<(), SecError> {
        todo!()
    }

    /// Write SEC file to the new path.
    pub fn write_back_raw_to(&self, path: PathBuf) -> Result<(), SecError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tu01() {
        Sec::from_file(PathBuf::from("../test_sec/TU01EX.SEC")).unwrap();
    }

    #[test]
    fn test_parse_tu02() {
        Sec::from_file(PathBuf::from("../test_sec/TU02EX.SEC")).unwrap();
    }

    #[test]
    fn test_parse_bu() {
        Sec::from_file(PathBuf::from("../test_sec/BUEX.SEC")).unwrap();
    }

    #[test]
    fn test_parse_is() {
        Sec::from_file(PathBuf::from("../test_sec/ISEX.SEC")).unwrap();
    }
}
