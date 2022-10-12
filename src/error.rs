use ron::error::{Error as RonError, SpannedError};
use std::{io::Error as IoError, process::ExitStatus};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Ron(RonError),
    RonSpan(SpannedError),
    Io(IoError),
    IndexError,
    BadString,
    ApplicationDirNotAccessible,
    ExtCmdFailed(ExitStatus),
}

impl From<RonError> for Error {
    fn from(e: RonError) -> Self {
        Error::Ron(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::Io(e)
    }
}

impl From<SpannedError> for Error {
    fn from(e: SpannedError) -> Self {
        Error::RonSpan(e)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
