use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ChainError {
    details: String,
}

impl ChainError {
    pub fn new(msg: &str) -> ChainError {
        ChainError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ChainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ChainError {
    fn description(&self) -> &str {
        &self.details
    }
}
