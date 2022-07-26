use std::fmt;
use std::error;

#[derive(Debug)]
pub struct Error {
    message: &'static str,
}
impl Error {
    pub fn new(message: &'static str) -> Self {
        Error { message }
    }
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    //fn source(&self) -> Option<&(dyn Error + 'static)> {
    //}
}
