use nom::error::{ErrorKind, ParseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecError {
    #[error("failed to open file: {0}")]
    OpenFile(std::io::Error),

    #[error("unknown version sign: {0}")]
    UnknownVersionSign(i64),

    #[error("parse type err: {0}")]
    ParseTypeErr(String),

    #[error("{0}")]
    NomParseErr(String),

    #[error("parsing from slice failed")]
    FromSliceErr,
}

impl<I> ParseError<I> for SecError {
    fn from_error_kind(_input: I, kind: ErrorKind) -> Self {
        SecError::NomParseErr(format!("Parse error: {:?}", kind))
    }

    fn append(_input: I, kind: ErrorKind, other: Self) -> Self {
        SecError::NomParseErr(format!("{:?}: {}", kind, other))
    }
}

impl<I> From<nom::error::Error<I>> for SecError {
    fn from(err: nom::error::Error<I>) -> Self {
        SecError::NomParseErr(format!("Nom error: {:?}", err.code))
    }
}

impl<I> From<(I, ErrorKind)> for SecError {
    fn from((_, kind): (I, ErrorKind)) -> Self {
        SecError::NomParseErr(format!("Parse error: {:?}", kind))
    }
}

impl From<SecError> for nom::Err<SecError> {
    fn from(err: SecError) -> Self {
        nom::Err::Failure(err)
    }
}
