use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct NoneError;

impl Display for NoneError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "NoneError")
    }
}

impl Error for NoneError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}