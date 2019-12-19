use std::fmt;
use std::io::Error as IoError;
use std::result;

use failure::{Backtrace, Context, Fail};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "I/O error: {}", reason)]
    Io { reason: String },
    #[fail(
        display = "Passed an invalid UTF-8 value: {:?} at index {}",
        value, index
    )]
    Utf8 { value: Vec<u8>, index: usize },
    #[fail(display = "Command raised an error: {}", description)]
    InvalidCommand { description: String },
    #[fail(
        display = "Tests for the {} crate are failing. Output: \n{:#?}",
        crate_name, output
    )]
    TestsFailure {
        crate_name: String,
        output: Vec<String>,
    },
    #[fail(display = "{}", description)]
    Other { description: String },
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::from(ErrorKind::Io {
            reason: format!("{}", err),
        })
    }
}
