use std::fmt;
use std::error;

pub type BoxErr = Box<dyn std::error::Error>;

#[derive(Debug)]
pub struct Error {
    message: &'static str,
}
impl Error {
    pub fn new(message: &'static str) -> Self {
        Error { message }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {}

#[macro_export]
macro_rules! err {
    ( $lit:literal ) => {
        Err(Box::new(Error::new($lit)))
    };
}
