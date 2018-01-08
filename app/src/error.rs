use std::fmt::{Display, Formatter};
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    IO(::std::io::Error),
    UTF8(::std::string::FromUtf8Error),
    Custom(String),
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<::std::string::FromUtf8Error> for Error {
    fn from(err: ::std::string::FromUtf8Error) -> Self {
        Error::UTF8(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Custom(err)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        return match self {
            &Error::IO(ref err) => err.description(),
            &Error::UTF8(ref err) => err.description(),
            &Error::Custom(ref err) => err.as_str(),
        };
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", error::Error::description(self))
    }
}
