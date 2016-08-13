use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
/// An error that occurred during reading or writing of CONLL-X data.
pub enum Error {
    /// An IO error.
    Io(io::Error),

    /// Error parsing numerical columns: token identifier, head or
    /// projective head.
    Parse(num::ParseIntError),

    #[doc(hidden)]
    __Nonexhaustive,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::Parse(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Parse(ref err) => write!(f, "{}", err),
            Error::__Nonexhaustive => unreachable!(),
        }
    }
}
