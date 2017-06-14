use std;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum NetparseError {
    Io(std::io::Error),
    Netparse(String),
    Unimplemented(String),
}

impl NetparseError {
    pub fn unimplemented(element: &str) -> NetparseError {
        NetparseError::Unimplemented(element.to_owned())
    }
}

impl Display for NetparseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            NetparseError::Io(ref err) => write!(f, "IoError: {}", err),
            NetparseError::Netparse(ref err) => write!(f, "NetparseError: {}", err),
            NetparseError::Unimplemented(ref err) => write!(f, "UnimplementedError: {}", err),
        }
    }
}

impl From<std::io::Error> for NetparseError {
    fn from(err: std::io::Error) -> NetparseError {
        NetparseError::Io(err)
    }
}

impl<'a> From<&'a str> for NetparseError {
    fn from(err: &'a str) -> NetparseError {
        NetparseError::Netparse(String::from(err))
    }
}

impl From<String> for NetparseError {
    fn from(err: String) -> NetparseError {
        NetparseError::Netparse(err)
    }
}
