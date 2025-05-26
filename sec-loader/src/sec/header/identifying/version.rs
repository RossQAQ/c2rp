use crate::err::SecError;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use nom::IResult;
use nom::bytes::complete::take;

/// Identify the version of the SEC file
#[repr(i64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
pub enum SecVersion {
    /// commandos 2
    C2 = 0x100000001,
    /// todo
    C3 = 0x200000001,
}

#[derive(Debug, Clone)]
pub(crate) struct VersionSign {
    /// version
    version: SecVersion,
}

impl VersionSign {
    pub fn parse(raw: &[u8]) -> IResult<&[u8], Self, SecError> {
        let (ret, bytes) = take(8usize)(raw)?;

        let version = i64::from_le_bytes(bytes.try_into().map_err(|_| SecError::FromSliceErr)?);

        Ok((ret, Self::new(version)?))
    }

    pub fn version(&self) -> SecVersion {
        self.version
    }

    fn new(version: i64) -> Result<Self, SecError> {
        let version =
            SecVersion::try_from(version).map_err(|_| SecError::UnknownVersionSign(version))?;
        Ok(Self { version })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_version() {
        let version_sign = VersionSign::new(0x100000001).unwrap();
        assert_eq!(version_sign.version(), SecVersion::C2);

        let version_sign = VersionSign::new(4_294_967_297).unwrap();
        assert_eq!(version_sign.version(), SecVersion::C2);

        let version_sign = VersionSign::new(0x200000001).unwrap();
        assert_eq!(version_sign.version(), SecVersion::C3);
    }

    #[test]
    fn test_invalid_version() {
        let version_sign = VersionSign::new(0x100000002);
        assert!(version_sign.is_err());
    }
}
