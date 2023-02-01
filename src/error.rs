use regex::Error as RegexError;
use std::{array::TryFromSliceError, io, num::ParseIntError, string::FromUtf8Error};
use std::fmt::{Display, Formatter, Write};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TryFromSlice(TryFromSliceError),
    FromUtf8(FromUtf8Error),
    Regex(RegexError),
    IO(std::io::Error),
    ParseInt(ParseIntError),
    Encode(String),
    Decode(String),
    Path(String),
    Unknown,
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Error::FromUtf8(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseInt(value)
    }
}

impl From<TryFromSliceError> for Error {
    fn from(value: TryFromSliceError) -> Self {
        Error::TryFromSlice(value)
    }
}

impl From<RegexError> for Error {
    fn from(value: RegexError) -> Self {
        Error::Regex(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IO(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        match self {
            Error::TryFromSlice(e) => msg = e.to_string(),
            Error::FromUtf8(e) => msg = e.to_string(),
            Error::Regex(e) => msg = e.to_string(),
            Error::ParseInt(e) => msg = e.to_string(),
            Error::IO(e) => msg = e.to_string(),
            Error::Encode(e) => msg = e.to_string(),
            Error::Decode(e) => msg = e.to_string(),
            Error::Path(e) => msg = e.to_string(),
            Error::Unknown => msg = "unknown".to_string(),
        }
        f.write_str(msg.as_str())
    }
}