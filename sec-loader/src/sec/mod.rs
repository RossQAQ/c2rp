pub mod body;
pub mod header;
pub mod tail;

use nom::IResult;
use std::path::PathBuf;

use body::SecMain;
use header::SecHeader;
use tail::SecTail;

use crate::err::SecError;

/// Raw SEC file data.
///
/// Just the path and SEC bytes.
struct SecRaw {
    data: Vec<u8>,
}

impl SecRaw {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn parse(&mut self) -> anyhow::Result<Sec> {
        let (next, header) = self.parse_header()?;
        todo!()
    }

    pub fn parse_header(&mut self) -> IResult<&[u8], SecHeader, SecError> {
        SecHeader::from_raw(&self.data)
    }
}

/// SEC file structure
///
/// please check cmdt docs
///
/// https://github.com/IanusInferus/cmdt
pub struct Sec {
    /// sec file path
    pub path: PathBuf,

    /// sec header
    pub header: SecHeader,

    /// sec main body
    pub main: SecMain,

    /// sec tail
    pub tail: SecTail,
}

impl Sec {
    /// Read SEC file from path and then try parse it.
    pub fn from_file(path: PathBuf) -> anyhow::Result<Self> {
        let data = std::fs::read(path).map_err(SecError::OpenFile)?;
        let mut raw = SecRaw::new(data);
        raw.parse()?;
        todo!()
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
