use std::io;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use zip::result::ZipError;

use crate::xmlutils::XMLError;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    Utf8Error(Utf8Error),
    XMLError(XMLError),
    ZipError(ZipError),
    CurrentChapterNotFound,
    CurrentChapterIsBroken,
    CoverNotFound,
    NoMorePages,
    PageNotValid,
    PathNotFound,
    ResourceIdNotFound,
    TOCNotFound,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;
        match self {
            XMLError(err) => write!(f, "{}", err),
            Utf8Error(err) => write!(f, "{}", err),
            ZipError(err) => write!(f, "{}", err),
            IoError(err) => write!(f, "{}", err),
            CurrentChapterNotFound => write!(f, "Current chapter not found"),
            CurrentChapterIsBroken => write!(f, "Current chapter is broken"),
            CoverNotFound => write!(f, "Cover not found"),
            NoMorePages => write!(f, "No more pages"),
            PageNotValid => write!(f, "Page number not valid"),
            PathNotFound => write!(f, "Path not found"),
            ResourceIdNotFound => write!(f, "Resource ID not found"),
            TOCNotFound => write!(f, "TOC not found"),
        }
    }
}

impl std::convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl std::convert::From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}

impl std::convert::From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::Utf8Error(err.utf8_error())
    }
}

impl std::convert::From<XMLError> for Error {
    fn from(err: XMLError) -> Self {
        Error::XMLError(err)
    }
}

impl std::convert::From<ZipError> for Error {
    fn from(err: ZipError) -> Self {
        Error::ZipError(err)
    }
}
