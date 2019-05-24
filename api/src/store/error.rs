use std::{error, fmt};

#[derive(Clone, Debug)]
pub struct StoreError(String);

impl StoreError {
    pub fn new(msg: String) -> StoreError {
        StoreError(msg)
    }
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for StoreError {
    fn description(&self) -> &str {
        "Store error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
